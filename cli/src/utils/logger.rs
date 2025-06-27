// 日志系统 - CLI 版本优化
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Once;
use log::LevelFilter;
use env_logger::{Builder, Target};

static INIT: Once = Once::new();

/// 日志配置
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// 日志级别
    pub level: LevelFilter,
    /// 日志文件路径（None 表示不输出到文件）
    pub file_path: Option<String>,
    /// 是否为 MCP 模式（MCP 模式下不输出到 stderr）
    pub is_mcp_mode: bool,
    /// 是否启用彩色输出
    pub enable_colors: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LevelFilter::Info, // CLI 默认使用 Info 级别
            file_path: None,
            is_mcp_mode: false,
            enable_colors: true, // CLI 默认启用彩色输出
        }
    }
}

/// 初始化日志系统
pub fn init_logger(config: LogConfig) -> anyhow::Result<()> {
    INIT.call_once(|| {
        let mut builder = Builder::new();

        // 设置日志级别
        builder.filter_level(config.level);

        // 设置日志格式 - CLI 友好的格式
        if config.is_mcp_mode {
            // MCP 模式：详细格式，包含时间戳
            builder.format(|buf, record| {
                writeln!(
                    buf,
                    "{} [{}] [{}] {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.module_path().unwrap_or("unknown"),
                    record.args()
                )
            });
        } else {
            // CLI 模式：简洁格式，适合终端显示
            if config.enable_colors {
                builder.format(|buf, record| {
                    use std::io::Write;
                    let level_color = match record.level() {
                        log::Level::Error => "\x1b[31m", // 红色
                        log::Level::Warn => "\x1b[33m",  // 黄色
                        log::Level::Info => "\x1b[32m",  // 绿色
                        log::Level::Debug => "\x1b[36m", // 青色
                        log::Level::Trace => "\x1b[35m", // 紫色
                    };
                    writeln!(
                        buf,
                        "{}[{}]\x1b[0m {}",
                        level_color,
                        record.level(),
                        record.args()
                    )
                });
            } else {
                builder.format(|buf, record| {
                    writeln!(
                        buf,
                        "[{}] {}",
                        record.level(),
                        record.args()
                    )
                });
            }
        }

        // 根据模式设置输出目标
        if config.is_mcp_mode {
            // MCP 模式：只输出到文件，不输出到 stderr
            if let Some(file_path) = &config.file_path {
                if let Ok(log_file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)
                {
                    builder.target(Target::Pipe(Box::new(log_file)));
                } else {
                    // 如果文件打开失败，禁用日志输出
                    builder.filter_level(LevelFilter::Off);
                }
            } else {
                // MCP 模式下没有指定文件路径，禁用日志输出
                builder.filter_level(LevelFilter::Off);
            }
        } else {
            // CLI 模式：输出到 stderr
            builder.target(Target::Stderr);
        }

        builder.init();
    });

    Ok(())
}

/// 自动检测模式并初始化日志系统 - CLI 版本优化
pub fn auto_init_logger() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_mcp_mode = args.iter().any(|arg| arg == "--mcp-request" || arg.contains("mcp_server"));

    // 检测是否在终端环境中运行
    let is_terminal = atty::is(atty::Stream::Stderr);

    let config = if is_mcp_mode {
        // MCP 模式：输出到文件
        let log_file_path = env::var("MCP_LOG_FILE")
            .unwrap_or_else(|_| {
                let temp_dir = env::temp_dir();
                temp_dir.join("cunzhi-mcp.log").to_string_lossy().to_string()
            });

        LogConfig {
            level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "warn".to_string())
                .parse::<LevelFilter>()
                .unwrap_or(LevelFilter::Warn),
            file_path: Some(log_file_path),
            is_mcp_mode: true,
            enable_colors: false, // MCP 模式不需要颜色
        }
    } else {
        // CLI 模式：输出到 stderr，根据环境决定是否启用颜色
        LogConfig {
            level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string())
                .parse::<LevelFilter>()
                .unwrap_or(LevelFilter::Info),
            file_path: None,
            is_mcp_mode: false,
            enable_colors: is_terminal && env::var("NO_COLOR").is_err(), // 支持 NO_COLOR 环境变量
        }
    };

    init_logger(config)
}

/// 便利宏：只在重要情况下记录日志
#[macro_export]
macro_rules! log_important {
    (error, $($arg:tt)*) => {
        log::error!($($arg)*)
    };
    (warn, $($arg:tt)*) => {
        log::warn!($($arg)*)
    };
    (info, $($arg:tt)*) => {
        log::info!($($arg)*)
    };
}

/// 便利宏：调试日志（只在 debug 级别下输出）
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*)
    };
}

/// 便利宏：跟踪日志（只在 trace 级别下输出）
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        log::trace!($($arg)*)
    };
}

/// CLI 专用的成功消息宏
#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        if atty::is(atty::Stream::Stderr) && std::env::var("NO_COLOR").is_err() {
            eprintln!("\x1b[32m✅ {}\x1b[0m", format!($($arg)*));
        } else {
            eprintln!("✅ {}", format!($($arg)*));
        }
    };
}

/// CLI 专用的错误消息宏
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        if atty::is(atty::Stream::Stderr) && std::env::var("NO_COLOR").is_err() {
            eprintln!("\x1b[31m❌ {}\x1b[0m", format!($($arg)*));
        } else {
            eprintln!("❌ {}", format!($($arg)*));
        }
    };
}

/// CLI 专用的警告消息宏
#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        if atty::is(atty::Stream::Stderr) && std::env::var("NO_COLOR").is_err() {
            eprintln!("\x1b[33m⚠️  {}\x1b[0m", format!($($arg)*));
        } else {
            eprintln!("⚠️  {}", format!($($arg)*));
        }
    };
}
