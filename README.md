# starflask-miniapps

A collection of niche webapps powered by Starflask (+ Axoniac) as their core AI backend engine.

Each app has a Starflask account and API key, and uses Starflask agents to handle AI-powered functionality.

## Apps

### Swatch Swipe
**Tinder for Color Palettes.** Enter a design premise, get AI-generated color palettes, swipe to save your favorites.

- `apps/swatch-swipe/backend/` — Rust/Axum proxy server
- `apps/swatch-swipe/frontend/` — React/Vite/Tailwind UI

#### AI DevOps with Claude Code

Agent provisioning is done conversationally through Claude Code. Tell it:

> run seed using starflask api key sk_eb4ba3...

Claude Code runs the Rust seed binary, which creates the Starflask agent, provisions the pack, and returns output like:

```
Seed completed successfully. Agent "Swatch Swipe" is created and provisioned
with the generate_palette hook at agent ID e3773c98-5e3c-438c-a2e8-062c46377f15
```

Add the returned agent ID to your backend `.env`:

```env
STARFLASK_API_URL=https://starflask.com
STARFLASK_API_KEY=sk_<your key>
STARFLASK_AGENT_ID=e3773c98-5e3c-438c-a2e8-062c46377f15
```

Or run it directly:

```bash
cd apps/swatch-swipe/backend
STARFLASK_API_KEY=sk_... STARFLASK_API_URL=https://starflask.com/api cargo run --bin seed
```

#### Quick start

```bash
# Backend
cd apps/swatch-swipe/backend
cp .env.example .env  # fill in credentials from seed output
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
- **Seed**: Rust binary (`cargo run --bin seed`) using the `starflask` crate — provisions the agent in one command

See [AXONIAC_AGENT_SEED.md](./AXONIAC_AGENT_SEED.md) for how to create new Axoniac agent packs.
