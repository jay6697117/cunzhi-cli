use anyhow::Result;
use crate::mcp::types::{McpError, CallToolResult, Content};
use super::{MemoryManager, MemoryCategory};
use crate::mcp::JiyiRequest;

/// 全局记忆管理工具
///
/// 用于存储和管理重要的开发规范、用户偏好和最佳实践
#[derive(Clone)]
pub struct MemoryTool;

impl MemoryTool {
    pub async fn jiyi(
        request: JiyiRequest,
    ) -> Result<CallToolResult, McpError> {
        // 检查项目路径是否存在
        if !std::path::Path::new(&request.project_path).exists() {
            return Err(McpError::invalid_params(
                format!("项目路径不存在: {}", request.project_path), None
            ));
        }

        let manager = MemoryManager::new(&request.project_path)
            .map_err(|e| McpError::internal_error(format!("创建记忆管理器失败: {}", e), None))?;

        let result = match request.action.as_str() {
            "记忆" => {
                if request.content.trim().is_empty() {
                    return Err(McpError::invalid_params("缺少记忆内容".to_string(), None));
                }

                let category = match request.category.as_str() {
                    "rule" => MemoryCategory::Rule,
                    "preference" => MemoryCategory::Preference,
                    "pattern" => MemoryCategory::Pattern,
                    "context" => MemoryCategory::Context,
                    _ => MemoryCategory::Context,
                };

                let id = manager.add_memory(&request.content, category)
                    .map_err(|e| McpError::internal_error(format!("添加记忆失败: {}", e), None))?;

                format!("✅ 记忆已添加，ID: {}\n📝 内容: {}\n📂 分类: {:?}", id, request.content, category)
            }
            "回忆" => {
                manager.get_project_summary()
                    .map_err(|e| McpError::internal_error(format!("获取项目信息失败: {}", e), None))?
            }
            _ => {
                return Err(McpError::invalid_params(
                    format!("未知的操作类型: {}，支持的操作: 记忆, 回忆", request.action), None
                ));
            }
        };

        // 构建响应内容
        let content = vec![Content::text(result)];
        Ok(CallToolResult::success(content))
    }
}
