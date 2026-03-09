# Swatch Swipe

Tinder for color palettes. Describe a design vibe, get AI-generated color palettes, swipe to save your favorites.

Powered by [Starflask](https://starflask.com) — the AI agent does the creative work, your app just calls one function.

## How it works

1. You type a design premise ("cyberpunk dashboard", "cozy coffee shop")
2. Backend fires a `generate_palette` hook to a Starflask agent
3. The agent — a color theorist persona — generates 3 distinct palettes with named colors, roles, and mood descriptions
4. You swipe through them, saving the ones you like

The entire AI integration is a single function call:

```rust
let session = sf.fire_hook_and_wait(
    &agent_id,
    "generate_palette",
    json!({ "premise": "cyberpunk dashboard" }),
).await?;
```

Starflask handles the LLM calls, agentic loop, prompt assembly, and structured output. Your app just gets back palettes.

## Project structure

```
swatch-swipe/
├── backend/
│   └── src/
│       ├── main.rs         # Axum server, 4 routes
│       ├── models.rs       # Palette, PaletteColor, request/response types
│       ├── handlers.rs     # Endpoint handlers
│       ├── starflask.rs    # Starflask client + response parsing
│       └── seed.rs         # One-time agent provisioning script
└── frontend/
    └── src/
        ├── App.tsx                       # Tab navigation (Generate / Collection)
        ├── types.ts                      # TypeScript interfaces
        ├── api.ts                        # Axios client (23 lines)
        └── components/
            ├── GeneratorView.tsx         # Premise input + swipe UI
            ├── CollectionView.tsx        # Saved palettes grid
            └── PaletteCard.tsx           # Color swatch card
```

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- [Node.js](https://nodejs.org/) 20+
- A running Starflask instance (or use mock mode for frontend dev)

## Setup

### 1. Seed the agent

The seed script creates a "Swatch Swipe" agent in Starflask and provisions its soul prompt, persona, and hook configuration.

```bash
cd backend
cp .env.example .env

# Run the seed (only needs STARFLASK_API_KEY and optionally STARFLASK_API_URL)
STARFLASK_API_KEY=sk_your-key cargo run --bin seed
```

The seed will print the agent ID. Add it to your `.env`:

```env
STARFLASK_API_URL=http://localhost:8080
STARFLASK_API_KEY=sk_your-key
STARFLASK_AGENT_ID=<printed-agent-id>
PORT=3001
```

You can verify it works immediately:

```bash
STARFLASK_API_KEY=sk_your-key cargo run --bin seed -- --test
```

### 2. Run the backend

```bash
cd backend
cargo run
```

The server starts on port 3001 (configurable via `PORT`).

### 3. Run the frontend

```bash
cd frontend
npm install
npm run dev
```

Opens at http://localhost:5173. The Vite dev server proxies `/api` requests to the backend on port 3001.

## Mock mode

If you omit the `STARFLASK_*` env vars, the backend returns hardcoded mock palettes. This lets you develop the frontend without running Starflask or any AI services.

## API

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/generate` | Generate palettes from a premise |
| `GET` | `/api/palettes` | List saved palettes |
| `POST` | `/api/palettes/save` | Save a palette to your collection |
| `DELETE` | `/api/palettes/{id}` | Remove a saved palette |

### Generate palettes

```bash
curl -X POST http://localhost:3001/api/generate \
  -H "Content-Type: application/json" \
  -d '{"premise": "minimal SaaS landing page"}'
```

Returns:

```json
{
  "palettes": [
    {
      "name": "Glacier Protocol",
      "mood": "Clean and confident",
      "colors": [
        { "hex": "#f8fafc", "name": "Snowfield", "role": "background" },
        { "hex": "#f1f5f9", "name": "Frost Glass", "role": "surface" },
        { "hex": "#0f172a", "name": "Obsidian", "role": "primary" },
        { "hex": "#475569", "name": "Slate Drift", "role": "secondary" },
        { "hex": "#3b82f6", "name": "Signal Blue", "role": "accent" },
        { "hex": "#0f172a", "name": "Deep Ink", "role": "text" }
      ],
      "use_case": "SaaS dashboards and marketing sites that need to feel trustworthy"
    }
  ]
}
```

## Tech stack

**Backend:** Rust, Axum, starflask-rs SDK, tokio, serde, tower-http

**Frontend:** React 19, TypeScript, Vite, Tailwind CSS 4, Framer Motion, Axios

## Production deployment

Build the frontend and serve it from the backend:

```bash
cd frontend && npm run build
cd ../backend && STATIC_DIR=../frontend/dist cargo run
```

The backend serves the built frontend as static files with SPA fallback, so everything runs on a single port.
