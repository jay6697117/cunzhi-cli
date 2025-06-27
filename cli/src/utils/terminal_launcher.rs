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

        log_debug!("执行终端命令: {} {:?}", terminal_cmd, terminal_args);

        let output = cmd.spawn();

        match output {
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

    /// 检测最佳可用终端
    fn detect_best_terminal(&self) -> Result<TerminalType> {
        // 如果配置了首选终端，先尝试
        if let Some(ref preferred) = self.config.preferred_terminal {
            if self.is_terminal_available(preferred) {
                return Ok(preferred.clone());
            }
        }

        // 按平台检测可用终端
        let candidates = self.get_platform_terminal_candidates();

        for terminal in candidates {
            if self.is_terminal_available(&terminal) {
                return Ok(terminal);
            }
        }

        anyhow::bail!("未找到可用的终端程序")
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
        let command = match terminal {
            TerminalType::TerminalApp => {
                // macOS Terminal.app 通过 open 命令启动
                return Command::new("open")
                    .args(&["-a", "Terminal"])
                    .arg("--")
                    .output()
                    .map(|out| out.status.success())
                    .unwrap_or(false);
            }
            TerminalType::ITerm2 => "iterm",
            TerminalType::Alacritty => "alacritty",
            TerminalType::GnomeTerminal => "gnome-terminal",
            TerminalType::Konsole => "konsole",
            TerminalType::Xterm => "xterm",
            TerminalType::Cmd => "cmd",
            TerminalType::PowerShell => "powershell",
            TerminalType::WindowsTerminal => "wt",
            TerminalType::Custom(cmd) => cmd.as_str(),
        };

        // 测试命令是否存在
        self.test_command_exists(command)
    }

    /// 测试命令是否存在
    pub fn test_command_exists(&self, command: &str) -> bool {
        Command::new(command)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false) ||
        // 某些终端可能不支持 --version，尝试 -v
        Command::new(command)
            .arg("-v")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false) ||
        // Windows cmd 特殊处理
        (command == "cmd" && Command::new("cmd")
            .args(&["/C", "echo"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false))
    }

    /// 构建终端启动命令
    fn build_terminal_command(&self, terminal: &TerminalType, command: &str, args: &[String]) -> Result<(String, Vec<String>)> {
        let full_command = if args.is_empty() {
            command.to_string()
        } else {
            format!("{} {}", command, args.join(" "))
        };

        let window_title = self.config.window_title.as_deref().unwrap_or("寸止 CLI");

        match terminal {
            TerminalType::TerminalApp => {
                Ok(("open".to_string(), vec![
                    "-a".to_string(),
                    "Terminal".to_string(),
                    "--args".to_string(),
                    "-T".to_string(),
                    window_title.to_string(),
                    "-e".to_string(),
                    "bash".to_string(),
                    "-c".to_string(),
                    full_command,
                ]))
            }
            TerminalType::ITerm2 => {
                Ok(("open".to_string(), vec![
                    "-a".to_string(),
                    "iTerm".to_string(),
                    "--args".to_string(),
                    "-e".to_string(),
                    "bash".to_string(),
                    "-c".to_string(),
                    full_command,
                ]))
            }
            TerminalType::Alacritty => {
                Ok(("alacritty".to_string(), vec![
                    "-t".to_string(),
                    window_title.to_string(),
                    "-e".to_string(),
                    "bash".to_string(),
                    "-c".to_string(),
                    full_command,
                ]))
            }
            TerminalType::GnomeTerminal => {
                Ok(("gnome-terminal".to_string(), vec![
                    "--title".to_string(),
                    window_title.to_string(),
                    "--".to_string(),
                    "bash".to_string(),
                    "-c".to_string(),
                    full_command,
                ]))
            }
            TerminalType::Konsole => {
                Ok(("konsole".to_string(), vec![
                    "-p".to_string(),
                    format!("tabtitle={}", window_title),
                    "-e".to_string(),
                    "bash".to_string(),
                    "-c".to_string(),
                    full_command,
                ]))
            }
            TerminalType::Xterm => {
                Ok(("xterm".to_string(), vec![
                    "-T".to_string(),
                    window_title.to_string(),
                    "-e".to_string(),
                    "bash".to_string(),
                    "-c".to_string(),
                    full_command,
                ]))
            }
            TerminalType::Cmd => {
                Ok(("cmd".to_string(), vec![
                    "/K".to_string(),
                    format!("title {} && {}", window_title, full_command),
                ]))
            }
            TerminalType::PowerShell => {
                Ok(("powershell".to_string(), vec![
                    "-NoExit".to_string(),
                    "-Command".to_string(),
                    format!("$Host.UI.RawUI.WindowTitle = '{}'; {}", window_title, full_command),
                ]))
            }
            TerminalType::WindowsTerminal => {
                Ok(("wt".to_string(), vec![
                    "new-tab".to_string(),
                    "--title".to_string(),
                    window_title.to_string(),
                    "cmd".to_string(),
                    "/K".to_string(),
                    full_command,
                ]))
            }
            TerminalType::Custom(cmd) => {
                Ok((cmd.clone(), vec![full_command]))
            }
        }
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
