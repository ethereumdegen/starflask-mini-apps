# Creating Axoniac Agent Packs for Starflask Mini-Apps

This guide explains how to create a new Axoniac agent pack that a Starflask mini-app can use.

## Overview

Each mini-app talks to a **Starflask agent**, which is powered by an **Axoniac agent pack**. A pack is a bundle of:

- **Soul** — The agent's core identity/personality (markdown)
- **Persona(s)** — Context-specific instructions triggered by hooks (markdown)
- **Agent Pack** — JSON config tying soul + personas + hooks together

All seed data lives in `~/ai/axoniac-monorepo/seed/`.

## Step-by-step

### 1. Create the Soul

File: `seed/souls/{your_agent_name}.md`

```markdown
---
name: your_agent_name
description: One-line description of the agent's personality.
tags: [relevant, tags, here]
---

# Agent Name - Soul

Who is this agent? What's its identity?

## Identity
Describe personality, expertise, communication style.

## Core Truths
Numbered list of principles the agent follows.

## Output Format
If the agent should return structured data (JSON, etc.), specify here.

## What You Don't Do
Boundaries and anti-patterns.
```

**Key rules:**
- `name` in frontmatter must be unique and use snake_case
- The body (after frontmatter) is the actual soul content that gets hashed
- Content hash is computed as SHA-256 of the body text
- Keep it focused — the soul defines *who*, not *what to do*

### 2. Create the Persona(s)

File: `seed/personas/{your_persona_name}.md`

```markdown
---
name: your_persona_name
description: One-line description of what this persona handles.
tags: [relevant, tags, hook, reactive]
---

# Persona Name — Reactive Hook

[HOOK DESCRIPTION — What triggered this]
Variable: {variable_name}

---

Instructions for what the agent should do when this hook fires.

## Output Format
Specify exact expected output format.

## Rules
Numbered constraints.
```

**Key rules:**
- `name` must be unique, snake_case
- Use `{variable_name}` placeholders for data injected from the hook payload
- Each persona maps to one hook event
- Be explicit about output format — the worker will try to parse the response

### 3. Create the Agent Pack

File: `seed/agent-packs/{your-pack-name}.json`

```json
{
  "name": "Human Readable Pack Name",
  "description": "What this agent pack does.",
  "version": "1.0.0",
  "definition": {
    "role": "your-agent-role",
    "tool_sets": [],
    "soul": { "name": "your_agent_name", "hash": "" },
    "skills": [],
    "hooks": [
      {
        "event": "your_hook_event",
        "persona": { "name": "your_persona_name", "hash": "" }
      }
    ],
    "recommended_integrations": []
  },
  "metadata": {
    "category": "creative",
    "icon": "palette"
  }
}
```

**Key rules:**
- `soul.name` must match the soul's frontmatter `name`
- `persona.name` in hooks must match the persona's frontmatter `name`
- Leave `hash` as `""` — the seed script auto-computes content hashes
- `tool_sets` is an array of tool names the agent can use (e.g., `["discord", "bash"]`). Leave empty for pure text generation agents.
- `event` is the hook event name your mini-app will fire via `POST /api/worker/fire_event`
- `recommended_integrations` lists platform integrations (discord, github, etc.) — leave empty for API-only agents

### 4. Run the Seed Script

```bash
cd ~/ai/axoniac-monorepo
cargo run --bin seed
```

This will:
1. Parse all markdown files, extract frontmatter + body
2. Compute SHA-256 content hashes
3. Upload skill content to S3 (if any skills)
4. Insert/update souls, personas, skills in the database
5. Backfill hashes into agent pack definitions
6. Insert/update agent packs

### 5. Link in Starflask

After seeding, you need to:

1. Find the agent pack's `content_hash` from Axoniac (check DB or API)
2. Create a Starflask agent linked to it via `axoniac_agent_hash`
3. Note the agent's UUID — this is what your mini-app uses as `STARFLASK_AGENT_ID`

### 6. Configure the Mini-App

In your mini-app's `.env`:

```env
STARFLASK_API_URL=http://localhost:8080   # or production URL
STARFLASK_SECRET_KEY=your-secret-key      # matches Starflask's STARFLASK_SECRET_KEY
STARFLASK_AGENT_ID=uuid-of-the-agent      # from step 5
```

The mini-app backend fires events via:

```
POST {STARFLASK_API_URL}/api/worker/fire_event
Authorization: Bearer {hmac_sha256_hex("starflask-worker-auth", STARFLASK_SECRET_KEY)}
Body: { "agent_id": "...", "event": "your_hook_event", "payload": { ... } }
```

## Example: Palette Generator (Swatch Swipe)

| Component | File | Name |
|-----------|------|------|
| Soul | `seed/souls/palette_generator.md` | `palette_generator` |
| Persona | `seed/personas/palette_generator_reactive.md` | `palette_generator_reactive` |
| Pack | `seed/agent-packs/palette-generator.json` | `Palette Generator` |
| Hook event | — | `generate_palette` |
| Payload | — | `{ "premise": "user's design theme description" }` |

## Tips

- **Pure generation agents** (no tools needed): leave `tool_sets` and `skills` empty
- **Heartbeat agents** (periodic tasks): add `"heartbeat_schedule": "*/5 * * * *"` to the definition and a heartbeat persona
- **Multi-hook agents**: add multiple entries to the `hooks` array, each with a different event + persona
- **Structured output**: Be very explicit in the persona about the expected JSON format. Include a full example. The worker/mini-app will need to parse this.
- **Content hashes**: Don't worry about computing them manually — the seed script handles it. Leave `hash` fields as `""`.
