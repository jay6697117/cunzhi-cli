// 统一错误处理模块
use anyhow::{Result, Context};
use console::{style, Term};
use thiserror::Error;

use super::ui::StatusIndicator;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("配置错误: {message}")]
    Config { message: String },

    #[error("文件操作错误: {message}")]
    FileOperation { message: String },

    #[error("网络错误: {message}")]
    Network { message: String },

    #[error("MCP 服务器错误: {message}")]
    McpServer { message: String },

    #[error("用户输入错误: {message}")]
    UserInput { message: String },

    #[error("权限错误: {message}")]
    Permission { message: String },

    #[error("依赖错误: {message}")]
    Dependency { message: String },

    #[error("内部错误: {message}")]
    Internal { message: String },
}

impl AppError {
    /// 创建配置错误
    pub fn config(msg: &str) -> Self {
        Self::Config { message: msg.to_string() }
    }

    /// 创建文件操作错误
    pub fn file_operation(msg: &str) -> Self {
        Self::FileOperation { message: msg.to_string() }
    }

    /// 创建网络错误
    pub fn network(msg: &str) -> Self {
        Self::Network { message: msg.to_string() }
    }

    /// 创建 MCP 服务器错误
    pub fn mcp_server(msg: &str) -> Self {
        Self::McpServer { message: msg.to_string() }
    }

    /// 创建用户输入错误
    pub fn user_input(msg: &str) -> Self {
        Self::UserInput { message: msg.to_string() }
    }

    /// 创建权限错误
    pub fn permission(msg: &str) -> Self {
        Self::Permission { message: msg.to_string() }
    }

    /// 创建依赖错误
    pub fn dependency(msg: &str) -> Self {
        Self::Dependency { message: msg.to_string() }
    }

    /// 创建内部错误
    pub fn internal(msg: &str) -> Self {
        Self::Internal { message: msg.to_string() }
    }
}

/// 错误处理器
pub struct ErrorHandler {
    indicator: StatusIndicator,
    term: Term,
    verbose: bool,
}

impl ErrorHandler {
    pub fn new(verbose: bool) -> Self {
        Self {
            indicator: StatusIndicator::new(),
            term: Term::stderr(),
            verbose,
        }
    }

    /// 处理错误并显示用户友好的消息
    pub fn handle_error(&self, error: &anyhow::Error) -> i32 {
        // 尝试转换为应用程序错误
        if let Some(app_error) = error.downcast_ref::<AppError>() {
            self.handle_app_error(app_error)
        } else {
            self.handle_generic_error(error)
        }
    }

    /// 处理应用程序特定错误
    fn handle_app_error(&self, error: &AppError) -> i32 {
        match error {
            AppError::Config { message } => {
                self.indicator.error(&format!("配置错误: {}", message));
                self.show_config_help();
                1
            }
            AppError::FileOperation { message } => {
                self.indicator.error(&format!("文件操作失败: {}", message));
                self.show_file_help();
                2
            }
            AppError::Network { message } => {
                self.indicator.error(&format!("网络连接失败: {}", message));
                self.show_network_help();
                3
            }
            AppError::McpServer { message } => {
                self.indicator.error(&format!("MCP 服务器错误: {}", message));
                self.show_mcp_help();
                4
            }
            AppError::UserInput { message } => {
                self.indicator.error(&format!("输入错误: {}", message));
                self.show_input_help();
                5
            }
            AppError::Permission { message } => {
                self.indicator.error(&format!("权限不足: {}", message));
                self.show_permission_help();
                6
            }
            AppError::Dependency { message } => {
                self.indicator.error(&format!("依赖错误: {}", message));
                self.show_dependency_help();
                7
            }
            AppError::Internal { message } => {
                self.indicator.error(&format!("内部错误: {}", message));
                self.show_internal_help();
                8
            }
        }
    }

    /// 处理通用错误
    fn handle_generic_error(&self, error: &anyhow::Error) -> i32 {
        self.indicator.error(&format!("发生错误: {}", error));

        if self.verbose {
            self.term.write_line("").unwrap_or(());
            self.term.write_line(&style("详细错误信息:").dim().to_string()).unwrap_or(());

            // 显示错误链
            let mut current = error.source();
            let mut level = 1;
            while let Some(err) = current {
                let indent = "  ".repeat(level);
                self.term.write_line(&format!("{}└─ {}", indent, err)).unwrap_or(());
                current = err.source();
                level += 1;
            }
        }

        99
    }

    /// 显示配置相关帮助
    fn show_config_help(&self) {
        self.indicator.info("尝试以下解决方案:");
        println!("  • 运行 {} 重新初始化配置", style("cunzhi init").cyan());
        println!("  • 检查配置文件格式是否正确");
        println!("  • 运行 {} 查看当前配置", style("cunzhi config show").cyan());
    }

    /// 显示文件操作相关帮助
    fn show_file_help(&self) {
        self.indicator.info("尝试以下解决方案:");
        println!("  • 检查文件路径是否正确");
        println!("  • 确保有足够的磁盘空间");
        println!("  • 检查文件权限设置");
    }

    /// 显示网络相关帮助
    fn show_network_help(&self) {
        self.indicator.info("尝试以下解决方案:");
        println!("  • 检查网络连接");
        println!("  • 确认防火墙设置");
        println!("  • 稍后重试");
    }

    /// 显示 MCP 服务器相关帮助
    fn show_mcp_help(&self) {
        self.indicator.info("尝试以下解决方案:");
        println!("  • 运行 {} 检查服务器状态", style("cunzhi server status").cyan());
        println!("  • 运行 {} 重启服务器", style("cunzhi server start").cyan());
        println!("  • 检查 MCP 工具配置");
    }

    /// 显示输入相关帮助
    fn show_input_help(&self) {
        self.indicator.info("请检查输入格式并重试");
        println!("  • 运行 {} 查看帮助", style("cunzhi --help").cyan());
    }

    /// 显示权限相关帮助
    fn show_permission_help(&self) {
        self.indicator.info("尝试以下解决方案:");
        println!("  • 检查文件和目录权限");
        println!("  • 确保有足够的访问权限");
        println!("  • 考虑使用管理员权限运行");
    }

    /// 显示依赖相关帮助
    fn show_dependency_help(&self) {
        self.indicator.info("尝试以下解决方案:");
        println!("  • 检查系统依赖是否安装");
        println!("  • 更新到最新版本");
        println!("  • 查看安装文档");
    }

    /// 显示内部错误帮助
    fn show_internal_help(&self) {
        self.indicator.info("这是一个内部错误，请:");
        println!("  • 报告此问题到项目仓库");
        println!("  • 包含错误信息和操作步骤");
        println!("  • 尝试重启应用程序");
    }
}

/// 结果扩展 trait
pub trait ResultExt<T> {
    /// 添加用户友好的上下文
    fn with_user_context(self, context: &str) -> Result<T>;

    /// 转换为应用程序错误
    fn to_app_error(self, error_type: fn(&str) -> AppError) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_user_context(self, context: &str) -> Result<T> {
        self.with_context(|| context.to_string())
    }

    fn to_app_error(self, error_type: fn(&str) -> AppError) -> Result<T> {
        self.map_err(|e| error_type(&e.to_string()).into())
    }
}

/// 错误恢复策略
pub struct RecoveryStrategy {
    max_retries: u32,
    retry_delay: std::time::Duration,
}

impl RecoveryStrategy {
    pub fn new(max_retries: u32, retry_delay: std::time::Duration) -> Self {
        Self { max_retries, retry_delay }
    }

    /// 执行带重试的操作
    pub async fn execute_with_retry<F, T, E>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut() -> std::result::Result<T, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let mut attempts = 0;

        loop {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    if attempts > self.max_retries {
                        return Err(e.into());
                    }

                    let indicator = StatusIndicator::new();
                    indicator.warning(&format!("操作失败，{} 秒后重试 ({}/{})",
                        self.retry_delay.as_secs(), attempts, self.max_retries));

                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }
    }
}

/// 全局错误处理函数
pub fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        let indicator = StatusIndicator::new();
        indicator.error("程序发生严重错误");

        if let Some(location) = panic_info.location() {
            eprintln!("位置: {}:{}:{}", location.file(), location.line(), location.column());
        }

        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("错误信息: {}", message);
        }

        eprintln!("\n请报告此错误到项目仓库，并包含上述信息。");
    }));
}
