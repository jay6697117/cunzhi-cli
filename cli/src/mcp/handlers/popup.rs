// 弹窗处理模块
// 在 CLI 版本中，这些函数将被重新实现为命令行交互

use anyhow::Result;
use crate::mcp::PopupRequest;

/// 创建弹窗（CLI 版本占位实现）
pub fn create_tauri_popup(request: &PopupRequest) -> Result<String> {
    // 在 CLI 版本中，这将被实现为命令行交互
    // 目前返回占位响应
    Ok(format!(
        "CLI 占位响应 - 消息: {}, 选项: {:?}", 
        request.message, 
        request.predefined_options
    ))
}
