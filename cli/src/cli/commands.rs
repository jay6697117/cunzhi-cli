// CLI 命令实现
use anyhow::Result;
use crate::config::{load_standalone_config, get_standalone_config_path};
use crate::utils::{print_boxed_message, format_file_size, colorize, colors};
use crate::{log_success, log_warning, log_error};
use std::env;

/// 显示版本信息
pub async fn show_version() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");

    print_boxed_message(
        &format!("寸止 CLI v{}", version),
        description
    );

    println!("\n📦 构建信息:");
    println!("  版本: {}", version);
    println!("  目标平台: {}", std::env::consts::ARCH);
    println!("  操作系统: {}", std::env::consts::OS);
    println!("  Rust 版本: {}", env!("CARGO_PKG_RUST_VERSION"));

    Ok(())
}

/// 显示默认帮助信息
pub async fn show_default_help() -> Result<()> {
    println!("{}", colorize("🤖 寸止 CLI", colors::BOLD));
    println!("智能代码审查工具的命令行版本 v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("{}:", colorize("常用命令", colors::CYAN));
    println!("  {} - 项目初始化向导", colorize("cunzhi init", colors::GREEN));
    println!("  {} - 显示配置信息", colorize("cunzhi config show", colors::GREEN));
    println!("  {} - 启动 MCP 服务器", colorize("cunzhi server start", colors::GREEN));
    println!("  {} - 系统诊断", colorize("cunzhi doctor", colors::GREEN));
    println!();
    println!("使用 {} 查看完整帮助信息", colorize("cunzhi --help", colors::YELLOW));

    Ok(())
}

/// 运行系统诊断
pub async fn run_doctor() -> Result<()> {
    print_boxed_message("系统诊断", "检查 cunzhi CLI 的运行环境和配置");

    println!("\n🔍 环境检查:");

    // 检查 Rust 版本
    if let Ok(output) = std::process::Command::new("rustc").arg("--version").output() {
        let version = String::from_utf8_lossy(&output.stdout);
        log_success!("Rust 编译器: {}", version.trim());
    } else {
        log_warning!("无法检测 Rust 编译器版本");
    }

    // 检查配置文件
    println!("\n📁 配置检查:");
    match get_standalone_config_path() {
        Ok(config_path) => {
            if config_path.exists() {
                match std::fs::metadata(&config_path) {
                    Ok(metadata) => {
                        log_success!("配置文件: {} ({})",
                            config_path.display(),
                            format_file_size(metadata.len())
                        );

                        // 验证配置
                        match load_standalone_config() {
                            Ok(config) => {
                                match config.validate() {
                                    Ok(_) => log_success!("配置验证: 通过"),
                                    Err(e) => log_error!("配置验证: 失败 - {}", e),
                                }
                            }
                            Err(e) => log_error!("配置加载: 失败 - {}", e),
                        }
                    }
                    Err(e) => log_error!("配置文件访问失败: {}", e),
                }
            } else {
                log_warning!("配置文件不存在: {}", config_path.display());
                println!("  运行 {} 创建默认配置", colorize("cunzhi init", colors::CYAN));
            }
        }
        Err(e) => log_error!("无法获取配置路径: {}", e),
    }

    // 检查终端环境
    println!("\n🖥️  终端环境:");
    log_success!("终端类型: {}",
        if atty::is(atty::Stream::Stderr) { "交互式" } else { "非交互式" }
    );
    log_success!("彩色支持: {}",
        if crate::utils::supports_color() { "是" } else { "否" }
    );
    log_success!("CI 环境: {}",
        if crate::utils::is_ci_environment() { "是" } else { "否" }
    );

    // 检查权限
    println!("\n🔐 权限检查:");
    match get_standalone_config_path() {
        Ok(config_path) => {
            if let Some(parent) = config_path.parent() {
                match std::fs::create_dir_all(parent) {
                    Ok(_) => log_success!("配置目录写入权限: 正常"),
                    Err(e) => log_error!("配置目录写入权限: 失败 - {}", e),
                }
            }
        }
        Err(e) => log_error!("权限检查失败: {}", e),
    }

    println!("\n✅ 诊断完成");
    Ok(())
}
