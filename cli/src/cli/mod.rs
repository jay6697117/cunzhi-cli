pub mod commands;
pub mod init;
pub mod interactive;
pub mod config;
pub mod server;
pub mod mcp;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::utils::{ErrorHandler, setup_panic_handler};

#[derive(Parser)]
#[command(name = "cunzhi")]
#[command(about = "寸止 CLI - 智能代码审查工具的命令行版本")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// 启用详细输出
    #[arg(short, long)]
    pub verbose: bool,

    /// 指定配置文件路径
    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// MCP 服务器管理
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },
    /// 启动纯净的 MCP 服务器（无 UI）
    McpServer,
    /// 配置管理
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// MCP 配置管理
    Mcp {
        #[command(subcommand)]
        action: McpAction,
    },
    /// 项目初始化向导
    Init {
        /// 项目名称
        #[arg(short, long)]
        name: Option<String>,
        /// 跳过交互式提示，使用默认配置
        #[arg(short, long)]
        yes: bool,
    },
    /// 显示版本信息
    Version,
    /// 显示系统信息和诊断
    Doctor,
}

#[derive(Subcommand)]
pub enum ServerAction {
    /// 启动服务器
    Start,
    /// 停止服务器
    Stop,
    /// 查看服务器状态
    Status,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// 交互式配置设置
    Set,
    /// 显示当前配置
    Show,
    /// 验证配置
    Validate,
}

#[derive(Subcommand)]
pub enum McpAction {
    /// 生成 MCP 客户端配置文件
    Generate,
    /// 验证 MCP 配置
    Validate,
    /// 启动交互式聊天界面
    Chat,
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        // 设置 panic 处理器
        setup_panic_handler();

        let result = match self.command {
            Some(Commands::Server { action }) => {
                server::handle_server_command(action).await
            }
            Some(Commands::Config { action }) => {
                config::handle_config_command(action).await
            }
            Some(Commands::Mcp { action }) => {
                mcp::handle_mcp_command(action).await
            }
            Some(Commands::Init { name, yes }) => {
                // 使用新的项目初始化向导
                init::run_project_init_wizard(name, yes).await
            }
            Some(Commands::Version) => {
                commands::show_version().await
            }
            Some(Commands::Doctor) => {
                commands::run_doctor().await
            }
            Some(Commands::McpServer) => {
                // 启动纯净的 MCP 服务器，无 UI 交互
                crate::mcp::run_server().await.map_err(|e| anyhow::anyhow!("{}", e))
            }
            None => {
                commands::show_default_help().await
            }
        };

        // 如果有错误，使用错误处理器处理
        if let Err(ref error) = result {
            let handler = ErrorHandler::new(self.verbose);
            let exit_code = handler.handle_error(error);
            std::process::exit(exit_code);
        }

        result
    }
}
