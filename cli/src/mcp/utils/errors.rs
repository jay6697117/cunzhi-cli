/// MCP 错误处理工具模块
///
/// 提供统一的错误处理和转换功能

use crate::mcp::types::McpError;

/// MCP 错误类型枚举
#[derive(Debug, thiserror::Error)]
pub enum McpToolError {
    #[error("项目路径错误: {0}")]
    ProjectPath(String),

    #[error("弹窗创建失败: {0}")]
    PopupCreation(String),

    #[error("响应解析失败: {0}")]
    ResponseParsing(String),

    #[error("配置错误: {0}")]
    Configuration(String),

    #[error("内部错误: {0}")]
    Internal(String),
}

/// 创建项目路径错误
pub fn project_path_error(message: String) -> McpError {
    McpError::invalid_params(format!("项目路径错误: {}", message), None)
}

/// 创建弹窗错误
pub fn popup_error(message: String) -> McpError {
    McpError::internal_error(format!("弹窗创建失败: {}", message), None)
}

/// 创建响应解析错误
pub fn response_parsing_error(message: String) -> McpError {
    McpError::internal_error(format!("响应解析失败: {}", message), None)
}
