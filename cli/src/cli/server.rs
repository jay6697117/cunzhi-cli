// MCP 服务器管理命令实现
use anyhow::Result;
use crate::cli::ServerAction;
use crate::mcp::ZhiServer;
use crate::utils::{print_boxed_message, colorize, colors, ModernProgressBar, StatusIndicator, theme};
use console;
use crate::{log_success, log_warning, log_error};

pub async fn handle_server_command(action: ServerAction) -> Result<()> {
    match action {
        ServerAction::Start => {
            start_server().await
        }
        ServerAction::Stop => {
            stop_server().await
        }
        ServerAction::Status => {
            show_server_status().await
        }
    }
}

/// 启动 MCP 服务器
async fn start_server() -> Result<()> {
    let indicator = StatusIndicator::new();

    print_boxed_message("启动 MCP 服务器", "正在初始化智能代码审查服务");

    // 显示启动进度
    let pb = ModernProgressBar::new_spinner();
    pb.set_message("正在初始化服务器...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let server = ZhiServer::new();

    // 模拟启动过程
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    pb.set_message("正在加载配置...");

    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    pb.set_message("正在启动工具...");

    match server.start().await {
        Ok(_) => {
            pb.finish_with_message("服务器启动成功！");
            indicator.success("MCP 服务器已成功启动");

            // 显示服务器信息
            let tools = server.list_tools();
            if !tools.is_empty() {
                println!("\n📋 可用工具:");
                for tool in tools {
                    let status_color = if tool.enabled { theme::SUCCESS } else { theme::MUTED };
                    println!("  {} - {}",
                        console::style(&tool.name).fg(status_color).bold(),
                        console::style(&tool.description).dim()
                    );
                }
            }

            println!("\n💡 下一步操作:");
            println!("  {} - 查看服务器状态", console::style("cunzhi server status").fg(theme::PRIMARY));
            println!("  {} - 停止服务器", console::style("cunzhi server stop").fg(theme::PRIMARY));

            Ok(())
        }
        Err(e) => {
            pb.finish_with_message("启动失败");
            indicator.error(&format!("MCP 服务器启动失败: {}", e));
            Err(e)
        }
    }
}

/// 停止 MCP 服务器
async fn stop_server() -> Result<()> {
    print_boxed_message("停止 MCP 服务器", "正在关闭服务");

    let server = ZhiServer::new();

    match server.stop().await {
        Ok(_) => {
            log_success!("MCP 服务器已停止");
            Ok(())
        }
        Err(e) => {
            log_warning!("停止服务器时出现问题: {}", e);
            // 即使出错也返回成功，因为可能服务器本来就没运行
            Ok(())
        }
    }
}

/// 显示服务器状态
async fn show_server_status() -> Result<()> {
    print_boxed_message("MCP 服务器状态", "检查服务器运行状态");

    let server = ZhiServer::new();

    match server.status().await {
        Ok(status) => {
            println!("🔍 服务器状态: {}", colorize(&status, colors::GREEN));

            // 显示工具状态
            let tools = server.list_tools();
            if !tools.is_empty() {
                println!("\n🛠️  工具状态:");
                for tool in tools {
                    let status_icon = if tool.enabled { "✅" } else { "❌" };
                    println!("  {} {} - {}", status_icon, tool.name, tool.description);
                }
            } else {
                log_warning!("没有启用的工具");
            }

            // 显示配置信息
            match crate::config::load_standalone_config() {
                Ok(config) => {
                    println!("\n⚙️  配置信息:");
                    println!("  配置版本: {}", config.version);
                    let enabled_count = config.mcp_config.tools.values().filter(|&&v| v).count();
                    println!("  启用工具数: {}", enabled_count);
                }
                Err(_) => {
                    log_warning!("无法加载配置信息");
                }
            }

            Ok(())
        }
        Err(e) => {
            log_error!("获取服务器状态失败: {}", e);
            Err(e)
        }
    }
}
