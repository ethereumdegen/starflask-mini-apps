/**
 * Starflask API client.
 *
 * Usage:
 *   import { Starflask } from 'starflask';
 *   const sf = new Starflask({ apiKey: 'sk_...', baseUrl: 'https://...' });
 *   const agents = await sf.listAgents();
 */

export class Starflask {
  #apiKey;
  #baseUrl;

  /**
   * @param {{ apiKey: string, baseUrl?: string }} opts
   */
  constructor({ apiKey, baseUrl = "http://localhost:8080/api" }) {
    if (!apiKey) throw new Error("apiKey is required");
    this.#apiKey = apiKey;
    this.#baseUrl = baseUrl.replace(/\/+$/, "");
  }

  /** @param {string} path @param {RequestInit} opts */
  async #request(path, opts = {}) {
    const url = `${this.#baseUrl}${path}`;
    const res = await fetch(url, {
      ...opts,
      headers: {
        Authorization: `Bearer ${this.#apiKey}`,
        "Content-Type": "application/json",
        ...opts.headers,
      },
    });
    if (!res.ok) {
      const body = await res.text().catch(() => "");
      throw new Error(`Starflask ${res.status} ${res.statusText}: ${body}`);
    }
    if (res.headers.get("content-type")?.includes("application/json")) {
      return res.json();
    }
    return res.text();
  }

  // ── Agents ──────────────────────────────────────────────

  /** List all agents for this user. */
  async listAgents() {
    return this.#request("/agents");
  }

  /** Create a new agent. @param {{ name?: string }} opts */
  async createAgent(opts = {}) {
    return this.#request("/agents", {
      method: "POST",
      body: JSON.stringify(opts),
    });
  }

  /** Get a single agent by ID. @param {string} agentId */
  async getAgent(agentId) {
    return this.#request(`/agents/${agentId}`);
  }

  /** Update an agent. @param {string} agentId @param {{ name?: string, description?: string }} data */
  async updateAgent(agentId, data) {
    return this.#request(`/agents/${agentId}`, {
      method: "PUT",
      body: JSON.stringify(data),
    });
  }

  /** Delete an agent. @param {string} agentId */
  async deleteAgent(agentId) {
    return this.#request(`/agents/${agentId}`, { method: "DELETE" });
  }

  /** Set agent active status. @param {string} agentId @param {boolean} active */
  async setAgentActive(agentId, active) {
    return this.#request(`/agents/${agentId}/active`, {
      method: "PUT",
      body: JSON.stringify({ active }),
    });
  }

  /** Set agent heartbeat schedule. @param {string} agentId @param {string|null} schedule */
  async setHeartbeatSchedule(agentId, schedule) {
    return this.#request(`/agents/${agentId}/heartbeat_schedule`, {
      method: "PUT",
      body: JSON.stringify({ heartbeat_schedule: schedule }),
    });
  }

  // ── Agent Packs ─────────────────────────────────────────

  /** Install an Axoniac agent pack on an agent. @param {string} agentId @param {string} contentHash */
  async installAgentPack(agentId, contentHash) {
    return this.#request(`/agents/${agentId}/agent-pack`, {
      method: "PUT",
      body: JSON.stringify({ content_hash: contentHash }),
    });
  }

  // ── Hooks ───────────────────────────────────────────────

  /** Get available hooks for an agent. @param {string} agentId */
  async getHooks(agentId) {
    return this.#request(`/agents/${agentId}/hooks`);
  }

  /** Fire a hook event on an agent. @param {string} agentId @param {string} event @param {object} payload */
  async fireHook(agentId, event, payload = {}) {
    return this.#request(`/agents/${agentId}/fire_hook`, {
      method: "POST",
      body: JSON.stringify({ event, payload }),
    });
  }

  // ── Sessions ────────────────────────────────────────────

  /** List recent sessions for an agent. @param {string} agentId @param {{ limit?: number }} opts */
  async listSessions(agentId, opts = {}) {
    const params = opts.limit ? `?limit=${opts.limit}` : "";
    return this.#request(`/agents/${agentId}/sessions${params}`);
  }

  /** Get a single session. @param {string} agentId @param {string} sessionId */
  async getSession(agentId, sessionId) {
    return this.#request(`/agents/${agentId}/sessions/${sessionId}`);
  }

  /**
   * Fire a hook and wait for the session to complete.
   * @param {string} agentId
   * @param {string} event
   * @param {object} payload
   * @param {{ timeoutMs?: number, pollIntervalMs?: number }} opts
   * @returns {Promise<object>} The completed session
   */
  async fireHookAndWait(agentId, event, payload = {}, opts = {}) {
    const { timeoutMs = 120_000, pollIntervalMs = 2_000 } = opts;
    const session = await this.fireHook(agentId, event, payload);
    const sessionId = session.id;
    const deadline = Date.now() + timeoutMs;

    while (Date.now() < deadline) {
      await sleep(pollIntervalMs);
      const updated = await this.getSession(agentId, sessionId);
      if (updated.status === "completed") return updated;
      if (updated.status === "failed") {
        throw new Error(`Session failed: ${updated.error || "unknown error"}`);
      }
    }
    throw new Error(`Session ${sessionId} timed out after ${timeoutMs}ms`);
  }

  // ── Integrations ────────────────────────────────────────

  /** List integrations for an agent. @param {string} agentId */
  async listIntegrations(agentId) {
    return this.#request(`/agents/${agentId}/integrations`);
  }

  /** Create an integration. @param {string} agentId @param {string} platform */
  async createIntegration(agentId, platform) {
    return this.#request(`/agents/${agentId}/integrations`, {
      method: "POST",
      body: JSON.stringify({ platform }),
    });
  }

  /** Delete an integration. @param {string} agentId @param {string} integrationId */
  async deleteIntegration(agentId, integrationId) {
    return this.#request(`/agents/${agentId}/integrations/${integrationId}`, {
      method: "DELETE",
    });
  }

  // ── Tasks ───────────────────────────────────────────────

  /** List tasks for an agent. @param {string} agentId */
  async listTasks(agentId) {
    return this.#request(`/agents/${agentId}/tasks`);
  }

  /** Create a task. @param {string} agentId @param {{ name: string, hook_event?: string, schedule?: string }} data */
  async createTask(agentId, data) {
    return this.#request(`/agents/${agentId}/tasks`, {
      method: "POST",
      body: JSON.stringify(data),
    });
  }

  // ── Memories ────────────────────────────────────────────

  /** List agent memories. @param {string} agentId @param {{ limit?: number, offset?: number }} opts */
  async listMemories(agentId, opts = {}) {
    const params = new URLSearchParams();
    if (opts.limit) params.set("limit", String(opts.limit));
    if (opts.offset) params.set("offset", String(opts.offset));
    const qs = params.toString();
    return this.#request(`/agents/${agentId}/memories${qs ? "?" + qs : ""}`);
  }

  // ── Subscriptions ───────────────────────────────────────

  /** Get subscription and credits status. */
  async getSubscriptionStatus() {
    return this.#request("/subscriptions/status");
  }

  // ── Recommended Packs ───────────────────────────────────

  /** List recommended agent packs from Axoniac. */
  async listRecommendedPacks() {
    return this.#request("/recommended-packs");
  }
}

/** @param {number} ms */
function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
