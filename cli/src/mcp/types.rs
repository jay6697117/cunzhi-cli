use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ZhiRequest {
    /// 要显示给用户的消息
    pub message: String,
    /// 预定义的选项列表（可选）
    #[serde(default)]
    pub predefined_options: Vec<String>,
    /// 消息是否为Markdown格式，默认为true
    #[serde(default = "default_is_markdown")]
    pub is_markdown: bool,
    /// 是否启用终端模式，默认为false
    #[serde(default = "default_terminal_mode")]
    pub terminal_mode: Option<bool>,
}

fn default_is_markdown() -> bool {
    true
}

fn default_terminal_mode() -> Option<bool> {
    Some(false)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JiyiRequest {
    /// 操作类型：记忆(添加记忆), 回忆(获取项目信息)
    pub action: String,
    /// 项目路径（必需）
    pub project_path: String,
    /// 记忆内容（记忆操作时必需）
    #[serde(default)]
    pub content: String,
    /// 记忆分类：rule(规范规则), preference(用户偏好), pattern(最佳实践), context(项目上下文)
    #[serde(default = "default_category")]
    pub category: String,
}

fn default_category() -> String {
    "context".to_string()
}

// 简化的 MCP 类型定义（替代 rmcp）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
}

impl Content {
    pub fn text(text: String) -> Self {
        Self {
            content_type: "text".to_string(),
            text,
            annotations: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallToolResult {
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl CallToolResult {
    pub fn success(content: Vec<Content>) -> Self {
        Self {
            content,
            is_error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            content: vec![Content::text(message)],
            is_error: Some(true),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Method not found: {0}")]
    MethodNotFound(String),
}

impl McpError {
    pub fn invalid_params(msg: String, _data: Option<serde_json::Value>) -> Self {
        Self::InvalidParams(msg)
    }

    pub fn internal_error(msg: String, _data: Option<serde_json::Value>) -> Self {
        Self::InternalError(msg)
    }

    pub fn invalid_request(msg: String, _data: Option<serde_json::Value>) -> Self {
        Self::MethodNotFound(msg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PopupRequest {
    pub id: String,
    pub message: String,
    pub predefined_options: Option<Vec<String>>,
    pub is_markdown: bool,
}

/// 新的结构化响应数据格式
#[derive(Debug, Deserialize)]
pub struct McpResponse {
    pub user_input: Option<String>,
    pub selected_options: Vec<String>,
    pub images: Vec<ImageAttachment>,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAttachment {
    pub data: String,
    pub media_type: String,
    pub filename: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetadata {
    pub timestamp: Option<String>,
    pub request_id: Option<String>,
    pub source: Option<String>,
}

/// 旧格式兼容性支持
#[derive(Debug, Deserialize)]
pub struct McpResponseContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub source: Option<ImageSource>,
}

#[derive(Debug, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

/// 统一的响应构建函数
///
/// 用于生成标准的JSON响应格式，确保无GUI和有GUI模式输出一致
pub fn build_mcp_response(
    user_input: Option<String>,
    selected_options: Vec<String>,
    images: Vec<ImageAttachment>,
    request_id: Option<String>,
    source: &str,
) -> serde_json::Value {
    serde_json::json!({
        "user_input": user_input,
        "selected_options": selected_options,
        "images": images,
        "metadata": {
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "request_id": request_id,
            "source": source
        }
    })
}

/// 构建发送操作的响应
pub fn build_send_response(
    user_input: Option<String>,
    selected_options: Vec<String>,
    images: Vec<ImageAttachment>,
    request_id: Option<String>,
    source: &str,
) -> String {
    let response = build_mcp_response(user_input, selected_options, images, request_id, source);
    response.to_string()
}

/// 构建继续操作的响应
pub fn build_continue_response(request_id: Option<String>, source: &str) -> String {
    // 动态获取继续提示词
    let continue_prompt = if let Ok(config) = crate::config::load_standalone_config() {
        config.reply_config.continue_prompt
    } else {
        "请按照最佳实践继续".to_string()
    };

    let response = build_mcp_response(Some(continue_prompt), vec![], vec![], request_id, source);
    response.to_string()
}
