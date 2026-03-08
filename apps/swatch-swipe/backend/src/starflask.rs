use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::Value;
use sha2::Sha256;
use std::time::Duration;
use tracing::{info, warn};

use crate::models::Palette;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct StarflaskClient {
    pub api_url: String,
    pub secret_key: String,
    pub agent_id: String,
    client: Client,
}

impl StarflaskClient {
    pub fn new(api_url: String, secret_key: String, agent_id: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(90))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_url,
            secret_key,
            agent_id,
            client,
        }
    }

    fn compute_auth_token(&self) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(b"starflask-worker-auth");
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    pub async fn fire_event(&self, premise: &str) -> Result<String, String> {
        let token = self.compute_auth_token();
        let url = format!("{}/api/worker/fire_event", self.api_url);

        let payload = serde_json::json!({
            "agent_id": self.agent_id,
            "event": "generate_palette",
            "payload": {
                "premise": premise
            }
        });

        info!("Firing event to Starflask: {}", url);

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to fire event: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!(
                "Fire event failed with status {}: {}",
                status, body
            ));
        }

        let body: Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse fire_event response: {}", e))?;

        let session_id = body["id"]
            .as_str()
            .ok_or_else(|| "No session id in fire_event response".to_string())?
            .to_string();

        info!("Got session_id: {}", session_id);
        Ok(session_id)
    }

    pub async fn poll_session(&self, session_id: &str) -> Result<Vec<Palette>, String> {
        let token = self.compute_auth_token();
        let url = format!(
            "{}/api/worker/sessions/{}",
            self.api_url, session_id
        );

        let max_attempts = 30;
        let poll_interval = Duration::from_secs(2);

        for attempt in 1..=max_attempts {
            info!("Polling session {} (attempt {}/{})", session_id, attempt, max_attempts);

            let resp = self
                .client
                .get(&url)
                .bearer_auth(&token)
                .send()
                .await
                .map_err(|e| format!("Failed to poll session: {}", e))?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                warn!("Poll attempt {} failed: {} - {}", attempt, status, body);
                tokio::time::sleep(poll_interval).await;
                continue;
            }

            let body: Value = resp
                .json()
                .await
                .map_err(|e| format!("Failed to parse session response: {}", e))?;

            let status = body["status"].as_str().unwrap_or("unknown");

            match status {
                "completed" => {
                    info!("Session completed, extracting palettes");
                    return extract_palettes(&body);
                }
                "failed" => {
                    let error = body["error"].as_str().unwrap_or("Unknown error");
                    return Err(format!("Session failed: {}", error));
                }
                _ => {
                    info!("Session status: {}, continuing to poll...", status);
                    tokio::time::sleep(poll_interval).await;
                }
            }
        }

        Err("Timed out waiting for session to complete".to_string())
    }

    pub async fn generate_palettes(&self, premise: &str) -> Result<Vec<Palette>, String> {
        let session_id = self.fire_event(premise).await?;
        self.poll_session(&session_id).await
    }
}

fn extract_palettes(session: &Value) -> Result<Vec<Palette>, String> {
    // Try to find palettes in the session result/output
    // The worker response might be nested in different ways
    let result = session
        .get("result")
        .or_else(|| session.get("output"))
        .or_else(|| session.get("response"));

    if let Some(result) = result {
        // Try parsing as string first (might be JSON string)
        if let Some(s) = result.as_str() {
            if let Ok(palettes) = serde_json::from_str::<Vec<Palette>>(s) {
                return Ok(palettes);
            }
            // Try extracting JSON from markdown code blocks
            if let Some(json_str) = extract_json_from_text(s) {
                if let Ok(palettes) = serde_json::from_str::<Vec<Palette>>(&json_str) {
                    return Ok(palettes);
                }
                // Maybe it's a wrapper object with a palettes field
                if let Ok(val) = serde_json::from_str::<Value>(&json_str) {
                    if let Some(palettes_val) = val.get("palettes") {
                        if let Ok(palettes) =
                            serde_json::from_value::<Vec<Palette>>(palettes_val.clone())
                        {
                            return Ok(palettes);
                        }
                    }
                }
            }
        }
        // Try parsing directly as array
        if let Ok(palettes) = serde_json::from_value::<Vec<Palette>>(result.clone()) {
            return Ok(palettes);
        }
        // Try as object with palettes field
        if let Some(palettes_val) = result.get("palettes") {
            if let Ok(palettes) = serde_json::from_value::<Vec<Palette>>(palettes_val.clone()) {
                return Ok(palettes);
            }
        }
    }

    Err("Could not extract palettes from session result".to_string())
}

fn extract_json_from_text(text: &str) -> Option<String> {
    // Look for JSON in ```json ... ``` blocks
    if let Some(start) = text.find("```json") {
        let after_marker = &text[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return Some(after_marker[..end].trim().to_string());
        }
    }
    // Look for JSON in ``` ... ``` blocks
    if let Some(start) = text.find("```") {
        let after_marker = &text[start + 3..];
        if let Some(end) = after_marker.find("```") {
            let content = after_marker[..end].trim();
            if content.starts_with('[') || content.starts_with('{') {
                return Some(content.to_string());
            }
        }
    }
    // Look for raw JSON array or object
    if let Some(start) = text.find('[') {
        if let Some(end) = text.rfind(']') {
            return Some(text[start..=end].to_string());
        }
    }
    None
}

pub fn mock_palettes(premise: &str) -> Vec<Palette> {
    use crate::models::PaletteColor;

    vec![
        Palette {
            name: "Sunset Warmth".to_string(),
            mood: "Warm and inviting".to_string(),
            colors: vec![
                PaletteColor {
                    hex: "#FF6B35".to_string(),
                    name: "Tangerine".to_string(),
                    role: "Primary".to_string(),
                },
                PaletteColor {
                    hex: "#F7C59F".to_string(),
                    name: "Peach".to_string(),
                    role: "Secondary".to_string(),
                },
                PaletteColor {
                    hex: "#EFEFD0".to_string(),
                    name: "Cream".to_string(),
                    role: "Background".to_string(),
                },
                PaletteColor {
                    hex: "#004E89".to_string(),
                    name: "Deep Blue".to_string(),
                    role: "Accent".to_string(),
                },
                PaletteColor {
                    hex: "#1A1A2E".to_string(),
                    name: "Midnight".to_string(),
                    role: "Text".to_string(),
                },
            ],
            use_case: format!("A warm palette inspired by: {}", premise),
        },
        Palette {
            name: "Ocean Depths".to_string(),
            mood: "Calm and mysterious".to_string(),
            colors: vec![
                PaletteColor {
                    hex: "#0B3D91".to_string(),
                    name: "Navy".to_string(),
                    role: "Primary".to_string(),
                },
                PaletteColor {
                    hex: "#1B98E0".to_string(),
                    name: "Azure".to_string(),
                    role: "Secondary".to_string(),
                },
                PaletteColor {
                    hex: "#E8F1F2".to_string(),
                    name: "Ice".to_string(),
                    role: "Background".to_string(),
                },
                PaletteColor {
                    hex: "#13293D".to_string(),
                    name: "Abyss".to_string(),
                    role: "Accent".to_string(),
                },
                PaletteColor {
                    hex: "#16DB93".to_string(),
                    name: "Seafoam".to_string(),
                    role: "Highlight".to_string(),
                },
            ],
            use_case: format!("A cool palette inspired by: {}", premise),
        },
        Palette {
            name: "Forest Canopy".to_string(),
            mood: "Natural and grounded".to_string(),
            colors: vec![
                PaletteColor {
                    hex: "#2D6A4F".to_string(),
                    name: "Emerald".to_string(),
                    role: "Primary".to_string(),
                },
                PaletteColor {
                    hex: "#95D5B2".to_string(),
                    name: "Sage".to_string(),
                    role: "Secondary".to_string(),
                },
                PaletteColor {
                    hex: "#F0EBD8".to_string(),
                    name: "Parchment".to_string(),
                    role: "Background".to_string(),
                },
                PaletteColor {
                    hex: "#8B5E3C".to_string(),
                    name: "Bark".to_string(),
                    role: "Accent".to_string(),
                },
                PaletteColor {
                    hex: "#1B4332".to_string(),
                    name: "Pine".to_string(),
                    role: "Text".to_string(),
                },
            ],
            use_case: format!("An earthy palette inspired by: {}", premise),
        },
    ]
}
