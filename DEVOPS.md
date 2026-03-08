# Devops

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  Swatch Swipe   │     │    Starflask     │     │    Axoniac       │
│  Frontend       │────▶│    Backend       │────▶│    Backend       │
│  :5173          │     │    :8080         │     │    :8081         │
└─────────────────┘     └──────────────────┘     └──────────────────┘
        │                       │                        │
        ▼                       ▼                        ▼
┌─────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  Swatch Swipe   │     │   Postgres       │     │   Postgres       │
│  Backend        │     │   (starflask)    │     │   (axoniac)      │
│  :3001          │     └──────────────────┘     └──────────────────┘
└─────────────────┘
```

The swatch-swipe frontend proxies `/api` to the swatch-swipe backend (:3001).
The swatch-swipe backend calls Starflask's worker API to fire hooks and poll sessions.
Starflask proxies pack provisioning and bundle fetching to Axoniac.

## Prerequisites

- Rust toolchain (rustup)
- Node.js 20+
- PostgreSQL (two databases: `starflask` and `axoniac`)
- Axoniac backend running locally
- Starflask backend running locally
- A Starflask user account (via Clerk)

## Step-by-step

### 1. Databases

```bash
createdb starflask
createdb axoniac
```

### 2. Start Axoniac

```bash
cd ~/ai/axoniac-monorepo
# Ensure .env has DATABASE_URL, SERVICE_JWT_SECRET, S3 config, etc.
cargo run --bin migrate
cargo run --bin seed       # loads base packs, souls, personas
cargo run                  # starts on :8081
```

### 3. Start Starflask

```bash
cd ~/ai/starflask-monorepo
# Ensure .env has:
#   DATABASE_URL=postgres://...
#   AXONIAC_API_URL=http://localhost:8081
#   AXONIAC_SECRET_KEY=<matches axoniac's expected key>
#   STARFLASK_SECRET_KEY=<any secret string>
#   SERVICE_JWT_SECRET=<any secret string>
#   CLERK_DOMAIN=<your clerk domain>
#   CLERK_SECRET_KEY=<your clerk key>
cargo run --bin migrate
cargo run                  # starts on :8080
```

### 4. Get a Starflask API key

Log into the Starflask frontend to create a user account (Clerk auth).
Then grab your JWT and create an API key:

```bash
# Exchange your Clerk token for a Starflask JWT
CLERK_TOKEN="<from browser devtools>"
JWT=$(curl -s -X POST http://localhost:8080/api/auth/exchange \
  -H "Authorization: Bearer $CLERK_TOKEN" | jq -r '.token')

# Create an API key
curl -s -X POST http://localhost:8080/api/auth/api-keys \
  -H "Authorization: Bearer $JWT" \
  -H "Content-Type: application/json" \
  -d '{"name":"swatch-swipe"}' | jq .

# Save the "api_key" value (sk_...) — it's only shown once
```

### 5. Seed the Swatch Swipe agent

```bash
cd ~/ai/starflask-miniapps
STARFLASK_API_KEY=sk_... node apps/swatch-swipe/seed.js
```

This will:
- Create an agent named "Swatch Swipe" in Starflask
- Provision the Palette Generator pack (soul + persona) through Starflask → Axoniac
- Activate the agent
- Print the agent ID and `.env` values

### 6. Start Swatch Swipe backend

```bash
cd ~/ai/starflask-miniapps/apps/swatch-swipe/backend
cp .env.example .env
```

Fill in `.env` with the values from the seed output:

```env
STARFLASK_API_URL=http://localhost:8080
STARFLASK_SECRET_KEY=<same value as Starflask's STARFLASK_SECRET_KEY>
STARFLASK_AGENT_ID=<uuid printed by seed.js>
PORT=3001
```

```bash
cargo run                  # starts on :3001
```

### 7. Start Swatch Swipe frontend

```bash
cd ~/ai/starflask-miniapps/apps/swatch-swipe/frontend
npm install
npm run dev                # starts on :5173, proxies /api → :3001
```

Open http://localhost:5173

## Mock mode (UI development only)

To work on the frontend without running Starflask/Axoniac, leave `STARFLASK_API_URL` unset in the backend `.env`. The backend will return mock palettes.

```bash
cd apps/swatch-swipe/backend
echo "PORT=3001" > .env
cargo run

# In another terminal
cd apps/swatch-swipe/frontend
npm run dev
```

## Adding a new mini-app

1. Create `apps/your-app/backend/` (Rust/Axum) and `apps/your-app/frontend/` (React/Vite)
2. Define your agent pack inline in `apps/your-app/seed.js` (see `apps/swatch-swipe/seed.js`)
3. Run `STARFLASK_API_KEY=sk_... node apps/your-app/seed.js`
4. Use the `starflask` npm package (`packages/starflask/`) for API calls
5. See [AXONIAC_AGENT_SEED.md](./AXONIAC_AGENT_SEED.md) for pack authoring guide

## Ports

| Service              | Port  |
|----------------------|-------|
| Axoniac backend      | 8081  |
| Starflask backend    | 8080  |
| Swatch Swipe backend | 3001  |
| Swatch Swipe frontend| 5173  |

## Troubleshooting

**`UNAUTHORIZED` from seed.js** — API key is wrong or the `user_api_keys` migration hasn't run. Run `cargo run --bin migrate` in `sf-backend`.

**`PAYMENT_REQUIRED` from provision-pack** — User has no active subscription and zero credits. Grant credits via admin: `POST /api/admin/credits/grant { "user_id": "...", "amount": 100 }`.

**`FORBIDDEN` from create agent** — Hit the 10-agent limit. Delete unused agents first.

**`BAD_GATEWAY` from provision-pack** — Axoniac is not reachable. Check `AXONIAC_API_URL` in Starflask's `.env` and that Axoniac is running.

**Mock palettes only** — `STARFLASK_API_URL` is not set in swatch-swipe backend `.env`. Set it and restart.

**Session stays "ready" forever** — The Starflask worker (`sf-worker`) isn't running. It picks up sessions and processes them. Start it separately.
