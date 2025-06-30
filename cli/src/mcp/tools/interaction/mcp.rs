use anyhow::Result;
use crate::mcp::types::{McpError, CallToolResult, Content, PopupRequest};
use crate::mcp::ZhiRequest;
use crate::mcp::handlers::popup::create_cli_popup;
use crate::utils::{colorize, colorize_with_style, colors, terminal_launcher::TerminalLauncher};
use inquire::{Select, Text, InquireError};
use console::style;
use std::path::PathBuf;
use std::env;

/// 增强的CLI交互处理器
pub struct EnhancedCliInteraction;

impl EnhancedCliInteraction {
    /// 处理选项选择
    pub fn handle_option_selection(options: &[String]) -> Result<String, McpError> {
        // 创建选项列表，包含自定义输入和取消选项
        let mut all_options = options.to_vec();
        all_options.push("自定义输入".to_string());
        all_options.push("取消".to_string());

        let selection = Select::new("请选择一个选项:", all_options)
            .with_help_message("使用 ↑↓ 箭头键导航，回车确认")
            .prompt()
            .map_err(|e| match e {
                InquireError::OperationCanceled => {
                    McpError::internal_error("用户取消了操作".to_string(), None)
                }
                _ => McpError::internal_error(format!("选择失败: {}", e), None)
            })?;

        if selection == "取消" {
            return Ok("用户取消了操作".to_string());
        } else if selection == "自定义输入" {
            return Self::handle_custom_input();
        } else {
            return Ok(format!("用户选择: {}", selection));
        }
    }

    /// 处理自定义输入
    pub fn handle_custom_input() -> Result<String, McpError> {
        let input = Text::new("请输入您的回复:")
            .with_help_message("输入内容后按回车确认，Ctrl+C 取消")
            .prompt()
            .map_err(|e| match e {
                InquireError::OperationCanceled => {
                    McpError::internal_error("用户取消了操作".to_string(), None)
                }
                _ => McpError::internal_error(format!("输入失败: {}", e), None)
            })?;

        if input.trim().is_empty() {
            Ok("用户确认继续".to_string())
        } else if input.trim().to_lowercase() == "cancel" {
            Ok("用户取消了操作".to_string())
        } else if input.trim().to_lowercase() == "continue" {
            Ok("用户确认继续".to_string())
        } else {
            Ok(format!("用户输入: {}", input.trim()))
        }
    }
}

/// 智能代码审查交互工具
///
/// 支持预定义选项、自由文本输入和图片上传
#[derive(Clone)]
pub struct InteractionTool;

impl InteractionTool {
    pub async fn zhi(
        request: ZhiRequest,
    ) -> Result<CallToolResult, McpError> {
        // 首先尝试使用独立UI进程（类似原始项目的GUI方案）
        match Self::handle_ui_process_interaction(&request).await {
            Ok(response) => {
                let content = vec![Content::text(response)];
                return Ok(CallToolResult::success(content));
            }
            Err(e) => {
                log::warn!("独立UI进程失败: {}", e);

                // 检查是否请求终端模式作为回退
                let terminal_mode = request.terminal_mode.unwrap_or(false);
                if terminal_mode {
                    if let Ok(config) = crate::config::load_standalone_config() {
                        if config.terminal_config.enabled {
                            // 尝试在新终端窗口中启动交互
                            match Self::handle_terminal_interaction(&request).await {
                                Ok(response) => {
                                    let content = vec![Content::text(response)];
                                    return Ok(CallToolResult::success(content));
                                }
                                Err(e) => {
                                    log::warn!("终端模式也失败: {}", e);
                                }
                            }
                        }
                    }
                }

                // 最后回退到CLI交互模式
                log::info!("回退到CLI交互模式");
                let response = Self::handle_cli_interaction(&request).await?;
                let content = vec![Content::text(response)];
                Ok(CallToolResult::success(content))
            }
        }
    }

    /// 处理独立UI进程交互（类似原始项目的GUI方案）
    async fn handle_ui_process_interaction(request: &ZhiRequest) -> Result<String, McpError> {
        use crate::mcp::types::PopupRequest;
        use crate::mcp::handlers::popup::create_cli_popup;

        // 生成请求ID
        let request_id = uuid::Uuid::new_v4().to_string();

        // 创建弹窗请求
        let popup_request = PopupRequest {
            id: request_id,
            message: request.message.clone(),
            predefined_options: if request.predefined_options.is_empty() {
                None
            } else {
                Some(request.predefined_options.clone())
            },
            is_markdown: request.is_markdown,
        };

        // 调用独立UI进程
        match create_cli_popup(&popup_request) {
            Ok(response) => Ok(response),
            Err(e) => Err(McpError::internal_error(
                format!("UI进程交互失败: {}", e),
                None
            ))
        }
    }

    /// 处理终端交互模式
    async fn handle_terminal_interaction(request: &ZhiRequest) -> Result<String, McpError> {
        // 加载配置获取超时设置
        let config = crate::config::load_standalone_config()
            .map_err(|e| McpError::internal_error(
                format!("加载配置失败: {}", e),
                None
            ))?;

        // 创建临时脚本来运行交互
        let script_content = Self::generate_interaction_script(request)?;
        let temp_script_path = Self::create_temp_script(&script_content)?;

        // 配置终端启动器
        let mut launcher_config = crate::utils::terminal_launcher::TerminalLauncherConfig::default();
        launcher_config.window_title = Some(config.terminal_config.window_title.clone());
        launcher_config.fallback_to_cli = config.terminal_config.fallback_to_cli;
        launcher_config.working_directory = env::current_dir().ok();

        // 设置首选终端
        if let Some(ref preferred) = config.terminal_config.preferred_terminal {
            // 将字符串转换为 TerminalType
            launcher_config.preferred_terminal = Some(Self::parse_terminal_type(preferred));
        }

        let launcher = TerminalLauncher::new(launcher_config);

        // 启动终端并等待结果
        match launcher.launch_terminal_with_command("bash", &[temp_script_path.to_string_lossy().to_string()]).await {
            Ok(_) => {
                // 等待用户交互完成并读取结果
                Self::wait_for_terminal_result(&temp_script_path, config.terminal_config.timeout_seconds).await
            }
            Err(e) => {
                // 清理临时文件
                let _ = std::fs::remove_file(&temp_script_path);
                Err(McpError::internal_error(
                    format!("终端启动失败: {}", e),
                    None
                ))
            }
        }
    }

    /// 将字符串解析为 TerminalType
    fn parse_terminal_type(terminal_str: &str) -> crate::utils::terminal_launcher::TerminalType {
        use crate::utils::terminal_launcher::TerminalType;

        match terminal_str.to_lowercase().as_str() {
            "terminal" | "terminal.app" => TerminalType::TerminalApp,
            "iterm" | "iterm2" => TerminalType::ITerm2,
            "alacritty" => TerminalType::Alacritty,
            "gnome-terminal" => TerminalType::GnomeTerminal,
            "konsole" => TerminalType::Konsole,
            "xterm" => TerminalType::Xterm,
            "cmd" => TerminalType::Cmd,
            "powershell" => TerminalType::PowerShell,
            "wt" | "windows-terminal" => TerminalType::WindowsTerminal,
            _ => TerminalType::Custom(terminal_str.to_string()),
        }
    }

    /// 等待终端交互结果（使用配置的超时时间）
    async fn wait_for_terminal_result(script_path: &PathBuf, timeout_seconds: u32) -> Result<String, McpError> {
        use tokio::time::{sleep, Duration};

        // 解析响应文件路径
        let temp_dir = env::temp_dir();
        let mut response_files: Vec<_> = Vec::new();

        // 等待响应文件出现
        let max_iterations = timeout_seconds;
        for i in 0..max_iterations {
            // 每次都重新扫描目录，寻找新的响应文件
            if let Ok(entries) = std::fs::read_dir(&temp_dir) {
                response_files = entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        entry.file_name()
                            .to_string_lossy()
                            .starts_with("cunzhi_response_")
                    })
                    .collect();
            }

            // 检查是否有响应文件包含内容
            for response_file_entry in &response_files {
                let response_path = response_file_entry.path();
                if response_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&response_path) {
                        if !content.trim().is_empty() {
                            // 清理临时文件
                            let _ = std::fs::remove_file(script_path);
                            let _ = std::fs::remove_file(&response_path);

                            log::info!("收到用户响应，用时 {} 秒", i + 1);
                            return Ok(content.trim().to_string());
                        }
                    }
                }
            }

            // 每秒检查一次
            sleep(Duration::from_secs(1)).await;

            // 每30秒输出一次等待信息
            if i % 30 == 29 {
                log::info!("等待用户在终端中完成交互... ({}/{} 秒)", i + 1, max_iterations);
            }
        }

        // 超时清理
        let _ = std::fs::remove_file(script_path);

        Err(McpError::internal_error(
            format!("终端交互超时（{}秒），未收到用户响应", timeout_seconds),
            None
        ))
    }

    /// 生成交互脚本内容
    fn generate_interaction_script(request: &ZhiRequest) -> Result<String, McpError> {
        let temp_dir = env::temp_dir();
        let response_file = temp_dir.join(format!("cunzhi_response_{}.txt", uuid::Uuid::new_v4()));

        let script = format!(r#"#!/bin/bash

# 寸止 CLI 终端交互脚本
echo "🤖 寸止 AI 助手"
echo "════════════════════════════════════════════════"

# 显示消息
cat << 'EOF'
{}
EOF

echo "════════════════════════════════════════════════"

# 处理预定义选项
if [ {} -gt 0 ]; then
    echo "📋 可选选项:"
{}
    echo ""
    echo "请选择一个选项 (输入数字)，或输入自定义内容："

    read -p "> " user_input

    # 检查是否是数字选择
    if [[ "$user_input" =~ ^[0-9]+$ ]] && [ "$user_input" -ge 1 ] && [ "$user_input" -le {} ]; then
        case $user_input in
{})
        echo "用户选择: $selected_option" > "{}"
    else
        echo "用户输入: $user_input" > "{}"
    fi
else
    echo "请输入您的回复:"
    read -p "> " user_input
    echo "用户输入: $user_input" > "{}"
fi

echo ""
echo "回复已记录，您可以关闭此终端窗口。"
echo "按任意键退出..."
read -n 1
"#,
            request.message,
            request.predefined_options.len(),
            Self::format_options_for_script(&request.predefined_options),
            request.predefined_options.len(),
            Self::generate_case_statements(&request.predefined_options),
            response_file.to_string_lossy(),
            response_file.to_string_lossy(),
            response_file.to_string_lossy()
        );

        Ok(script)
    }

    /// 格式化选项用于脚本显示
    fn format_options_for_script(options: &[String]) -> String {
        options.iter()
            .enumerate()
            .map(|(i, option)| format!("    {}. {}", i + 1, option))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 生成 case 语句
    fn generate_case_statements(options: &[String]) -> String {
        options.iter()
            .enumerate()
            .map(|(i, option)| {
                format!("        {}) selected_option=\"{}\" ;;", i + 1, option)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 创建临时脚本文件
    fn create_temp_script(content: &str) -> Result<PathBuf, McpError> {
        let temp_dir = env::temp_dir();
        let script_path = temp_dir.join(format!("cunzhi_terminal_{}.sh", uuid::Uuid::new_v4()));

        std::fs::write(&script_path, content)
            .map_err(|e| McpError::internal_error(
                format!("创建临时脚本失败: {}", e),
                None
            ))?;

        // 在 Unix 系统上设置执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&script_path)
                .and_then(|m| Ok(m.permissions()))
                .map_err(|e| McpError::internal_error(
                    format!("获取文件权限失败: {}", e),
                    None
                ))?;
            perms.set_mode(0o755);
            std::fs::set_permissions(&script_path, perms)
                .map_err(|e| McpError::internal_error(
                    format!("设置文件权限失败: {}", e),
                    None
                ))?;
        }

        Ok(script_path)
    }

    /// 处理CLI交互
    async fn handle_cli_interaction(request: &ZhiRequest) -> Result<String, McpError> {
        // 显示消息头部
        println!("\n{}", style("🤖 AI助手").cyan().bold());
        println!("{}", style("─".repeat(50)).dim());

        // 显示消息内容
        if request.is_markdown {
            let formatted_message = Self::render_simple_markdown(&request.message);
            println!("{}", formatted_message);
        } else {
            println!("{}", request.message);
        }

        println!("{}", style("─".repeat(50)).dim());

        // 处理用户交互
        if !request.predefined_options.is_empty() {
            // 有预定义选项，使用增强的选择界面
            EnhancedCliInteraction::handle_option_selection(&request.predefined_options)
        } else {
            // 没有预定义选项，直接获取用户输入
            EnhancedCliInteraction::handle_custom_input()
        }
    }

    /// 简单的Markdown渲染
    fn render_simple_markdown(text: &str) -> String {
        let mut result = text.to_string();

        // 处理粗体 **text**
        while let Some(start) = result.find("**") {
            if let Some(end) = result[start + 2..].find("**") {
                let end = start + 2 + end;
                let bold_text = &result[start + 2..end];
                let colored_text = colorize_with_style(bold_text, colors::WHITE, true);
                result.replace_range(start..end + 2, &colored_text);
            } else {
                break;
            }
        }

        // 处理代码块 `code`
        while let Some(start) = result.find('`') {
            if let Some(end) = result[start + 1..].find('`') {
                let end = start + 1 + end;
                let code_text = &result[start + 1..end];
                let colored_text = colorize(code_text, colors::CYAN);
                result.replace_range(start..end + 1, &colored_text);
            } else {
                break;
            }
        }

        result
    }

    /// 处理MCP交互（现在也支持交互式输入）
    async fn handle_mcp_interaction(request: &ZhiRequest) -> Result<String, McpError> {
        // 修改：现在 MCP 模式也支持交互式输入
        // 直接调用 CLI 交互处理逻辑
        Self::handle_cli_interaction(request).await
    }

    /// 简单的Markdown渲染（纯文本版本）
    fn render_simple_markdown_plain(text: &str) -> String {
        let mut result = text.to_string();

        // 移除粗体标记 **text**
        while let Some(start) = result.find("**") {
            if let Some(end) = result[start + 2..].find("**") {
                let end = start + 2 + end;
                let bold_text = result[start + 2..end].to_string();
                result.replace_range(start..end + 2, &bold_text);
            } else {
                break;
            }
        }

        // 移除代码块标记 `code`
        while let Some(start) = result.find('`') {
            if let Some(end) = result[start + 1..].find('`') {
                let end = start + 1 + end;
                let code_text = result[start + 1..end].to_string();
                result.replace_range(start..end + 1, &code_text);
            } else {
                break;
            }
        }

        result
    }
}
