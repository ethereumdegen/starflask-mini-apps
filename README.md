# starflask-miniapps

A collection of niche webapps powered by Starflask (+ Axoniac) as their core AI backend engine.

Each app has a Starflask account and API key, and uses Starflask agents to handle AI-powered functionality.

## Apps

### Swatch Swipe
**Tinder for Color Palettes.** Enter a design premise, get AI-generated color palettes, swipe to save your favorites.

- `apps/swatch-swipe/backend/` — Rust/Axum proxy server
- `apps/swatch-swipe/frontend/` — React/Vite/Tailwind UI

#### Quick start
```bash
# Backend
cd apps/swatch-swipe/backend
cp .env.example .env  # configure your Starflask credentials
cargo run

# Frontend
cd apps/swatch-swipe/frontend
npm install
npm run dev
```

Works in mock mode (no Starflask needed) — just leave STARFLASK_API_URL unset.

## Architecture

Each mini-app follows the same pattern:
- **Frontend**: React + Vite + TypeScript + Tailwind
- **Backend**: Rust + Axum (thin proxy to Starflask)
- **AI**: Starflask agent with Axoniac agent pack (soul + persona + hooks)

See [AXONIAC_AGENT_SEED.md](./AXONIAC_AGENT_SEED.md) for how to create new Axoniac agent packs.
