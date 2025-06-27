// 跨平台终端启动器模块
use anyhow::Result;
use std::process::Command;
use std::path::PathBuf;
use crate::{log_debug, log_important};

/// 终端类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalType {
    // macOS 终端
    TerminalApp,
    ITerm2,
    Alacritty,

    // Linux 终端
    GnomeTerminal,
    Konsole,
    Xterm,

    // Windows 终端
    Cmd,
    PowerShell,
    WindowsTerminal,

    // 自定义终端
    Custom(String),
}

/// 终端启动器配置
#[derive(Debug, Clone)]
pub struct TerminalLauncherConfig {
    pub preferred_terminal: Option<TerminalType>,
    pub fallback_to_cli: bool,
    pub working_directory: Option<PathBuf>,
    pub window_title: Option<String>,
}

impl Default for TerminalLauncherConfig {
    fn default() -> Self {
        Self {
            preferred_terminal: None,
            fallback_to_cli: true,
            working_directory: None,
            window_title: Some("寸止 CLI 交互".to_string()),
        }
    }
}

/// 终端启动器
pub struct TerminalLauncher {
    config: TerminalLauncherConfig,
}

impl TerminalLauncher {
    pub fn new(config: TerminalLauncherConfig) -> Self {
        Self { config }
    }

    /// 启动新的终端窗口并执行命令
    pub async fn launch_terminal_with_command(&self, command: &str, args: &[String]) -> Result<()> {
        log_debug!("尝试启动终端执行命令: {} {:?}", command, args);

        // 检测可用的终端
        let available_terminal = self.detect_best_terminal()?;
        log_important!(info, "选择终端: {:?}", available_terminal);

        // 构建终端启动命令
        let (terminal_cmd, terminal_args) = self.build_terminal_command(&available_terminal, command, args)?;

        // 启动终端
        let mut cmd = Command::new(&terminal_cmd);
        cmd.args(&terminal_args);

        // 设置工作目录
        if let Some(ref working_dir) = self.config.working_directory {
            cmd.current_dir(working_dir);
        }

        // 配置进程分离 - 避免阻塞父进程
        self.configure_process_detachment(&mut cmd, &available_terminal);

        log_debug!("执行终端命令: {} {:?}", terminal_cmd, terminal_args);

        // 尝试启动终端，带重试机制
        let result = self.try_launch_with_retry(&mut cmd, &available_terminal).await;

        match result {
            Ok(_) => {
                log_important!(info, "终端启动成功");
                Ok(())
            }
            Err(e) => {
                log_important!(warn, "终端启动失败: {}", e);
                if self.config.fallback_to_cli {
                    log_important!(info, "回退到 CLI 模式");
                    anyhow::bail!("终端启动失败，需要回退到CLI模式")
                } else {
                    anyhow::bail!("终端启动失败: {}", e)
                }
            }
        }
    }

    /// 配置进程分离
    fn configure_process_detachment(&self, cmd: &mut Command, terminal: &TerminalType) {
        use std::process::Stdio;

        // 根据终端类型配置进程分离
        match terminal {
            TerminalType::TerminalApp | TerminalType::ITerm2 => {
                // macOS 终端通过 osascript 启动，不需要特殊配置
                cmd.stdin(Stdio::null())
                   .stdout(Stdio::null())
                   .stderr(Stdio::null());
            }
            TerminalType::Cmd | TerminalType::PowerShell | TerminalType::WindowsTerminal => {
                // Windows 终端需要分离进程
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x00000008); // DETACHED_PROCESS
                }
                cmd.stdin(Stdio::null())
                   .stdout(Stdio::null())
                   .stderr(Stdio::null());
            }
            _ => {
                // Linux 终端分离进程
                cmd.stdin(Stdio::null())
                   .stdout(Stdio::null())
                   .stderr(Stdio::null());
            }
        }
    }

    /// 带重试机制的启动尝试
    async fn try_launch_with_retry(&self, cmd: &mut Command, terminal: &TerminalType) -> Result<()> {
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            log_debug!("终端启动尝试 {}/{}", attempt, max_retries);

            match cmd.spawn() {
                Ok(mut child) => {
                    // 对于某些终端，我们需要等待一小段时间确保启动成功
                    if self.should_wait_for_startup(terminal) {
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                        // 检查进程是否还在运行
                        match child.try_wait() {
                            Ok(Some(status)) if !status.success() => {
                                last_error = Some(anyhow::anyhow!("终端进程异常退出: {}", status));
                                continue;
                            }
                            Ok(None) => {
                                // 进程仍在运行，成功
                                return Ok(());
                            }
                            Ok(Some(_)) => {
                                // 进程正常退出（某些终端如 osascript 会立即退出）
                                return Ok(());
                            }
                            Err(e) => {
                                last_error = Some(anyhow::anyhow!("检查进程状态失败: {}", e));
                                continue;
                            }
                        }
                    } else {
                        // 不需要等待的终端类型
                        return Ok(());
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("启动终端失败: {}", e));
                    if attempt < max_retries {
                        log_debug!("启动失败，等待后重试: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("未知错误")))
    }

    /// 判断是否需要等待启动完成
    fn should_wait_for_startup(&self, terminal: &TerminalType) -> bool {
        matches!(terminal,
            TerminalType::GnomeTerminal |
            TerminalType::Konsole |
            TerminalType::Alacritty |
            TerminalType::Xterm
        )
    }

    /// 检测最佳可用终端
    fn detect_best_terminal(&self) -> Result<TerminalType> {
        log_debug!("开始检测可用终端...");

        // 如果配置了首选终端，先尝试
        if let Some(ref preferred) = self.config.preferred_terminal {
            log_debug!("检查首选终端: {:?}", preferred);
            if self.is_terminal_available(preferred) {
                log_debug!("首选终端可用: {:?}", preferred);
                return Ok(preferred.clone());
            } else {
                log_debug!("首选终端不可用: {:?}", preferred);
            }
        }

        // 按平台检测可用终端
        let candidates = self.get_platform_terminal_candidates();
        log_debug!("平台终端候选列表: {:?}", candidates);

        for terminal in &candidates {
            log_debug!("检查终端: {:?}", terminal);
            if self.is_terminal_available(terminal) {
                log_debug!("找到可用终端: {:?}", terminal);
                return Ok(terminal.clone());
            } else {
                log_debug!("终端不可用: {:?}", terminal);
            }
        }

        // 输出详细的诊断信息
        self.log_diagnostic_info();
        anyhow::bail!("未找到可用的终端程序。候选列表: {:?}", candidates)
    }

    /// 输出诊断信息
    fn log_diagnostic_info(&self) {
        log_debug!("=== 终端检测诊断信息 ===");
        log_debug!("操作系统: {}", std::env::consts::OS);
        log_debug!("架构: {}", std::env::consts::ARCH);

        // 环境变量信息
        if let Ok(display) = std::env::var("DISPLAY") {
            log_debug!("DISPLAY: {}", display);
        }
        if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
            log_debug!("XDG_CURRENT_DESKTOP: {}", desktop);
        }
        if let Ok(session) = std::env::var("DESKTOP_SESSION") {
            log_debug!("DESKTOP_SESSION: {}", session);
        }
        if let Ok(shell) = std::env::var("SHELL") {
            log_debug!("SHELL: {}", shell);
        }

        // 测试基本命令
        let test_commands = if cfg!(target_os = "windows") {
            vec!["where", "cmd", "powershell", "wt"]
        } else if cfg!(target_os = "macos") {
            vec!["which", "osascript", "open"]
        } else {
            vec!["which", "gnome-terminal", "konsole", "xterm"]
        };

        for cmd in test_commands {
            let available = self.test_command_exists(cmd);
            log_debug!("命令 '{}' 可用: {}", cmd, available);
        }
        log_debug!("=== 诊断信息结束 ===");
    }

    /// 获取平台对应的终端候选列表
    fn get_platform_terminal_candidates(&self) -> Vec<TerminalType> {
        if cfg!(target_os = "macos") {
            vec![
                TerminalType::ITerm2,
                TerminalType::TerminalApp,
                TerminalType::Alacritty,
            ]
        } else if cfg!(target_os = "linux") {
            vec![
                TerminalType::GnomeTerminal,
                TerminalType::Konsole,
                TerminalType::Alacritty,
                TerminalType::Xterm,
            ]
        } else if cfg!(target_os = "windows") {
            vec![
                TerminalType::WindowsTerminal,
                TerminalType::PowerShell,
                TerminalType::Cmd,
            ]
        } else {
            vec![TerminalType::Xterm] // 默认回退
        }
    }

    /// 检查终端是否可用
    pub fn is_terminal_available(&self, terminal: &TerminalType) -> bool {
        match terminal {
            TerminalType::TerminalApp => {
                // macOS Terminal.app - 检查应用程序是否存在，同时检查 osascript
                cfg!(target_os = "macos") &&
                self.test_command_exists("osascript") &&
                (std::path::Path::new("/Applications/Utilities/Terminal.app").exists() ||
                 std::path::Path::new("/System/Applications/Utilities/Terminal.app").exists())
            }
            TerminalType::ITerm2 => {
                // macOS iTerm2 - 检查应用程序是否存在，同时检查 osascript
                cfg!(target_os = "macos") &&
                self.test_command_exists("osascript") &&
                (std::path::Path::new("/Applications/iTerm.app").exists() ||
                 std::path::Path::new("/Applications/iTerm2.app").exists())
            }
            TerminalType::Alacritty => self.test_command_exists("alacritty"),
            TerminalType::GnomeTerminal => {
                // GNOME Terminal - 检查命令和环境
                cfg!(target_os = "linux") &&
                self.test_command_exists("gnome-terminal") &&
                (std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().contains("GNOME") ||
                 std::env::var("DESKTOP_SESSION").unwrap_or_default().contains("gnome"))
            }
            TerminalType::Konsole => {
                // KDE Konsole - 检查命令和环境
                cfg!(target_os = "linux") &&
                self.test_command_exists("konsole") &&
                (std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().contains("KDE") ||
                 std::env::var("DESKTOP_SESSION").unwrap_or_default().contains("kde"))
            }
            TerminalType::Xterm => {
                // Xterm - 通用 X11 终端
                (cfg!(target_os = "linux") || cfg!(target_os = "freebsd")) &&
                self.test_command_exists("xterm") &&
                std::env::var("DISPLAY").is_ok()
            }
            TerminalType::Cmd => {
                // Windows cmd - 总是可用
                cfg!(target_os = "windows")
            }
            TerminalType::PowerShell => {
                // Windows PowerShell - 检查是否存在
                if cfg!(target_os = "windows") {
                    self.test_command_exists("powershell") || self.test_command_exists("pwsh")
                } else {
                    self.test_command_exists("pwsh") // PowerShell Core on Linux/macOS
                }
            }
            TerminalType::WindowsTerminal => {
                // Windows Terminal - 检查是否存在
                cfg!(target_os = "windows") && self.test_command_exists("wt")
            }
            TerminalType::Custom(cmd) => self.test_command_exists(cmd),
        }
    }

    /// 测试命令是否存在
    pub fn test_command_exists(&self, command: &str) -> bool {
        // 首先尝试 which/where 命令检查
        let which_cmd = if cfg!(target_os = "windows") { "where" } else { "which" };

        // 使用 which/where 命令检查
        if let Ok(output) = Command::new(which_cmd)
            .arg(command)
            .output()
        {
            if output.status.success() && !output.stdout.is_empty() {
                return true;
            }
        }

        // 特殊命令处理
        match command {
            "cmd" if cfg!(target_os = "windows") => {
                // Windows cmd 总是存在
                return true;
            }
            "osascript" if cfg!(target_os = "macos") => {
                // macOS osascript 检查
                return Command::new("osascript")
                    .args(&["-e", "return 1"])
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false);
            }
            "wt" if cfg!(target_os = "windows") => {
                // Windows Terminal 特殊检查
                return Command::new("wt")
                    .args(&["--help"])
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false);
            }
            _ => {}
        }

        // 备用方案：尝试执行命令检查版本或帮助
        let version_checks = [
            vec!["--version"],
            vec!["-v"],
            vec!["--help"],
            vec!["-h"],
        ];

        for args in &version_checks {
            if let Ok(output) = Command::new(command)
                .args(args)
                .output()
            {
                if output.status.success() {
                    return true;
                }
            }
        }

        // 最后尝试：直接执行命令（某些命令可能不支持 --version 等参数）
        if let Ok(output) = Command::new(command)
            .output()
        {
            // 如果命令存在但参数错误，通常会返回非零退出码但不会是 "command not found" 错误
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
                // 检查是否是 "command not found" 类型的错误
                if !stderr.contains("not found") &&
                   !stderr.contains("not recognized") &&
                   !stderr.contains("no such file") {
                    return true;
                }
            }
        }

        false
    }

    /// 构建终端启动命令
    fn build_terminal_command(&self, terminal: &TerminalType, command: &str, args: &[String]) -> Result<(String, Vec<String>)> {
        let full_command = if args.is_empty() {
            command.to_string()
        } else {
            format!("{} {}", command, args.join(" "))
        };

        let window_title = self.config.window_title.as_deref().unwrap_or("寸止 CLI");
        let shell = self.get_default_shell();

        match terminal {
            TerminalType::TerminalApp => {
                // macOS Terminal.app - 使用 osascript 启动
                let script = format!(
                    r#"tell application "Terminal"
                        activate
                        do script "{}" in window 1
                        set custom title of front window to "{}"
                    end tell"#,
                    self.escape_applescript(&full_command),
                    self.escape_applescript(window_title)
                );
                Ok(("osascript".to_string(), vec![
                    "-e".to_string(),
                    script,
                ]))
            }
            TerminalType::ITerm2 => {
                // macOS iTerm2 - 使用 osascript 启动
                let script = format!(
                    r#"tell application "iTerm"
                        activate
                        create window with default profile
                        tell current session of current window
                            write text "{}"
                            set name to "{}"
                        end tell
                    end tell"#,
                    self.escape_applescript(&full_command),
                    self.escape_applescript(window_title)
                );
                Ok(("osascript".to_string(), vec![
                    "-e".to_string(),
                    script,
                ]))
            }
            TerminalType::Alacritty => {
                // Alacritty - 跨平台终端
                Ok(("alacritty".to_string(), vec![
                    "--title".to_string(),
                    window_title.to_string(),
                    "-e".to_string(),
                    shell.clone(),
                    "-c".to_string(),
                    format!("{}; exec {}", full_command, shell),
                ]))
            }
            TerminalType::GnomeTerminal => {
                // GNOME Terminal - 新版本语法
                Ok(("gnome-terminal".to_string(), vec![
                    "--title".to_string(),
                    window_title.to_string(),
                    "--".to_string(),
                    shell.clone(),
                    "-c".to_string(),
                    format!("{}; exec {}", full_command, shell),
                ]))
            }
            TerminalType::Konsole => {
                // KDE Konsole
                Ok(("konsole".to_string(), vec![
                    "--new-tab".to_string(),
                    "-p".to_string(),
                    format!("tabtitle={}", window_title),
                    "-e".to_string(),
                    shell.clone(),
                    "-c".to_string(),
                    format!("{}; exec {}", full_command, shell),
                ]))
            }
            TerminalType::Xterm => {
                // Xterm - 经典终端
                Ok(("xterm".to_string(), vec![
                    "-T".to_string(),
                    window_title.to_string(),
                    "-e".to_string(),
                    shell.clone(),
                    "-c".to_string(),
                    format!("{}; exec {}", full_command, shell),
                ]))
            }
            TerminalType::Cmd => {
                // Windows Command Prompt
                Ok(("cmd".to_string(), vec![
                    "/C".to_string(),
                    "start".to_string(),
                    format!("\"{}\"", window_title),
                    "cmd".to_string(),
                    "/K".to_string(),
                    format!("title {} && {}", window_title, full_command),
                ]))
            }
            TerminalType::PowerShell => {
                // Windows PowerShell
                let ps_command = format!(
                    "$Host.UI.RawUI.WindowTitle = '{}'; {}; Read-Host 'Press Enter to continue'",
                    window_title, full_command
                );
                Ok(("powershell".to_string(), vec![
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    format!("Start-Process powershell -ArgumentList '-NoProfile', '-Command', \"{}\"",
                           self.escape_powershell(&ps_command)),
                ]))
            }
            TerminalType::WindowsTerminal => {
                // Windows Terminal
                Ok(("wt".to_string(), vec![
                    "new-tab".to_string(),
                    "--title".to_string(),
                    window_title.to_string(),
                    "--".to_string(),
                    "cmd".to_string(),
                    "/K".to_string(),
                    format!("title {} && {}", window_title, full_command),
                ]))
            }
            TerminalType::Custom(cmd) => {
                // 自定义终端 - 简单执行
                Ok((cmd.clone(), vec![full_command]))
            }
        }
    }

    /// 获取默认 shell
    fn get_default_shell(&self) -> String {
        if cfg!(target_os = "windows") {
            "cmd".to_string()
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
        }
    }

    /// 转义 AppleScript 字符串
    fn escape_applescript(&self, text: &str) -> String {
        text.replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
    }

    /// 转义 PowerShell 字符串
    fn escape_powershell(&self, text: &str) -> String {
        text.replace("\"", "`\"")
            .replace("'", "`'")
            .replace("`", "``")
            .replace("$", "`$")
    }
}

/// 快捷函数：使用默认配置启动终端
pub async fn launch_terminal_with_default_config(command: &str, args: &[String]) -> Result<()> {
    let launcher = TerminalLauncher::new(TerminalLauncherConfig::default());
    launcher.launch_terminal_with_command(command, args).await
}

/// 快捷函数：检测当前系统的最佳终端
pub fn detect_system_terminal() -> Result<TerminalType> {
    let launcher = TerminalLauncher::new(TerminalLauncherConfig::default());
    launcher.detect_best_terminal()
}

/// 诊断终端可用性问题
pub fn diagnose_terminal_availability() -> String {
    let launcher = TerminalLauncher::new(TerminalLauncherConfig::default());
    let mut report = String::new();

    report.push_str("=== 终端可用性诊断报告 ===\n");
    report.push_str(&format!("操作系统: {}\n", std::env::consts::OS));
    report.push_str(&format!("架构: {}\n", std::env::consts::ARCH));

    // 环境变量
    report.push_str("\n--- 环境变量 ---\n");
    let env_vars = ["DISPLAY", "XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "SHELL", "TERM"];
    for var in &env_vars {
        if let Ok(value) = std::env::var(var) {
            report.push_str(&format!("{}: {}\n", var, value));
        } else {
            report.push_str(&format!("{}: (未设置)\n", var));
        }
    }

    // 检查所有终端类型
    report.push_str("\n--- 终端可用性检查 ---\n");
    let all_terminals = vec![
        TerminalType::TerminalApp,
        TerminalType::ITerm2,
        TerminalType::Alacritty,
        TerminalType::GnomeTerminal,
        TerminalType::Konsole,
        TerminalType::Xterm,
        TerminalType::Cmd,
        TerminalType::PowerShell,
        TerminalType::WindowsTerminal,
    ];

    for terminal in &all_terminals {
        let available = launcher.is_terminal_available(terminal);
        report.push_str(&format!("{:?}: {}\n", terminal, if available { "✓ 可用" } else { "✗ 不可用" }));
    }

    // 平台候选列表
    report.push_str("\n--- 平台终端候选列表 ---\n");
    let candidates = launcher.get_platform_terminal_candidates();
    for (i, terminal) in candidates.iter().enumerate() {
        let available = launcher.is_terminal_available(terminal);
        report.push_str(&format!("{}. {:?}: {}\n", i + 1, terminal, if available { "✓" } else { "✗" }));
    }

    // 检测结果
    report.push_str("\n--- 检测结果 ---\n");
    match launcher.detect_best_terminal() {
        Ok(terminal) => {
            report.push_str(&format!("推荐终端: {:?}\n", terminal));
        }
        Err(e) => {
            report.push_str(&format!("检测失败: {}\n", e));
        }
    }

    report.push_str("=== 诊断报告结束 ===\n");
    report
}
