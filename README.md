# Starflask Miniapps

A collection of AI-powered web apps built on [Starflask](https://starflask.com). Each one is a working product — and each one is shockingly small, because Starflask handles all the hard parts.

## Why Starflask

Building an AI agent from scratch means wiring up LLM API calls, implementing a tool-calling loop, managing retries and timeouts, assembling prompts, tracking sessions, parsing structured output, and hoping it all holds together. That's hundreds of lines of infrastructure before you write a single line of product code.

Starflask replaces all of that with one concept: **fire a hook, get a result.**

```rust
let session = sf.fire_hook_and_wait(
    &agent_id,
    "generate_palette",
    json!({ "premise": "cyberpunk dashboard" }),
).await?;

// session.result contains structured data from the AI agent
```

That's the entire AI integration. One function call. Starflask takes care of:

- **Prompt assembly** — Your agent has a soul (who it is) and personas (what it does for each hook event). Starflask assembles the right prompt automatically.
- **LLM orchestration** — Model selection, API calls, retries, token management. You never touch it.
- **Agentic loop** — If the agent needs to call tools (report results, make HTTP requests, post to Twitter, etc.), Starflask runs the full tool-call cycle until the agent signals completion.
- **Session lifecycle** — Every request becomes a tracked session with status, logs, iterations, and results. Debug by looking at the session, not by adding print statements.
- **Structured output** — Agents return data via `report_result` with a summary and machine-readable `structured_data`. Your app parses JSON, not prose.
- **Built-in tools** — Twitter, Discord, Google Workspace, HTTP fetch — available to agents based on configured integrations. No tool implementation on your side.

The result: **your app code is just normal app code.** Routes, handlers, UI components. The AI is a service call, not an architecture.

## Apps

### [Swatch Swipe](./apps/swatch-swipe/)

**Tinder for Color Palettes.** Enter a design premise, get AI-generated color palettes, swipe to save your favorites.

- Backend: Rust/Axum, ~200 lines of app code
- Frontend: React/Vite/Tailwind, ~330 lines
- AI integration: 1 function call (`fire_hook_and_wait`)

The AI agent is a color theorist persona that understands WCAG contrast ratios, names colors evocatively, and returns structured JSON with hex codes, roles, and mood descriptions. Your app just renders what it gets back.

See the [full walkthrough](./BLOG.md) for a detailed look at the code.

## The pattern

Every miniapp follows the same structure:

```
app/
├── backend/
│   └── src/
│       ├── main.rs          # Server + routes
│       ├── models.rs         # Your data types (no AI stuff)
│       ├── handlers.rs       # Normal endpoint handlers
│       ├── starflask.rs      # fire_hook_and_wait + parse result
│       └── seed.rs           # One-time agent setup
└── frontend/
    └── src/
        ├── App.tsx           # UI shell
        ├── api.ts            # HTTP client (hits your backend, not Starflask)
        └── components/       # Normal React components
```

The key insight: **the frontend doesn't know AI exists.** It calls your backend API, which returns typed data. Whether that data came from a database, a cache, or an AI agent is an implementation detail hidden behind a normal endpoint.

## Getting started

### 1. Seed the agent

Each app has a seed script that provisions its Starflask agent in one command:

```bash
cd apps/swatch-swipe/backend
STARFLASK_API_KEY=sk_your-key cargo run --bin seed
```

This creates the agent, uploads its soul prompt and personas, configures hooks, and prints the agent ID for your `.env`.

### 2. Run the backend

```bash
cd apps/swatch-swipe/backend
cp .env.example .env   # add your API key and agent ID
cargo run
```

### 3. Run the frontend

```bash
cd apps/swatch-swipe/frontend
npm install
npm run dev
```

### Mock mode

Leave `STARFLASK_API_URL` unset and the backend returns mock data. This lets you develop the UI without running Starflask or any AI services.

## Tech stack

- **Backend:** Rust, Axum, [starflask-rs](https://crates.io/crates/starflask) SDK
- **Frontend:** React 19, TypeScript, Vite, Tailwind CSS 4
- **AI platform:** [Starflask](https://starflask.com) + [Axoniac](https://axoniac.com) (agent packs)

## Resources

- [BLOG.md](./BLOG.md) — Deep dive into how Swatch Swipe works and why it's so small
- [DEVOPS.md](./DEVOPS.md) — Full setup and deployment guide
- [AXONIAC_AGENT_SEED.md](./AXONIAC_AGENT_SEED.md) — How to create agent packs
