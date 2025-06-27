// MCP 客户端配置生成器
use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;
use crate::{log_important, log_debug};

/// MCP 客户端配置
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McpClientConfig {
    pub mcpServers: std::collections::HashMap<String, McpServerConfig>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McpServerConfig {
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
}

impl McpClientConfig {
    /// 创建默认的 cunzhi-cli MCP 配置
    pub fn new_cunzhi_config() -> Self {
        let mut servers = std::collections::HashMap::new();

        // 获取当前可执行文件路径
        let current_exe = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("cunzhi"));

        let exe_path = current_exe.to_string_lossy().to_string();

        servers.insert("cunzhi-cli".to_string(), McpServerConfig {
            command: exe_path,
            args: vec!["mcp-server".to_string()],
            env: Some({
                let mut env = std::collections::HashMap::new();
                env.insert("RUST_LOG".to_string(), "info".to_string());
                env
            }),
        });

        Self { mcpServers: servers }
    }

    /// 保存配置到文件
    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        log_important!(info, "MCP 客户端配置已保存到: {}", path.display());
        Ok(())
    }

    /// 从文件加载配置
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }
}

/// 生成 MCP 客户端配置文件
pub fn generate_mcp_config() -> Result<()> {
    let config = McpClientConfig::new_cunzhi_config();

    // 常见的 MCP 配置文件位置
    let config_paths = vec![
        // Claude Desktop 配置
        dirs::config_dir().map(|d| d.join("Claude").join("claude_desktop_config.json")),
        // Augment 配置
        dirs::config_dir().map(|d| d.join("augment").join("mcp_config.json")),
        // 通用 MCP 配置
        dirs::config_dir().map(|d| d.join("mcp").join("config.json")),
        // 当前目录
        Some(PathBuf::from("mcp_config.json")),
    ];

    for path_opt in config_paths {
        if let Some(path) = path_opt {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            // 如果文件已存在，尝试合并配置
            if path.exists() {
                match merge_config(&config, &path) {
                    Ok(_) => log_important!(info, "已更新现有配置: {}", path.display()),
                    Err(e) => {
                        log_debug!("合并配置失败，创建新配置: {}", e);
                        config.save_to_file(&path)?;
                    }
                }
            } else {
                config.save_to_file(&path)?;
            }
        }
    }

    println!("\n📋 MCP 配置文件已生成，包含以下工具:");
    println!("  🤖 zhi_cunzhi - 智能代码审查工具");
    println!("  🧠 ji_cunzhi - 记忆管理工具");
    println!("\n💡 使用方法:");
    println!("  1. 重启您的 MCP 客户端 (如 Claude Desktop, Augment)");
    println!("  2. 工具应该会自动出现在可用工具列表中");
    println!("  3. 如果没有出现，请检查客户端的 MCP 配置");

    Ok(())
}

/// 合并配置到现有文件
fn merge_config(new_config: &McpClientConfig, path: &PathBuf) -> Result<()> {
    let existing_content = std::fs::read_to_string(path)?;
    let mut existing_value: Value = serde_json::from_str(&existing_content)?;

    // 确保 mcpServers 字段存在
    if !existing_value.is_object() {
        existing_value = serde_json::json!({});
    }

    if existing_value.get("mcpServers").is_none() {
        existing_value["mcpServers"] = serde_json::json!({});
    }

    // 添加或更新 cunzhi-cli 配置
    let new_server_config = serde_json::to_value(&new_config.mcpServers["cunzhi-cli"])?;
    existing_value["mcpServers"]["cunzhi-cli"] = new_server_config;

    // 保存合并后的配置
    let merged_json = serde_json::to_string_pretty(&existing_value)?;
    std::fs::write(path, merged_json)?;

    Ok(())
}

/// 验证 MCP 配置
pub fn validate_mcp_config() -> Result<()> {
    log_important!(info, "验证 MCP 配置...");

    // 检查可执行文件
    let current_exe = std::env::current_exe()?;
    if !current_exe.exists() {
        return Err(anyhow::anyhow!("可执行文件不存在: {}", current_exe.display()));
    }

    log_important!(info, "✅ 可执行文件: {}", current_exe.display());

    // 检查工具可用性
    let server = crate::mcp::ZhiServer::new();
    let tools = server.list_tools();

    log_important!(info, "✅ 可用工具: {} 个", tools.len());
    for tool in tools {
        log_important!(info, "  - {} ({})", tool.name, if tool.enabled { "启用" } else { "禁用" });
    }

    log_important!(info, "MCP 配置验证完成");
    Ok(())
}
