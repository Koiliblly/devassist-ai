//! AI 客户端模块
//!
//! 覆盖 Rust 核心考点：
//! - 结构体（`AiClient`）与关联方法
//! - 泛型 + Trait 约束（`complete(&dyn PromptTemplate)` 动态分发）
//! - 错误处理（`Result<T, String>` + `map_err` + `?` 传播）
//! - 与外部 HTTP API 交互（reqwest + serde_json）

use crate::template::PromptTemplate;
use serde::{Deserialize, Serialize};

/// AI 服务设置（兼容 OpenAI API 规范，可接入任意兼容服务）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSettings {
    pub api_base: String,
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
}

impl Default for AiSettings {
    fn default() -> Self {
        Self {
            api_base: "https://api.openai.com/v1".into(),
            api_key: String::new(),
            model: "gpt-4o-mini".into(),
            temperature: 0.7,
        }
    }
}

/// 发送给模型的一条对话消息
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// AI 客户端：统一封装对 OpenAI 兼容接口的 HTTP 请求
///
/// 实现 Clone（reqwest::Client 内部为 Arc），便于在异步命令中
/// 先克隆再释放锁，避免跨 await 持有锁导致的 Send 问题。
#[derive(Clone)]
pub struct AiClient {
    http: reqwest::Client,
    settings: AiSettings,
}

impl AiClient {
    /// 根据设置构建客户端，配置统一的超时时间
    pub fn new(settings: AiSettings) -> Result<Self, String> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| format!("初始化 HTTP 客户端失败: {e}"))?;
        Ok(Self { http, settings })
    }

    /// 应用新的设置
    pub fn update(&mut self, settings: AiSettings) {
        self.settings = settings;
    }

    pub fn settings(&self) -> &AiSettings {
        &self.settings
    }

    /// 使用提示词模板完成一次 AI 生成：
    /// 通过 Trait 对象接收任意实现 `PromptTemplate` 的模板，实现与具体模板解耦
    pub async fn complete(
        &self,
        template: &dyn PromptTemplate,
        input: &str,
    ) -> Result<String, String> {
        let messages = vec![
            ChatMessage {
                role: "system".into(),
                content: template.system_prompt().to_string(),
            },
            ChatMessage {
                role: "user".into(),
                content: template.render(input),
            },
        ];
        self.chat(messages).await
    }

    /// 调用 OpenAI 兼容的 `/chat/completions` 接口
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, String> {
        if self.settings.api_key.trim().is_empty() {
            return Err("未配置 API Key，请先在「模型设置」中填写".into());
        }

        let url = format!(
            "{}/chat/completions",
            self.settings.api_base.trim_end_matches('/')
        );

        let body = serde_json::json!({
            "model": self.settings.model,
            "messages": messages,
            "temperature": self.settings.temperature,
        });

        let resp = self
            .http
            .post(&url)
            .bearer_auth(self.settings.api_key.trim())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求模型接口失败: {e}"))?;

        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| format!("读取模型响应失败: {e}"))?;

        if !status.is_success() {
            return Err(format!(
                "模型接口返回错误 (HTTP {status}): {}",
                truncate(&text, 300)
            ));
        }

        let json: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("解析模型响应失败: {e}"))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "模型响应中缺少 choices[0].message.content 字段".to_string())
    }
}

/// 截断过长的错误信息，避免刷屏
fn truncate(s: &str, max: usize) -> String {
    let count = s.chars().count();
    if count <= max {
        s.to_string()
    } else {
        let head: String = s.chars().take(max).collect();
        format!("{head}…")
    }
}
