// 配置结构定义 - CLI 版本简化版
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_reply_config")]
    pub reply_config: ReplyConfig, // 继续回复配置
    #[serde(default = "default_mcp_config")]
    pub mcp_config: McpConfig, // MCP工具配置
    #[serde(default = "default_terminal_config")]
    pub terminal_config: TerminalConfig, // 终端启动器配置
    #[serde(default = "default_version")]
    pub version: String, // 配置版本
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplyConfig {
    #[serde(default = "default_enable_continue_reply")]
    pub enable_continue_reply: bool,
    #[serde(default = "default_auto_continue_threshold")]
    pub auto_continue_threshold: u32, // 字符数阈值
    #[serde(default = "default_continue_prompt")]
    pub continue_prompt: String, // 继续回复的提示词
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpConfig {
    #[serde(default = "default_mcp_tools")]
    pub tools: HashMap<String, bool>, // MCP工具启用状态
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramConfig {
    #[serde(default = "default_telegram_enabled")]
    pub enabled: bool, // 是否启用Telegram Bot
    #[serde(default = "default_telegram_bot_token")]
    pub bot_token: String, // Bot Token
    #[serde(default = "default_telegram_chat_id")]
    pub chat_id: String, // Chat ID
    #[serde(default = "default_telegram_hide_frontend_popup")]
    pub hide_frontend_popup: bool, // 是否隐藏前端弹窗，仅使用Telegram交互
    #[serde(default = "default_telegram_api_base_url")]
    pub api_base_url: String, // Telegram API基础URL
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TerminalConfig {
    #[serde(default = "default_terminal_enabled")]
    pub enabled: bool, // 是否启用终端模式
    #[serde(default = "default_preferred_terminal")]
    pub preferred_terminal: Option<String>, // 首选终端类型
    #[serde(default = "default_fallback_to_cli")]
    pub fallback_to_cli: bool, // 终端启动失败时是否回退到CLI
    #[serde(default = "default_terminal_window_title")]
    pub window_title: String, // 终端窗口标题
    #[serde(default = "default_terminal_timeout")]
    pub timeout_seconds: u32, // 等待用户响应的超时时间（秒）
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            reply_config: default_reply_config(),
            mcp_config: default_mcp_config(),
            terminal_config: default_terminal_config(),
            version: default_version(),
        }
    }
}

// 默认值函数
pub fn default_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn default_reply_config() -> ReplyConfig {
    ReplyConfig {
        enable_continue_reply: default_enable_continue_reply(),
        auto_continue_threshold: default_auto_continue_threshold(),
        continue_prompt: default_continue_prompt(),
    }
}

pub fn default_mcp_config() -> McpConfig {
    McpConfig {
        tools: default_mcp_tools(),
    }
}

pub fn default_telegram_config() -> TelegramConfig {
    TelegramConfig {
        enabled: default_telegram_enabled(),
        bot_token: default_telegram_bot_token(),
        chat_id: default_telegram_chat_id(),
        hide_frontend_popup: default_telegram_hide_frontend_popup(),
        api_base_url: default_telegram_api_base_url(),
    }
}

pub fn default_terminal_config() -> TerminalConfig {
    TerminalConfig {
        enabled: default_terminal_enabled(),
        preferred_terminal: default_preferred_terminal(),
        fallback_to_cli: default_fallback_to_cli(),
        window_title: default_terminal_window_title(),
        timeout_seconds: default_terminal_timeout(),
    }
}

// MCP 相关默认值
pub fn default_enable_continue_reply() -> bool {
    true
}

pub fn default_auto_continue_threshold() -> u32 {
    1000
}

pub fn default_continue_prompt() -> String {
    "请按照最佳实践继续".to_string()
}

pub fn default_mcp_tools() -> HashMap<String, bool> {
    let mut tools = HashMap::new();
    tools.insert("zhi".to_string(), true); // 寸止工具默认启用
    tools.insert("ji".to_string(), true); // 记忆管理工具默认启用
    tools
}

// Telegram 相关默认值
pub fn default_telegram_enabled() -> bool {
    false
}

pub fn default_telegram_bot_token() -> String {
    "".to_string()
}

pub fn default_telegram_chat_id() -> String {
    "".to_string()
}

pub fn default_telegram_hide_frontend_popup() -> bool {
    false
}

pub fn default_telegram_api_base_url() -> String {
    "https://api.telegram.org/bot".to_string()
}

// 终端相关默认值
pub fn default_terminal_enabled() -> bool {
    true
}

pub fn default_preferred_terminal() -> Option<String> {
    None // 自动检测
}

pub fn default_fallback_to_cli() -> bool {
    true
}

pub fn default_terminal_window_title() -> String {
    "寸止 CLI 交互".to_string()
}

pub fn default_terminal_timeout() -> u32 {
    300 // 5分钟
}

// 配置验证和工具函数
impl AppConfig {
    /// 验证配置是否有效
    pub fn validate(&self) -> anyhow::Result<()> {
        // 验证继续回复配置
        if self.reply_config.auto_continue_threshold == 0 {
            return Err(anyhow::anyhow!("自动继续阈值不能为 0"));
        }

        Ok(())
    }

    /// 获取配置摘要
    pub fn get_summary(&self) -> String {
        let mut summary = Vec::new();

        summary.push(format!("配置版本: {}", self.version));

        // MCP 工具状态
        let enabled_tools: Vec<String> = self.mcp_config.tools
            .iter()
            .filter(|(_, &enabled)| enabled)
            .map(|(name, _)| name.clone())
            .collect();
        summary.push(format!("启用的 MCP 工具: {}", enabled_tools.join(", ")));

        // 继续回复状态
        if self.reply_config.enable_continue_reply {
            summary.push(format!("自动继续回复: 已启用 (阈值: {} 字符)", self.reply_config.auto_continue_threshold));
        } else {
            summary.push("自动继续回复: 已禁用".to_string());
        }

        summary.join("\n")
    }
}

impl TelegramConfig {
    /// 验证 Telegram 配置是否有效
    pub fn is_valid(&self) -> bool {
        if !self.enabled {
            return true; // 如果未启用，则认为有效
        }

        !self.bot_token.is_empty()
            && !self.chat_id.is_empty()
            && self.bot_token.contains(':')
    }

    /// 获取 API URL
    pub fn get_api_url(&self, method: &str) -> String {
        format!("{}{}/{}", self.api_base_url, self.bot_token, method)
    }
}

impl McpConfig {
    /// 检查工具是否启用
    pub fn is_tool_enabled(&self, tool_name: &str) -> bool {
        self.tools.get(tool_name).copied().unwrap_or(false)
    }

    /// 设置工具启用状态
    pub fn set_tool_enabled(&mut self, tool_name: &str, enabled: bool) {
        self.tools.insert(tool_name.to_string(), enabled);
    }

    /// 获取启用的工具列表
    pub fn get_enabled_tools(&self) -> Vec<String> {
        self.tools
            .iter()
            .filter(|(_, &enabled)| enabled)
            .map(|(name, _)| name.clone())
            .collect()
    }
}
