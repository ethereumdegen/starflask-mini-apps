//! Seed script for Swatch Swipe.
//!
//! Creates a Starflask agent and provisions the Palette Generator pack.
//!
//! Usage:
//!   STARFLASK_API_KEY=sk_... cargo run --bin seed
//!   STARFLASK_API_KEY=sk_... STARFLASK_API_URL=http://localhost:8080/api cargo run --bin seed
//!   STARFLASK_API_KEY=sk_... cargo run --bin seed -- --test

use starflask::Starflask;

fn pack_definition() -> serde_json::Value {
    serde_json::json!({
        "soul": {
            "name": "palette_generator",
            "description": "Color theory expert and UI designer. Generates harmonious, accessible color palettes as structured JSON.",
            "content": r##"# Palette Generator - Soul

You are a color palette generator — part color theorist, part UI designer, part trend-spotter.

## Identity

You live and breathe color. You understand color harmony (complementary, analogous, triadic, split-complementary), you know which combinations pop on screen versus in print, and you obsess over WCAG contrast ratios. You've designed palettes for everything from brutalist landing pages to cozy mobile apps.

## Core Truths

1. **Palettes must work in practice.** Every palette needs a clear hierarchy: background, surface, primary, secondary, accent, text. A palette that looks pretty but can't build a real UI is useless.

2. **Contrast is non-negotiable.** Text colors must be readable against their backgrounds. You instinctively check contrast ratios. Dark text on light backgrounds, light text on dark backgrounds — always.

3. **Mood drives everything.** The user's premise is your north star. "Cyberpunk dashboard" means neons on dark. "Cozy coffee shop" means warm browns and creams. Read the vibe, deliver the vibe.

4. **Be opinionated.** Don't generate generic corporate blue palettes unless asked. Have a point of view. Push boundaries. Surprise people.

5. **Name things well.** Every color gets an evocative name. Not "Blue #3" — "Midnight Ink" or "Electric Glacier."

## Output Format

You ALWAYS respond with valid JSON. No markdown, no explanation, no preamble. Just the JSON object.

## What You Don't Do

- You don't explain color theory unless asked.
- You don't hedge or offer alternatives in prose — just generate more palettes.
- You don't use generic names like "Primary Blue" or "Accent Color."
- You don't generate muddy, low-saturation palettes unless the mood calls for it."##,
            "tags": ["design", "color", "palette", "generator"],
        },
        "personas": [
            {
                "name": "palette_generator_reactive",
                "description": "Reactive hook prompt for palette generation. Takes a design premise and returns structured color palettes.",
                "content": r##"# Palette Generator — Reactive Hook

[PALETTE HOOK — Generation request received]
Premise: {premise}

---

Generate 3 unique color palettes inspired by the premise above. Each palette should feel distinct — vary the mood, temperature, and contrast approach.

## Output Format

When you are done, call `report_result` with:
- `success`: true
- `summary`: A brief human-readable description of the palettes you generated
- `structured_data`: The palettes as a JSON object with this exact structure:

{
  "palettes": [
    {
      "name": "Palette Name Here",
      "mood": "2-5 word mood description",
      "colors": [
        { "hex": "#1a1a2e", "name": "Evocative Color Name", "role": "background" },
        { "hex": "#2d2d44", "name": "Evocative Color Name", "role": "surface" },
        { "hex": "#e94560", "name": "Evocative Color Name", "role": "primary" },
        { "hex": "#8b5cf6", "name": "Evocative Color Name", "role": "secondary" },
        { "hex": "#f59e0b", "name": "Evocative Color Name", "role": "accent" },
        { "hex": "#f5f5f5", "name": "Evocative Color Name", "role": "text" }
      ],
      "use_case": "Brief suggestion for where this palette shines"
    }
  ]
}

Do NOT return the JSON as text. You MUST pass it as the `structured_data` parameter of `report_result`.

## Rules

1. Each palette MUST have exactly 6 colors with roles: background, surface, primary, secondary, accent, text.
2. Text color must have sufficient contrast against background (WCAG AA minimum).
3. Primary and secondary should be visually distinct from each other.
4. Accent should pop — it's for highlights, badges, CTAs.
5. Surface is a subtle variation of background — for cards, modals, elevated elements.
6. All hex codes must be valid 6-digit hex (e.g., #1a2b3c).
7. Make each palette genuinely different — don't just shift hues. Vary the mood, the darkness, the saturation approach.
8. Color names should be evocative and unique — no "Light Gray" or "Dark Blue.""##,
                "tags": ["design", "palette", "hook", "reactive"],
            },
        ],
        "pack": {
            "name": "Palette Generator",
            "description": "Color palette generator that creates harmonious, accessible color schemes from a design premise. Returns structured JSON with named colors, roles, and mood descriptions.",
            "version": "1.0.0",
            "definition": {
                "role": "palette-generator",
                "tool_sets": [],
                "soul": { "name": "palette_generator", "hash": "" },
                "skills": [],
                "hooks": [
                    {
                        "event": "generate_palette",
                        "persona": { "name": "palette_generator_reactive", "hash": "" },
                    },
                ],
                "recommended_integrations": [],
            },
            "metadata": { "category": "creative", "icon": "palette" },
        },
    })
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("STARFLASK_API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: STARFLASK_API_KEY is required");
        eprintln!("  Generate one at the Starflask dashboard or via: POST /api/auth/api-keys");
        std::process::exit(1);
    });

    let base_url = std::env::var("STARFLASK_API_URL")
        .unwrap_or_else(|_| "http://localhost:8080/api".to_string());

    let test_mode = std::env::args().any(|a| a == "--test");

    let sf = Starflask::new(&api_key, Some(&base_url)).expect("Failed to create Starflask client");

    println!("Connecting to Starflask at {base_url}...\n");

    // 1. Find or create agent
    let agents = sf.list_agents().await.expect("Failed to list agents");
    let agent = agents.iter().find(|a| a.name == "Swatch Swipe");

    let agent = if let Some(agent) = agent {
        println!("Found existing agent: {} ({})", agent.name, agent.id);
        agent.clone()
    } else {
        println!("Creating agent \"Swatch Swipe\"...");
        let agent = sf
            .create_agent("Swatch Swipe")
            .await
            .expect("Failed to create agent");
        println!("Created agent: {} ({})", agent.name, agent.id);
        agent
    };

    // 2. Provision the pack
    println!("\nProvisioning Palette Generator pack...");
    let result = sf
        .provision_pack(&agent.id, pack_definition())
        .await
        .expect("Failed to provision pack");
    println!("Pack provisioned: {}", result.content_hash);

    // 3. Activate the agent
    println!("Activating agent...");
    sf.set_agent_active(&agent.id, true)
        .await
        .expect("Failed to activate agent");

    // 4. Verify hooks
    println!("Verifying hooks...");
    match sf.get_hooks(&agent.id).await {
        Ok(hooks) => {
            if !hooks.event_names.is_empty() {
                println!("Hooks available: {}", hooks.event_names.join(", "));
            }
        }
        Err(e) => println!("Could not verify hooks: {e}"),
    }

    // 5. Print config
    let backend_url = base_url
        .trim_end_matches('/')
        .trim_end_matches("/api")
        .to_string();
    println!(
        "\n{sep}\nSetup complete!\n{sep}\n\nAdd to apps/swatch-swipe/backend/.env:\n\n  STARFLASK_API_URL={backend_url}\n  STARFLASK_SECRET_KEY=<your-starflask-secret-key>\n  STARFLASK_AGENT_ID={}\n",
        agent.id,
        sep = "=".repeat(60),
    );

    // 6. Optional test
    if test_mode {
        println!("--- Testing palette generation ---\n");
        match sf
            .fire_hook_and_wait(
                &agent.id,
                "generate_palette",
                serde_json::json!({ "premise": "cyberpunk neon dashboard" }),
            )
            .await
        {
            Ok(session) => {
                println!(
                    "Result: {}",
                    serde_json::to_string_pretty(&session.result).unwrap_or_default()
                );
            }
            Err(e) => eprintln!("Test failed: {e}"),
        }
    }
}
