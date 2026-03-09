use starflask::Starflask;
use uuid::Uuid;

use crate::models::Palette;

#[derive(Clone)]
pub struct StarflaskClient {
    sf: Starflask,
    agent_id: Uuid,
}

impl StarflaskClient {
    pub fn new(api_url: &str, api_key: &str, agent_id: &str) -> Self {
        let base_url = format!("{}/api", api_url.trim_end_matches('/'));
        let sf = Starflask::new(api_key, Some(&base_url))
            .expect("Failed to create Starflask client");
        let agent_id: Uuid = agent_id.parse().expect("Invalid STARFLASK_AGENT_ID");

        Self { sf, agent_id }
    }

    pub async fn generate_palettes(&self, premise: &str) -> Result<Vec<Palette>, String> {
        let session = self
            .sf
            .fire_hook_and_wait(
                &self.agent_id,
                "generate_palette",
                serde_json::json!({ "premise": premise }),
            )
            .await
            .map_err(|e| e.to_string())?;

        extract_palettes(&session.result)
    }
}

fn extract_palettes(result: &Option<serde_json::Value>) -> Result<Vec<Palette>, String> {
    let result = result
        .as_ref()
        .ok_or_else(|| "No result in session".to_string())?;

    // Try as string (might be JSON string or markdown-wrapped)
    if let Some(s) = result.as_str() {
        if let Ok(palettes) = serde_json::from_str::<Vec<Palette>>(s) {
            return Ok(palettes);
        }
        if let Some(json_str) = extract_json_from_text(s) {
            if let Ok(palettes) = serde_json::from_str::<Vec<Palette>>(&json_str) {
                return Ok(palettes);
            }
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(p) = val.get("palettes") {
                    if let Ok(palettes) = serde_json::from_value::<Vec<Palette>>(p.clone()) {
                        return Ok(palettes);
                    }
                }
            }
        }
    }

    // Try directly as array
    if let Ok(palettes) = serde_json::from_value::<Vec<Palette>>(result.clone()) {
        return Ok(palettes);
    }

    // Try as object with palettes field
    if let Some(p) = result.get("palettes") {
        if let Ok(palettes) = serde_json::from_value::<Vec<Palette>>(p.clone()) {
            return Ok(palettes);
        }
    }

    // Try summary field (report_result wraps output here)
    if let Some(summary) = result.get("summary").and_then(|s| s.as_str()) {
        if let Some(json_str) = extract_json_from_text(summary) {
            if let Ok(palettes) = serde_json::from_str::<Vec<Palette>>(&json_str) {
                return Ok(palettes);
            }
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(p) = val.get("palettes") {
                    if let Ok(palettes) = serde_json::from_value::<Vec<Palette>>(p.clone()) {
                        return Ok(palettes);
                    }
                }
            }
        }
    }

    Err("Could not extract palettes from session result".to_string())
}

fn extract_json_from_text(text: &str) -> Option<String> {
    // ```json ... ```
    if let Some(start) = text.find("```json") {
        let after = &text[start + 7..];
        if let Some(end) = after.find("```") {
            return Some(after[..end].trim().to_string());
        }
    }
    // ``` ... ```
    if let Some(start) = text.find("```") {
        let after = &text[start + 3..];
        if let Some(end) = after.find("```") {
            let content = after[..end].trim();
            if content.starts_with('[') || content.starts_with('{') {
                return Some(content.to_string());
            }
        }
    }
    // Raw JSON
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
                PaletteColor { hex: "#FF6B35".into(), name: "Tangerine".into(), role: "Primary".into() },
                PaletteColor { hex: "#F7C59F".into(), name: "Peach".into(), role: "Secondary".into() },
                PaletteColor { hex: "#EFEFD0".into(), name: "Cream".into(), role: "Background".into() },
                PaletteColor { hex: "#004E89".into(), name: "Deep Blue".into(), role: "Accent".into() },
                PaletteColor { hex: "#1A1A2E".into(), name: "Midnight".into(), role: "Text".into() },
            ],
            use_case: format!("A warm palette inspired by: {}", premise),
        },
        Palette {
            name: "Ocean Depths".to_string(),
            mood: "Calm and mysterious".to_string(),
            colors: vec![
                PaletteColor { hex: "#0B3D91".into(), name: "Navy".into(), role: "Primary".into() },
                PaletteColor { hex: "#1B98E0".into(), name: "Azure".into(), role: "Secondary".into() },
                PaletteColor { hex: "#E8F1F2".into(), name: "Ice".into(), role: "Background".into() },
                PaletteColor { hex: "#13293D".into(), name: "Abyss".into(), role: "Accent".into() },
                PaletteColor { hex: "#16DB93".into(), name: "Seafoam".into(), role: "Highlight".into() },
            ],
            use_case: format!("A cool palette inspired by: {}", premise),
        },
        Palette {
            name: "Forest Canopy".to_string(),
            mood: "Natural and grounded".to_string(),
            colors: vec![
                PaletteColor { hex: "#2D6A4F".into(), name: "Emerald".into(), role: "Primary".into() },
                PaletteColor { hex: "#95D5B2".into(), name: "Sage".into(), role: "Secondary".into() },
                PaletteColor { hex: "#F0EBD8".into(), name: "Parchment".into(), role: "Background".into() },
                PaletteColor { hex: "#8B5E3C".into(), name: "Bark".into(), role: "Accent".into() },
                PaletteColor { hex: "#1B4332".into(), name: "Pine".into(), role: "Text".into() },
            ],
            use_case: format!("An earthy palette inspired by: {}", premise),
        },
    ]
}
