// 响应处理模块

use anyhow::Result;
use crate::mcp::types::Content;

/// 解析 MCP 响应（CLI 版本简化实现）
pub fn parse_mcp_response(response: &str) -> Result<Vec<Content>> {
    // 简化的响应解析，返回文本内容
    Ok(vec![Content::text(response.to_string())])
}
