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

## Instruction rules
A household `ai_instruction` rule holds a free-text sentence that is injected **only into the system
prompt** so the assistant "thinks for it" — e.g. "use leftovers first, make rice the usual lunch, bread
the basis of breakfast/dinner, and buy spices by a fixed amount (20 EGP of salt) rather than by the
kilo." All six seeded defaults are loaded on each chat turn. Instruction rules are guidance, not hard
constraints: they never grant write access and never bypass the proposal workflow.

## Hard constraints
- AI never invents prices, inventory, nutrition facts or dates.
- AI cannot directly modify plans, prices, purchases or household settings; every mutating tool still
  creates a pending `ai_action_proposals` row for human approval.
- System prompt requires Arabic by default, embeds the household's instruction sentences and explicitly
  labels uncertainty.
- All provider traffic is server-side; API keys are never sent to the browser.
