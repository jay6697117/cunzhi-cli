/// MCP 通用工具函数模块
///
/// 包含 MCP 相关的通用工具函数和辅助方法

use anyhow::Result;
use std::path::Path;

/// 验证项目路径是否存在
pub fn validate_project_path(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        anyhow::bail!("项目路径不存在: {}", path);
    }
    Ok(())
}

/// 生成唯一的请求 ID
pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}




