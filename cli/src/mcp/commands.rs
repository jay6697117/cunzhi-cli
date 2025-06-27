// MCP 命令处理模块
// 这个模块在 CLI 版本中主要用于配置管理

// HashMap 导入已移除

/// MCP工具配置
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MCPToolConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub can_disable: bool,
    pub icon: String,
    pub icon_bg: String,
    pub dark_icon_bg: String,
}

/// 获取默认的 MCP 工具配置
pub fn get_default_mcp_tools_config() -> Vec<MCPToolConfig> {
    vec![
        MCPToolConfig {
            id: "zhi".to_string(),
            name: "寸止工具".to_string(),
            description: "智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传".to_string(),
            enabled: true,
            can_disable: false, // 核心工具不可禁用
            icon: "🤖".to_string(),
            icon_bg: "#3b82f6".to_string(),
            dark_icon_bg: "#1e40af".to_string(),
        },
        MCPToolConfig {
            id: "ji".to_string(),
            name: "记忆管理工具".to_string(),
            description: "全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践".to_string(),
            enabled: true,
            can_disable: true,
            icon: "🧠".to_string(),
            icon_bg: "#10b981".to_string(),
            dark_icon_bg: "#047857".to_string(),
        },
    ]
}
