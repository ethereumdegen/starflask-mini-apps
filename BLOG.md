# Building an AI-Powered App in 400 Lines of Code

**How Starflask turns "build an AI agent" into "call one function"**

---

You know that feeling when someone shows you an AI demo and you think "cool, but how many thousands of lines of LLM plumbing, prompt engineering, and retry logic is buried under that?"

What if I told you an entire AI-powered app — one where a language model generates creative, structured output on demand — could fit in about 400 lines of actual application code?

That's what we built with **Swatch Swipe**, and the reason it's so small is **Starflask**.

## What is Swatch Swipe?

Swatch Swipe is "Tinder for color palettes." You describe a design vibe — *cyberpunk dashboard*, *cozy coffee shop*, *brutalist SaaS* — and an AI agent generates three distinct color palettes for you. You swipe to save or skip. That's it.

Behind that simple interaction is a real AI agent: it has a soul prompt defining its personality as a color theorist, it understands WCAG contrast ratios, it names colors evocatively ("Midnight Ink" instead of "Dark Blue #3"), and it returns structured JSON with exact hex codes, roles, and mood descriptions.

The interesting part isn't what it does. It's how little code it takes.

## The entire backend: 4 files, ~200 lines

The Swatch Swipe backend is a Rust/Axum server. Here's the complete file list:

- `main.rs` — server setup and routes (55 lines)
- `models.rs` — data structures (42 lines)
- `handlers.rs` — endpoint logic (80 lines)
- `starflask.rs` — the AI integration (46 lines of actual client code)

Let's look at the interesting parts.

### Models: just regular data structures

```rust
pub struct Palette {
    pub name: String,
    pub mood: String,
    pub colors: Vec<PaletteColor>,
    pub use_case: String,
}

pub struct PaletteColor {
    pub hex: String,
    pub name: String,
    pub role: String,
}
```

Nothing AI-specific here. These are the same structs you'd write for any color palette app. The AI doesn't leak into your data model.

### The AI integration: one function call

Here's the core of the entire AI interaction:

```rust
pub async fn generate_palettes(&self, premise: &str) -> Result<Vec<Palette>, String> {
    let session = self.sf
        .fire_hook_and_wait(
            &self.agent_id,
            "generate_palette",
            serde_json::json!({ "premise": premise }),
        )
        .await
        .map_err(|e| e.to_string())?;

    extract_palettes(&session.result)
}
```

That's it. One function call: `fire_hook_and_wait`. You tell Starflask which agent, which hook event, and what payload to send. Starflask handles everything else:

- Routing the hook to the right persona prompt
- Calling the LLM
- Running the agentic loop (tool calls, retries, iteration)
- Returning structured results

Your app just gets back a session with a `result` field containing exactly the structured data the agent produced.

### The handler: just normal web code

```rust
pub async fn generate_palette(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, (StatusCode, String)> {
    let premise = req.premise.trim().to_string();

    let palettes = if let Some(ref client) = state.starflask_client {
        client.generate_palettes(&premise).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
    } else {
        starflask::mock_palettes(&premise)  // offline dev mode
    };

    Ok(Json(GenerateResponse { palettes }))
}
```

This looks like any CRUD endpoint. There's no prompt engineering, no token counting, no retry logic, no streaming handler, no model selection. The AI is just another service call — like calling Stripe or S3.

### Server setup: 4 routes

```rust
let api_routes = Router::new()
    .route("/api/generate", post(handlers::generate_palette))
    .route("/api/palettes", get(handlers::list_palettes))
    .route("/api/palettes/save", post(handlers::save_palette))
    .route("/api/palettes/{id}", delete(handlers::delete_palette));
```

Four endpoints. One of them talks to AI. The others are standard CRUD for saving favorites. That's the entire backend API.

## The frontend: completely AI-unaware

The React frontend has zero knowledge that AI exists. Here's the API client in its entirety:

```typescript
const api = axios.create({ baseURL: '/api' });

export async function generatePalettes(premise: string): Promise<Palette[]> {
  const { data } = await api.post('/generate', { premise });
  return data.palettes;
}

export async function getSavedPalettes(): Promise<SavedPalette[]> {
  const { data } = await api.get('/palettes');
  return data;
}

export async function savePalette(palette: Palette, premise: string): Promise<SavedPalette> {
  const { data } = await api.post('/palettes/save', { ...palette, premise });
  return data;
}

export async function deletePalette(id: string): Promise<void> {
  await api.delete(`/palettes/${id}`);
}
```

23 lines. Four functions. `generatePalettes` could be hitting a database of pre-made palettes for all the frontend knows. The fact that a language model is inventing these in real-time is completely invisible.

The generator component calls it like any async operation:

```typescript
const handleGenerate = async () => {
  setLoading(true);
  try {
    const palettes = await generatePalettes(premise.trim());
    setQueue(palettes);
  } catch (err) {
    console.error('Failed to generate:', err);
  } finally {
    setLoading(false);
  }
};
```

No WebSocket connections, no streaming tokens, no polling for completion. Just `await` and you have palettes.

## What Starflask is doing behind the scenes

When your app calls `fire_hook_and_wait("generate_palette", { premise: "cyberpunk dashboard" })`, here's what Starflask handles for you:

1. **Session creation** — Creates a worker session, tracks its lifecycle
2. **Agent configuration** — Looks up the agent's soul prompt, finds the persona linked to the `generate_palette` hook
3. **Prompt assembly** — Combines the soul prompt ("You are a color palette generator — part color theorist, part UI designer...") with the persona prompt, injecting the `{premise}` variable
4. **LLM orchestration** — Calls the language model with the assembled prompt and available tools
5. **Agentic loop** — If the LLM calls tools (like `report_result`), executes them and loops back until the agent signals completion
6. **Result extraction** — Returns the structured data the agent produced via `report_result`

Your app doesn't manage any of this. You don't pick a model. You don't format prompts. You don't handle tool calling. You don't implement retry logic. You don't parse streaming responses.

You define what your agent *is* (its soul and personas), and Starflask runs it.

## Agent setup: a seed script

The agent's personality and behavior are defined in a seed script that runs once:

```rust
let sf = Starflask::new(&api_key, Some(&base_url))?;

// Find or create the agent
let agent = sf.create_agent("Swatch Swipe").await?;

// Provision the pack (soul + personas + hooks)
sf.provision_pack(&agent.id, pack_definition()).await?;

// Activate it
sf.set_agent_active(&agent.id, true).await?;
```

The `pack_definition()` contains the soul prompt, the reactive persona, and the hook configuration — all as a JSON value. Run the seed once, and your agent exists in Starflask, ready to handle requests.

The soul prompt tells the agent who it is:

> *You are a color palette generator — part color theorist, part UI designer, part trend-spotter. You live and breathe color.*

The persona prompt tells it what to do when the hook fires:

> *Generate 3 unique color palettes inspired by the premise. Call `report_result` with `structured_data` containing the palettes as JSON.*

That's the full AI configuration. It lives in your seed script, version-controlled alongside your app code.

## The line count

Let's be honest about the numbers:

| File | Lines | What it does |
|------|-------|-------------|
| `main.rs` | 93 | Server, routes, config |
| `models.rs` | 43 | Data structures |
| `handlers.rs` | 80 | 4 endpoint handlers |
| `starflask.rs` | 46* | AI client + result parsing |
| `App.tsx` | 53 | Tab navigation shell |
| `types.ts` | 17 | TypeScript interfaces |
| `api.ts` | 23 | HTTP client |
| `GeneratorView.tsx` | 132 | Swipe UI + animations |
| `CollectionView.tsx` | 47 | Saved palettes grid |
| `PaletteCard.tsx` | 57 | Color swatch display |

*\*46 lines for the Starflask client itself. There's additional extraction/parsing logic for handling various response formats, which you'd have less of in a production setup with a well-tuned agent.*

**Total application code: ~590 lines.** And most of that is UI. The AI integration is a rounding error.

## What this means

Swatch Swipe is a toy. But the pattern scales.

The same architecture works for an app that analyzes documents, generates reports, processes customer support tickets, or orchestrates multi-step workflows. Your app stays simple because Starflask absorbs the complexity:

- **You don't build an agentic loop.** Starflask runs it.
- **You don't manage LLM calls.** Starflask makes them.
- **You don't implement tool execution.** Starflask has built-in tools (Twitter, Discord, Google Workspace, HTTP) and handles the tool-call cycle.
- **You don't handle sessions.** Starflask tracks every session, logs every iteration, and stores results.
- **You don't write prompt infrastructure.** You write prompts as text in your agent pack. Starflask assembles and injects them.

Your code just does what application code should do: define routes, handle requests, render UI. The AI is a service you call — not infrastructure you maintain.

That's the point. An AI-powered app should be as simple to build as any other app. With Starflask, it is.

---

*Swatch Swipe is open source in the [starflask-miniapps](https://github.com/anthropics/starflask-miniapps) repository. Starflask is at [starflask.com](https://starflask.com).*
