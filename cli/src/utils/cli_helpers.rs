// CLI 辅助工具函数
use std::io::{self, Write};
use std::time::Duration;

/// 显示进度指示器
pub struct ProgressIndicator {
    message: String,
    spinner_chars: Vec<char>,
    current_char: usize,
}

impl ProgressIndicator {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            spinner_chars: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            current_char: 0,
        }
    }

    pub fn tick(&mut self) {
        if atty::is(atty::Stream::Stderr) {
            print!("\r{} {}", self.spinner_chars[self.current_char], self.message);
            io::stdout().flush().unwrap_or(());
            self.current_char = (self.current_char + 1) % self.spinner_chars.len();
        }
    }

    pub fn finish(&self, success_message: &str) {
        if atty::is(atty::Stream::Stderr) {
            print!("\r");
            io::stdout().flush().unwrap_or(());
        }
        crate::log_success!("{}", success_message);
    }

    pub fn fail(&self, error_message: &str) {
        if atty::is(atty::Stream::Stderr) {
            print!("\r");
            io::stdout().flush().unwrap_or(());
        }
        crate::log_error!("{}", error_message);
    }
}

/// 格式化文件大小
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// 格式化持续时间
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis();

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}.{}s", seconds, millis / 100)
    } else {
        format!("{}ms", millis)
    }
}

/// 检查是否在 CI 环境中
pub fn is_ci_environment() -> bool {
    std::env::var("CI").is_ok() ||
    std::env::var("GITHUB_ACTIONS").is_ok() ||
    std::env::var("GITLAB_CI").is_ok() ||
    std::env::var("JENKINS_URL").is_ok()
}

/// 检查是否支持彩色输出
pub fn supports_color() -> bool {
    atty::is(atty::Stream::Stderr) &&
    std::env::var("NO_COLOR").is_err() &&
    !is_ci_environment()
}

/// 彩色文本辅助函数
pub fn colorize(text: &str, color: &str) -> String {
    if supports_color() {
        format!("{}{}\x1b[0m", color, text)
    } else {
        text.to_string()
    }
}

/// 常用颜色常量
pub mod colors {
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const GRAY: &str = "\x1b[90m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
}

/// 带样式的彩色文本辅助函数
pub fn colorize_with_style(text: &str, color: &str, bold: bool) -> String {
    if supports_color() {
        let style = if bold { colors::BOLD } else { "" };
        format!("{}{}{}\x1b[0m", style, color, text)
    } else {
        text.to_string()
    }
}

/// 打印带边框的消息
pub fn print_boxed_message(title: &str, message: &str) {
    let lines: Vec<&str> = message.lines().collect();
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0).max(title.len());
    let box_width = max_width + 4;

    println!("┌{}┐", "─".repeat(box_width - 2));
    println!("│ {}{} │", title, " ".repeat(max_width - title.len()));
    if !message.is_empty() {
        println!("├{}┤", "─".repeat(box_width - 2));
        for line in lines {
            println!("│ {}{} │", line, " ".repeat(max_width - line.len()));
        }
    }
    println!("└{}┘", "─".repeat(box_width - 2));
}

/// 确认提示
pub fn confirm(message: &str, default: bool) -> bool {
    let default_text = if default { "[Y/n]" } else { "[y/N]" };
    print!("{} {}: ", message, default_text);
    io::stdout().flush().unwrap_or(());

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim().to_lowercase();
            match input.as_str() {
                "y" | "yes" => true,
                "n" | "no" => false,
                "" => default,
                _ => default,
            }
        }
        Err(_) => default,
    }
}

/// 简单的输入提示
pub fn prompt(message: &str) -> Option<String> {
    print!("{}: ", message);
    io::stdout().flush().unwrap_or(());

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            if input.is_empty() {
                None
            } else {
                Some(input.to_string())
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1023), "1023 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(Duration::from_secs(1)), "1.0s");
        assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
        assert_eq!(format_duration(Duration::from_secs(3665)), "1h 1m 5s");
    }
}
