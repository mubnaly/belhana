# 12 — AI assistant

## Provider abstraction
`belhana-ai::provider_from_env()` selects Gemini (`BELHANA_GEMINI_API_KEY` / `_MODEL`) or an
OpenAI-compatible endpoint (`BELHANA_OPENAI_API_KEY`, `_MODEL`, `_BASE_URL`). With no key, the API
answers in Arabic manual mode (`assistant_enabled: false`) and the rest of the app works normally.
No live provider call was exercised in this environment.

## Tool loop
`POST /api/ai/chat` runs up to five provider hops with structured tool schemas from
`belhana_ai::tools::tool_schemas()`. Read-only tools (e.g. price lookup, budget status, recipe/inventory
queries) execute deterministically. **Mutating tools never write domain data**: they insert
`ai_action_proposals` rows (`status = pending`) with a human-readable summary and payload.

## Approval
`GET /api/ai/proposals?status=pending` lists proposals;
`POST /api/ai/proposals/{id}/resolve {decision: approve|reject}` records the user decision.
Approval endpoints exist for the API contract; the web screen shows conversation and manual guidance,
with proposal resolution available via API.

## Hard constraints
- AI never invents prices, inventory, nutrition facts or dates.
- AI cannot directly modify plans, prices, purchases or household settings.
- System prompt requires Arabic by default and explicitly labels uncertainty.
- All provider traffic is server-side; API keys are never sent to the browser.
