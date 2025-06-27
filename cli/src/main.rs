use anyhow::Result;
use clap::Parser;
use cunzhi_cli::cli::Cli;
use cunzhi_cli::utils::auto_init_logger;
use cunzhi_cli::{log_important, log_error};

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    if let Err(e) = auto_init_logger() {
        eprintln!("初始化日志系统失败: {}", e);
        std::process::exit(1);
    }

    // 解析命令行参数
    let cli = Cli::parse();

    // 记录启动信息
    let verbose = cli.verbose;
    if verbose {
        log_important!(info, "启动 cunzhi CLI v{}", env!("CARGO_PKG_VERSION"));
        log_important!(info, "详细模式已启用");
    }

    // 执行命令
    match cli.execute().await {
        Ok(_) => {
            if verbose {
                log_important!(info, "命令执行完成");
            }
            Ok(())
        }
        Err(e) => {
            log_error!("命令执行失败: {}", e);
            Err(e)
        }
    }
}
