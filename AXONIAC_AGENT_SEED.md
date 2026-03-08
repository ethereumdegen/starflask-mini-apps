# Creating Agent Packs for Starflask Mini-Apps

This guide explains how to create an AI agent pack for a Starflask mini-app.

## Overview

Each mini-app talks to a **Starflask agent**, which is powered by an **Axoniac agent pack**. A pack is a bundle of:

- **Soul** — The agent's core identity/personality
- **Persona(s)** — Context-specific instructions triggered by hooks
- **Pack** — JSON config tying soul + personas + hooks together

## The Easy Way: Inline Provisioning

You can define everything inline in your seed script and provision it via the Starflask API — no Axoniac CLI, no separate repos, no manual hash management.

### 1. Define the soul, personas, and pack in your seed script

```js
import { Starflask } from "starflask";

const sf = new Starflask({
  apiKey: process.env.STARFLASK_API_KEY,
  baseUrl: process.env.STARFLASK_API_URL,
});

const SOUL = {
  name: "my_agent",
  description: "What this agent is",
  content: "# My Agent - Soul\n\nYou are...",
  tags: ["tag1", "tag2"],
};

const PERSONAS = [
  {
    name: "my_agent_reactive",
    description: "What this persona handles",
    content: "# My Agent — Reactive Hook\n\nDo this when triggered...",
    tags: ["hook", "reactive"],
  },
];

const PACK = {
  name: "My Agent Pack",
  description: "What this pack does.",
  version: "1.0.0",
  definition: {
    role: "my-agent-role",
    tool_sets: [],
    soul: { name: "my_agent", hash: "" },
    skills: [],
    hooks: [
      {
        event: "my_event",
        persona: { name: "my_agent_reactive", hash: "" },
      },
    ],
    recommended_integrations: [],
  },
  metadata: { category: "creative", icon: "wand" },
};
```

### 2. Create agent and provision

```js
// Create the agent
const agent = await sf.createAgent({ name: "My App Agent" });

// Provision the pack (creates soul + personas + pack in Axoniac, installs on agent)
const result = await sf.provisionPack(agent.id, {
  soul: SOUL,
  personas: PERSONAS,
  pack: PACK,
});
// result.content_hash is the pack's hash

// Activate
await sf.setAgentActive(agent.id, true);

// Verify
const hooks = await sf.getHooks(agent.id);
console.log(hooks.hooks.map(h => h.event));
```

### 3. Use the agent

```js
// Fire and wait
const session = await sf.fireHookAndWait(
  agent.id,
  "my_event",
  { key: "value" },
  { timeoutMs: 60_000 }
);
console.log(session.result);
```

That's it. One API key, one script, everything provisioned.

## Writing Good Packs

### Soul

The soul defines **who** the agent is. Keep it focused on identity, not instructions.

```
name: snake_case, unique
content: Markdown describing personality, expertise, communication style
```

Key sections:
- **Identity** — Who is this agent? What's its expertise?
- **Core Truths** — Numbered principles it always follows
- **Output Format** — If it should return structured data (JSON), specify here
- **What You Don't Do** — Boundaries and anti-patterns

### Persona

A persona defines **what to do** when a specific hook fires. Each persona maps to one hook event.

```
name: snake_case, unique
content: Markdown with instructions for handling the event
```

Tips:
- Use `{variable_name}` placeholders for data from the hook payload
- Be very explicit about output format — include a full example
- The worker will try to parse the response, so structured output matters

### Pack Definition

```json
{
  "role": "descriptive-role-name",
  "tool_sets": [],
  "soul": { "name": "soul_name", "hash": "" },
  "skills": [],
  "hooks": [
    { "event": "event_name", "persona": { "name": "persona_name", "hash": "" } }
  ],
  "recommended_integrations": []
}
```

- Leave `hash` fields as `""` — the provision endpoint computes them from content
- `soul.name` must match the soul's `name`
- `persona.name` in hooks must match the persona's `name`
- `tool_sets`: array of tools the agent can use (e.g., `["discord", "bash"]`). Empty for pure text generation.
- `event`: the hook event name your mini-app fires

### Patterns

| Pattern | tool_sets | skills | hooks | heartbeat_schedule |
|---------|-----------|--------|-------|--------------------|
| Pure generation (text in, text out) | `[]` | `[]` | 1 reactive | none |
| Discord bot | `["discord"]` | moderation skills | reactive + heartbeat | `"*/5 * * * *"` |
| Multi-hook agent | varies | varies | multiple events, each with its own persona | optional |

## Limits

- Users are limited to **10 agents** max
- Provisioning requires an **active subscription or positive credit balance**
- Each hook fire costs **1 credit**
- Provisioning the same pack twice is idempotent (content-hash based dedup)

## Alternative: Axoniac Seed Files

For platform-level packs (not per-app), you can still use the Axoniac seed system:

```
~/ai/axoniac-monorepo/seed/
  souls/my_agent.md          # Markdown with YAML frontmatter
  personas/my_persona.md     # Markdown with YAML frontmatter
  agent-packs/my-pack.json   # Pack definition JSON
```

Then run `cd ~/ai/axoniac-monorepo && cargo run --bin seed` to load them into the database.

## Example

See `apps/swatch-swipe/seed.js` for a complete working example that provisions the Palette Generator pack.
