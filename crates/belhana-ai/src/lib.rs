//! AI provider abstraction for Belhana.
//!
//! Rules enforced here:
//! - Provider API keys live on the server only (env vars); never in clients.
//! - The LLM never writes to the database directly; it may only call
//!   validated domain tools, and mutating actions become *proposals* that
//!   require user confirmation.
//! - The AI must not invent prices, inventory, or nutrition facts; tools
//!   return real data or explicitly report missing data.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod tools;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String, // openai_compatible | gemini
    pub model: String,
    pub base_url: String,
    pub api_key_env: String,
}

impl ProviderConfig {
    pub fn from_env() -> Option<ProviderConfig> {
        // Gemini first, then OpenAI-compatible.
        if std::env::var("BELHANA_GEMINI_API_KEY").map(|v| !v.is_empty()).unwrap_or(false) {
            return Some(ProviderConfig {
                provider: "gemini".into(),
                model: std::env::var("BELHANA_GEMINI_MODEL").unwrap_or_else(|_| "gemini-2.0-flash".into()),
                base_url: "https://generativelanguage.googleapis.com/v1beta".into(),
                api_key_env: "BELHANA_GEMINI_API_KEY".into(),
            });
        }
        if std::env::var("BELHANA_OPENAI_API_KEY").map(|v| !v.is_empty()).unwrap_or(false) {
            return Some(ProviderConfig {
                provider: "openai_compatible".into(),
                model: std::env::var("BELHANA_OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".into()),
                base_url: std::env::var("BELHANA_OPENAI_BASE_URL")
                    .unwrap_or_else(|_| "https://api.openai.com/v1".into()),
                api_key_env: "BELHANA_OPENAI_API_KEY".into(),
            });
        }
        None
    }

    fn api_key(&self) -> Option<String> {
        std::env::var(&self.api_key_env).ok().filter(|v| !v.is_empty())
    }
}

pub struct ChatMessage {
    pub role: String, // user | assistant | tool | system
    pub content: String,
    pub tool_name: Option<String>,
    pub tool_payload: Option<Value>,
}

pub struct ChatResult {
    pub reply: String,
    pub tool_calls: Vec<ToolCallRequest>,
}

#[derive(Debug, Serialize)]
pub struct ToolCallRequest {
    pub name: String,
    pub args: Value,
}

#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    async fn chat(
        &self,
        system_prompt: &str,
        messages: &[ChatMessage],
        tools_schema: &[Value],
    ) -> Result<ChatResult, CommonError>;
}

pub struct OpenAiCompatible {
    pub cfg: ProviderConfig,
    pub http: reqwest::Client,
}

#[async_trait::async_trait]
impl AiProvider for OpenAiCompatible {
    async fn chat(
        &self,
        system_prompt: &str,
        messages: &[ChatMessage],
        tools_schema: &[Value],
    ) -> Result<ChatResult, CommonError> {
        let key = self
            .cfg
            .api_key()
            .ok_or_else(|| CommonError::Internal("AI provider key missing".into()))?;
        let mut msgs: Vec<Value> = vec![serde_json::json!({"role": "system", "content": system_prompt})];
        for m in messages {
            match (&m.tool_name, &m.tool_payload) {
                (Some(_), Some(p)) => {
                    msgs.push(serde_json::json!({"role": "tool", "tool_call_id": format!("call_{}", m.tool_name.clone().unwrap_or_default()), "content": p.to_string()}));
                }
                _ => {
                    msgs.push(serde_json::json!({"role": m.role, "content": m.content}));
                }
            }
        }
        let mut body = serde_json::json!({"model": self.cfg.model, "messages": msgs});
        if !tools_schema.is_empty() {
            body["tools"] = Value::Array(
                tools_schema
                    .iter()
                    .map(|t| serde_json::json!({"type": "function", "function": t}))
                    .collect(),
            );
        }
        let url = format!("{}/chat/completions", self.cfg.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .post(url)
            .bearer_auth(key)
            .json(&body)
            .send()
            .await
            .map_err(|e| CommonError::Internal(format!("AI request failed: {e}")))?;
        let status = resp.status();
        let v: Value = resp.json().await.map_err(|e| CommonError::Internal(format!("AI response parse failed: {e}")))?;
        if !status.is_success() {
            return Err(CommonError::Internal(format!("AI provider error {status}: {v}")));
        }
        let choice = v["choices"][0].clone();
        let msg = &choice["message"];
        let reply = msg["content"].as_str().unwrap_or("").to_string();
        let mut tool_calls = Vec::new();
        if let Some(calls) = msg["tool_calls"].as_array() {
            for c in calls {
                tool_calls.push(ToolCallRequest {
                    name: c["function"]["name"].as_str().unwrap_or("").to_string(),
                    args: serde_json::from_str(c["function"]["arguments"].as_str().unwrap_or("{}"))
                        .unwrap_or(Value::Object(Default::default())),
                });
            }
        }
        Ok(ChatResult { reply, tool_calls })
    }
}

pub struct GeminiProvider {
    pub cfg: ProviderConfig,
    pub http: reqwest::Client,
}

#[async_trait::async_trait]
impl AiProvider for GeminiProvider {
    async fn chat(
        &self,
        system_prompt: &str,
        messages: &[ChatMessage],
        _tools_schema: &[Value],
    ) -> Result<ChatResult, CommonError> {
        let key = self
            .cfg
            .api_key()
            .ok_or_else(|| CommonError::Internal("AI provider key missing".into()))?;
        let mut contents: Vec<Value> = Vec::new();
        for m in messages {
            if m.role == "user" || m.role == "assistant" {
                contents.push(serde_json::json!({"role": if m.role == "assistant" { "model" } else { "user" }, "parts": [{"text": m.content}]}));
            }
        }
        let body = serde_json::json!({
            "system_instruction": {"parts": [{"text": system_prompt}]},
            "contents": contents,
        });
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.cfg.base_url.trim_end_matches('/'),
            self.cfg.model,
            key
        );
        let resp = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CommonError::Internal(format!("AI request failed: {e}")))?;
        let status = resp.status();
        let v: Value = resp.json().await.map_err(|e| CommonError::Internal(format!("AI response parse failed: {e}")))?;
        if !status.is_success() {
            return Err(CommonError::Internal(format!("AI provider error {status}: {v}")));
        }
        let reply = v["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(ChatResult { reply, tool_calls: vec![] })
    }
}

/// Builds the configured provider, or None when no key is set (manual mode).
pub fn provider_from_env() -> Option<Box<dyn AiProvider>> {
    let cfg = ProviderConfig::from_env()?;
    let http = reqwest::Client::new();
    match cfg.provider.as_str() {
        "gemini" => Some(Box::new(GeminiProvider { cfg, http })),
        _ => Some(Box::new(OpenAiCompatible { cfg, http })),
    }
}

pub const SYSTEM_PROMPT: &str = r#"أنت مساعد بالهنا (Belhana) لمساعدة الأسرة في تخطيط الطعام والميزانية.
قواعد صارمة:
- لا تخترع أسعارا أو أوزانا أو معلومات مخزون. استخدم الأدوات المتاحة لجلب البيانات، وإذا كانت البيانات غير متوفرة قل ذلك بوضوح.
- لا تنفذ أي عملية تغيير (قواعد، خطط، مخزون) مباشرة؛ اقترحها وانتظر موافقة المستخدم.
- الحساسيات والاستثناءات الغذائية قيود صارمة لا يمكن تجاوزها.
- كل الأسعار بالجنيه المصري (EGP) ما لم يذكر خلاف ذلك، واذكر تاريخ ملاحظة السعر ومصدره إن كان متوفرا.
- تحدث بالعربية المصرية الواضحة والودية.
"#;

pub const NO_PROVIDER_MESSAGE: &str = "المساعد الذكي غير مفعّل حالياً. لتشغيله، أضف مفتاح مزوّد الذكاء الاصطناعي في إعدادات الخادم (BELHANA_GEMINI_API_KEY أو BELHANA_OPENAI_API_KEY). كل ميزات التطبيق الأخرى تعمل بدون المساعد الذكي.";
