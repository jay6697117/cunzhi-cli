// 独立的CLI交互UI程序
// 类似于原始项目的GUI程序，但使用命令行界面

use anyhow::Result;
use clap::{Arg, Command};
use cunzhi_cli::mcp::types::{PopupRequest, Content};
use cunzhi_cli::mcp::tools::interaction::mcp::EnhancedCliInteraction;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let matches = Command::new("cunzhi-ui")
        .version(env!("CARGO_PKG_VERSION"))
        .about("寸止 CLI 交互界面")
        .arg(
            Arg::new("mcp-request")
                .long("mcp-request")
                .value_name("FILE")
                .help("MCP请求文件路径")
                .required(true)
        )
        .get_matches();

    if let Some(request_file) = matches.get_one::<String>("mcp-request") {
        handle_mcp_request(request_file)
    } else {
        eprintln!("错误: 需要提供 --mcp-request 参数");
        std::process::exit(1);
    }
}

/// 处理MCP请求
fn handle_mcp_request(request_file: &str) -> Result<()> {
    let request_path = PathBuf::from(request_file);

    // 读取请求文件
    let request_json = fs::read_to_string(&request_path)?;
    let popup_request: PopupRequest = serde_json::from_str(&request_json)?;

    // 执行CLI交互
    let response = execute_cli_interaction(&popup_request)?;

    // 输出响应到stdout（MCP服务器会读取这个输出）
    println!("{}", response);

    Ok(())
}

/// 执行CLI交互
fn execute_cli_interaction(request: &PopupRequest) -> Result<String> {
    // 显示消息头部
    println!("\n🤖 寸止 AI 助手");
    println!("{}", "─".repeat(50));

    // 显示消息内容
    if request.is_markdown {
        let formatted_message = render_simple_markdown(&request.message);
        println!("{}", formatted_message);
    } else {
        println!("{}", request.message);
    }

    println!("{}", "─".repeat(50));

    // 检查是否在TTY环境中
    if !atty::is(atty::Stream::Stdin) {
        // 非TTY环境，使用简单的文本输入
        return handle_non_tty_interaction(request);
    }

    // 处理用户交互
    if let Some(ref options) = request.predefined_options {
        if !options.is_empty() {
            // 有预定义选项，使用选择界面
            return Ok(EnhancedCliInteraction::handle_option_selection(options)
                .map_err(|e| anyhow::anyhow!("交互失败: {}", e))?);
        }
    }

    // 没有预定义选项，直接获取用户输入
    Ok(EnhancedCliInteraction::handle_custom_input()
        .map_err(|e| anyhow::anyhow!("交互失败: {}", e))?)
}

/// 处理非TTY环境的交互（如测试环境）
fn handle_non_tty_interaction(request: &PopupRequest) -> Result<String> {
    // 在非TTY环境中，我们返回第一个预定义选项或默认响应
    if let Some(ref options) = request.predefined_options {
        if !options.is_empty() {
            // 返回第一个选项作为默认选择
            return Ok(options[0].clone());
        }
    }

    // 没有预定义选项，返回默认响应
    Ok("自动响应（非交互环境）".to_string())
}

/// 简单的Markdown渲染
fn render_simple_markdown(text: &str) -> String {
    use console::style;

    let mut result = String::new();
    let lines: Vec<&str> = text.lines().collect();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("# ") {
            // H1 标题
            let title = &trimmed[2..];
            result.push_str(&format!("{}\n", style(title).bold().cyan()));
        } else if trimmed.starts_with("## ") {
            // H2 标题
            let title = &trimmed[3..];
            result.push_str(&format!("{}\n", style(title).bold().yellow()));
        } else if trimmed.starts_with("### ") {
            // H3 标题
            let title = &trimmed[4..];
            result.push_str(&format!("{}\n", style(title).bold().green()));
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            // 列表项
            let item = &trimmed[2..];
            result.push_str(&format!("  • {}\n", item));
        } else if trimmed.starts_with("**") && trimmed.ends_with("**") && trimmed.len() > 4 {
            // 粗体
            let bold_text = &trimmed[2..trimmed.len()-2];
            result.push_str(&format!("{}\n", style(bold_text).bold()));
        } else if trimmed.starts_with("*") && trimmed.ends_with("*") && trimmed.len() > 2 {
            // 斜体
            let italic_text = &trimmed[1..trimmed.len()-1];
            result.push_str(&format!("{}\n", style(italic_text).italic()));
        } else if trimmed.starts_with("`") && trimmed.ends_with("`") && trimmed.len() > 2 {
            // 内联代码
            let code_text = &trimmed[1..trimmed.len()-1];
            result.push_str(&format!("{}\n", style(code_text).on_black().white()));
        } else if trimmed.starts_with("```") {
            // 代码块开始/结束 - 简单处理
            result.push_str(&format!("{}\n", style("───").dim()));
        } else if trimmed.is_empty() {
            // 空行
            result.push('\n');
        } else {
            // 普通文本
            result.push_str(&format!("{}\n", line));
        }
    }

    result
}
