#!/usr/bin/env node

/**
 * Seed script for Swatch Swipe.
 *
 * Creates a Starflask agent and provisions the Palette Generator pack —
 * everything in one shot, no Axoniac CLI needed.
 *
 * Usage:
 *   STARFLASK_API_KEY=sk_... node seed.js
 *   STARFLASK_API_KEY=sk_... STARFLASK_API_URL=http://localhost:8080/api node seed.js
 *   STARFLASK_API_KEY=sk_... node seed.js --test
 */

import { Starflask } from "../../packages/starflask/index.js";

// ── Agent Pack Definition (inline) ──────────────────────────

const SOUL = {
  name: "palette_generator",
  description:
    "Color theory expert and UI designer. Generates harmonious, accessible color palettes as structured JSON.",
  content: `# Palette Generator - Soul

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
- You don't generate muddy, low-saturation palettes unless the mood calls for it.`,
  tags: ["design", "color", "palette", "generator"],
};

const PERSONAS = [
  {
    name: "palette_generator_reactive",
    description:
      "Reactive hook prompt for palette generation. Takes a design premise and returns structured color palettes.",
    content: `# Palette Generator — Reactive Hook

[PALETTE HOOK — Generation request received]
Premise: {premise}

---

Generate 3 unique color palettes inspired by the premise above. Each palette should feel distinct — vary the mood, temperature, and contrast approach.

## Output Format

Respond with ONLY this JSON structure, no other text:

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

## Rules

1. Each palette MUST have exactly 6 colors with roles: background, surface, primary, secondary, accent, text.
2. Text color must have sufficient contrast against background (WCAG AA minimum).
3. Primary and secondary should be visually distinct from each other.
4. Accent should pop — it's for highlights, badges, CTAs.
5. Surface is a subtle variation of background — for cards, modals, elevated elements.
6. All hex codes must be valid 6-digit hex (e.g., #1a2b3c).
7. Make each palette genuinely different — don't just shift hues. Vary the mood, the darkness, the saturation approach.
8. Color names should be evocative and unique — no "Light Gray" or "Dark Blue."`,
    tags: ["design", "palette", "hook", "reactive"],
  },
];

const PACK = {
  name: "Palette Generator",
  description:
    "Color palette generator that creates harmonious, accessible color schemes from a design premise. Returns structured JSON with named colors, roles, and mood descriptions.",
  version: "1.0.0",
  definition: {
    role: "palette-generator",
    tool_sets: [],
    soul: { name: "palette_generator", hash: "" },
    skills: [],
    hooks: [
      {
        event: "generate_palette",
        persona: { name: "palette_generator_reactive", hash: "" },
      },
    ],
    recommended_integrations: [],
  },
  metadata: { category: "creative", icon: "palette" },
};

// ── Main ────────────────────────────────────────────────────

const apiKey = process.env.STARFLASK_API_KEY;
const baseUrl = process.env.STARFLASK_API_URL || "http://localhost:8080/api";

if (!apiKey) {
  console.error("Error: STARFLASK_API_KEY is required");
  console.error(
    "  Generate one at the Starflask dashboard or via: POST /api/auth/api-keys"
  );
  process.exit(1);
}

const sf = new Starflask({ apiKey, baseUrl });

async function main() {
  console.log(`Connecting to Starflask at ${baseUrl}...\n`);

  // 1. Find or create agent
  const agents = await sf.listAgents();
  let agent = agents.find((a) => a.name === "Swatch Swipe");

  if (agent) {
    console.log(`Found existing agent: ${agent.name} (${agent.id})`);
  } else {
    console.log('Creating agent "Swatch Swipe"...');
    agent = await sf.createAgent({ name: "Swatch Swipe" });
    console.log(`Created agent: ${agent.name} (${agent.id})`);
  }

  // 2. Provision the pack (creates soul + persona + pack in Axoniac, installs on agent)
  console.log("\nProvisioning Palette Generator pack...");
  const result = await sf.provisionPack(agent.id, {
    soul: SOUL,
    personas: PERSONAS,
    pack: PACK,
  });
  console.log(`Pack provisioned: ${result.content_hash}`);

  // 3. Activate the agent
  console.log("Activating agent...");
  await sf.setAgentActive(agent.id, true);

  // 4. Verify hooks
  console.log("Verifying hooks...");
  try {
    const hooks = await sf.getHooks(agent.id);
    if (hooks.hooks?.length > 0) {
      console.log(
        `Hooks available: ${hooks.hooks.map((h) => h.event).join(", ")}`
      );
    }
  } catch (e) {
    console.log("Could not verify hooks:", e.message);
  }

  // 5. Print config
  const backendUrl = baseUrl.replace(/\/api\/?$/, "");
  console.log(`
${"=".repeat(60)}
Setup complete!
${"=".repeat(60)}

Add to apps/swatch-swipe/backend/.env:

  STARFLASK_API_URL=${backendUrl}
  STARFLASK_SECRET_KEY=<your-starflask-secret-key>
  STARFLASK_AGENT_ID=${agent.id}
`);

  // 6. Optional test
  if (process.argv.includes("--test")) {
    console.log("--- Testing palette generation ---\n");
    try {
      const session = await sf.fireHookAndWait(
        agent.id,
        "generate_palette",
        { premise: "cyberpunk neon dashboard" },
        { timeoutMs: 60_000 }
      );
      console.log("Result:", JSON.stringify(session.result, null, 2));
    } catch (e) {
      console.error("Test failed:", e.message);
    }
  }
}

main().catch((e) => {
  console.error("Fatal error:", e.message);
  process.exit(1);
});
