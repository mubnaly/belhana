//! AI assistant chat (tool loop) and proposal approval endpoints.
//!
//! Guarantees:
//! - The assistant cannot write to the DB directly; mutating tool calls become
//!   `ai_action_proposals` rows pending user approval.
//! - Without a configured provider, the endpoint reports manual mode instead
//!   of failing the app.

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Row;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_ai::tools;
use belhana_common::CommonError;

#[derive(Debug, Deserialize)]
pub struct ChatMessageIn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessageIn>,
    /// Optional client-side conversation handle; persistence is planned.
    #[allow(dead_code)]
    pub conversation_id: Option<String>,
}

#[derive(Deserialize)]
pub struct ListProposalsQuery {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResolveProposalRequest {
    pub decision: String, // approve | reject
}

/// Runs one chat turn: provider loop with tool execution (max 5 hops).
async fn chat(State(state): State<SharedState>, user: AuthUser, Json(req): Json<ChatRequest>) -> ApiResult<Json<serde_json::Value>> {
    if req.messages.is_empty() {
        return Err(ApiError::from(CommonError::Validation("الرسالة فارغة".into())));
    }
    let Some(provider) = belhana_ai::provider_from_env() else {
        return Ok(Json(json!({
            "reply": belhana_ai::NO_PROVIDER_MESSAGE,
            "assistant_enabled": false,
            "tool_calls": [],
        })));
    };

    // Household free-text instruction rules are injected server-side into the
    // system context. They are guidance only — the AI still cannot write data
    // except through proposals that a human must approve.
    let instructions = belhana_core::rules::ai_instruction_sentences(&state.pool, &user.household_id).await?;
    let system_prompt = if instructions.is_empty() {
        belhana_ai::SYSTEM_PROMPT.to_string()
    } else {
        format!(
            "{}\n\n## تعليمات خاصة بهذه الأسرة (التزم بها قدر الإمكان):\n{}",
            belhana_ai::SYSTEM_PROMPT,
            instructions
                .iter()
                .map(|s| format!("- {s}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    let schemas = tools::tool_schemas();
    let mut msgs: Vec<belhana_ai::ChatMessage> = req
        .messages
        .iter()
        .map(|m| belhana_ai::ChatMessage {
            role: m.role.clone(),
            content: m.content.clone(),
            tool_name: None,
            tool_payload: None,
        })
        .collect();

    let mut all_tool_calls: Vec<serde_json::Value> = Vec::new();
    let mut reply = String::new();
    for _hop in 0..5 {
        let result = provider
            .chat(&system_prompt, &msgs, &schemas)
            .await
            .map_err(|e| ApiError::from(e))?;
        reply = result.reply.clone();
        if result.tool_calls.is_empty() {
            break;
        }
        msgs.push(belhana_ai::ChatMessage {
            role: "assistant".into(),
            content: reply.clone(),
            tool_name: None,
            tool_payload: None,
        });
        for call in &result.tool_calls {
            all_tool_calls.push(json!({"name": call.name, "args": call.args}));
            let payload = match tools::execute_tool(
                &state.pool,
                &user.household_id,
                &user.user_id,
                &call.name,
                &call.args,
            )
            .await
            {
                Ok(v) => v,
                Err(e) => json!({"error": e.to_string()}),
            };
            msgs.push(belhana_ai::ChatMessage {
                role: "tool".into(),
                content: String::new(),
                tool_name: Some(call.name.clone()),
                tool_payload: Some(payload),
            });
        }
    }

    Ok(Json(json!({
        "reply": reply,
        "assistant_enabled": true,
        "tool_calls": all_tool_calls,
    })))
}

async fn proposals(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<ListProposalsQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "SELECT id, action_type, payload, summary, status, created_at, resolved_at
         FROM ai_action_proposals WHERE household_id = ? AND (? IS NULL OR status = ?)
         ORDER BY created_at DESC LIMIT 100",
    )
    .bind(&user.household_id)
    .bind(q.status.as_deref())
    .bind(q.status.as_deref())
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    let mut out: Vec<Value> = Vec::new();
    for r in rows {
        let payload: Value =
            serde_json::from_str(&r.get::<String, _>("payload")).unwrap_or(Value::Null);
        out.push(json!({
            "id": r.get::<String, _>("id"),
            "action_type": r.get::<String, _>("action_type"),
            "payload": payload,
            "summary": r.get::<Option<String>, _>("summary"),
            "status": r.get::<String, _>("status"),
            "created_at": r.get::<String, _>("created_at"),
            "resolved_at": r.get::<Option<String>, _>("resolved_at"),
        }));
    }
    Ok(Json(json!(out)))
}

async fn resolve_proposal(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<ResolveProposalRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let row: Option<(String, String, String)> = sqlx::query_as(
        "SELECT action_type, payload, status FROM ai_action_proposals WHERE id = ? AND household_id = ?",
    )
    .bind(&id)
    .bind(&user.household_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    let Some((action_type, payload_text, status)) = row else {
        return Err(ApiError::from(CommonError::NotFound("proposal".into())));
    };
    if status != "pending" {
        return Err(ApiError::from(CommonError::Validation("تم الفصل في هذا الاقتراح بالفعل".into())));
    }
    let now = belhana_core::now_rfc3339();
    match req.decision.as_str() {
        "reject" => {
            sqlx::query("UPDATE ai_action_proposals SET status='rejected', resolved_at=? WHERE id=?")
                .bind(&now)
                .bind(&id)
                .execute(&state.pool)
                .await
                .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
            belhana_core::audit::log(
                &state.pool,
                Some(&user.household_id),
                Some(&user.user_id),
                "reject",
                "ai_proposal",
                &id,
                None,
                Some(json!({"action_type": action_type})),
            )
            .await?;
        }
        "approve" => {
            let payload: Value = serde_json::from_str(&payload_text)
                .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
            let result = apply_proposal(&state.pool, &user.household_id, &user.user_id, &action_type, &payload).await;
            let result_text = match &result {
                Ok(v) => v.to_string(),
                Err(e) => json!({"error": e.to_string()}).to_string(),
            };
            sqlx::query(
                "UPDATE ai_action_proposals SET status='applied', resolved_at=?, result=? WHERE id=?",
            )
            .bind(&now)
            .bind(&result_text)
            .bind(&id)
            .execute(&state.pool)
            .await
            .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
            result?;
        }
        _ => {
            return Err(ApiError::from(CommonError::Validation(
                "decision يجب أن يكون approve أو reject".into(),
            )));
        }
    }
    let status_now: String = sqlx::query_scalar("SELECT status FROM ai_action_proposals WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    Ok(Json(json!({"id": id, "status": status_now})))
}

/// Applies an approved proposal using the real domain core (never the LLM).
async fn apply_proposal(
    pool: &sqlx::SqlitePool,
    household_id: &str,
    user_id: &str,
    action_type: &str,
    payload: &Value,
) -> Result<serde_json::Value, CommonError> {
    match action_type {
        "add_rule" => {
            let input = belhana_core::rules::CreateRule {
                rule_type: payload["rule_type"].as_str().unwrap_or_default().into(),
                // `must` and `hard` are accepted from older proposals; both map
                // onto the rule domain's valid `hard` priority.
                priority: match payload["priority"].as_str() {
                    Some("hard") | Some("must") => "hard",
                    _ => "soft",
                }
                .into(),
                params: payload["params"].clone(),
                notes: payload["notes"].as_str().map(|s| s.to_string()),
            };
            let r = belhana_core::rules::create_rule(pool, household_id, &input).await?;
            Ok(json!({"rule_id": r.id}))
        }
        "set_budget" => {
            let field = payload["field"].as_str().unwrap_or("weekly_budget_minor");
            let amount = payload["amount_minor"].as_i64().unwrap_or(0);
            let hh = belhana_core::households::get_household(pool, household_id).await?;
            let mut u = belhana_core::households::UpdateHousehold {
                name: None,
                country: None,
                city: None,
                area: None,
                currency: None,
                household_size: None,
                shopping_frequency: None,
                daily_budget_minor: None,
                weekly_budget_minor: None,
                monthly_budget_minor: None,
                occasion_budget_minor: None,
                snacks_budget_minor: None,
                contingency_minor: None,
                preferred_merchants: None,
                kitchen_equipment: None,
                notes: None,
                price_policy: None,
                consumption_workflow: None,
            };
            match field {
                "daily_budget_minor" => u.daily_budget_minor = Some(amount),
                "monthly_budget_minor" => u.monthly_budget_minor = Some(amount),
                "weekly_budget_minor" => u.weekly_budget_minor = Some(amount),
                _ => u.weekly_budget_minor = Some(amount),
            }
            let _ = hh;
            let hh2 = belhana_core::households::update_household(pool, household_id, user_id, &u).await?;
            Ok(json!({"household": hh2}))
        }
        "add_preference" => {
            let member_id = payload["member_id"].as_str().unwrap_or_default();
            let input = belhana_core::members::CreatePreference {
                kind: payload["kind"].as_str().unwrap_or("like").into(),
                ref_type: payload["ref_type"].as_str().unwrap_or("product").into(),
                ref_id: payload["ref_id"].as_str().map(|s| s.to_string()),
                text_value: payload["text_value"].as_str().map(|s| s.to_string()),
                strength: payload["strength"].as_str().map(|s| s.to_string()),
                raw_text: payload["raw_text"].as_str().map(|s| s.to_string()),
            };
            let p = belhana_core::members::create_preference(pool, member_id, &input).await?;
            Ok(json!({"preference": p}))
        }
        other => Err(CommonError::Validation(format!(
            "نوع الاقتراح غير مدعوم للتطبيق التلقائي: {other}"
        ))),
    }
}

async fn assistant_status(State(_state): State<SharedState>) -> ApiResult<Json<serde_json::Value>> {
    let enabled = belhana_ai::provider_from_env().is_some();
    Ok(Json(json!({"assistant_enabled": enabled})))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/ai/chat", post(chat))
        .route("/api/ai/status", get(assistant_status))
        .route("/api/ai/proposals", get(proposals))
        .route("/api/ai/proposals/{id}/resolve", post(resolve_proposal))
}
