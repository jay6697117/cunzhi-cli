// 现代化 UI 工具模块 - 使用 console 和 indicatif
use console::{style, Emoji, Term};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::time::Duration;
use std::thread;

/// 表情符号常量
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨", ":-)");
pub static ROCKET: Emoji<'_, '_> = Emoji("🚀", ">>>");
pub static GEAR: Emoji<'_, '_> = Emoji("⚙️", "[*]");
pub static CHECK: Emoji<'_, '_> = Emoji("✅", "[+]");
pub static CROSS: Emoji<'_, '_> = Emoji("❌", "[x]");
pub static WARNING: Emoji<'_, '_> = Emoji("⚠️", "[!]");
pub static INFO: Emoji<'_, '_> = Emoji("ℹ️", "[i]");
pub static FOLDER: Emoji<'_, '_> = Emoji("📁", "[D]");
pub static FILE: Emoji<'_, '_> = Emoji("📄", "[F]");
pub static WRENCH: Emoji<'_, '_> = Emoji("🔧", "[T]");

/// 颜色主题
pub mod theme {
    use console::Color;

    pub const PRIMARY: Color = Color::Cyan;
    pub const SUCCESS: Color = Color::Green;
    pub const ERROR: Color = Color::Red;
    pub const WARNING: Color = Color::Yellow;
    pub const INFO: Color = Color::Blue;
    pub const MUTED: Color = Color::Color256(8);
    pub const ACCENT: Color = Color::Magenta;
}

/// 现代化进度条
pub struct ModernProgressBar {
    pb: ProgressBar,
    multi: Option<MultiProgress>,
}

impl ModernProgressBar {
    /// 创建新的进度条
    pub fn new(len: u64) -> Self {
        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        Self { pb, multi: None }
    }

    /// 创建不确定长度的进度条（旋转器）
    pub fn new_spinner() -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        Self { pb, multi: None }
    }

    /// 设置消息
    pub fn set_message(&self, msg: &str) {
        self.pb.set_message(msg.to_string());
    }

    /// 增加进度
    pub fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }

    /// 设置位置
    pub fn set_position(&self, pos: u64) {
        self.pb.set_position(pos);
    }

    /// 完成进度条
    pub fn finish_with_message(&self, msg: &str) {
        self.pb.finish_with_message(msg.to_string());
    }

    /// 启用稳定的 tick
    pub fn enable_steady_tick(&self, interval: Duration) {
        self.pb.enable_steady_tick(interval);
    }
}

/// 多任务进度管理器
pub struct TaskProgressManager {
    multi: MultiProgress,
    tasks: Vec<ProgressBar>,
}

impl TaskProgressManager {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
            tasks: Vec::new(),
        }
    }

    /// 添加新任务
    pub fn add_task(&mut self, name: &str, len: u64) -> usize {
        let pb = self.multi.add(ProgressBar::new(len));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!("{{spinner:.green}} {} [{{wide_bar:.cyan/blue}}] {{pos}}/{{len}} {{msg}}", name))
                .unwrap()
                .progress_chars("#>-")
        );
        self.tasks.push(pb);
        self.tasks.len() - 1
    }

    /// 更新任务进度
    pub fn update_task(&self, task_id: usize, pos: u64, msg: &str) {
        if let Some(pb) = self.tasks.get(task_id) {
            pb.set_position(pos);
            pb.set_message(msg.to_string());
        }
    }

    /// 完成任务
    pub fn finish_task(&self, task_id: usize, msg: &str) {
        if let Some(pb) = self.tasks.get(task_id) {
            pb.finish_with_message(msg.to_string());
        }
    }

    /// 等待所有任务完成
    pub fn join(&self) {
        // indicatif 的 MultiProgress 没有 join 方法，这里简化处理
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// 状态指示器
pub struct StatusIndicator {
    term: Term,
}

impl StatusIndicator {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    /// 显示成功消息
    pub fn success(&self, msg: &str) {
        let styled = format!("{} {}",
            style(CHECK).green(),
            style(msg).green()
        );
        self.term.write_line(&styled).unwrap_or(());
    }

    /// 显示错误消息
    pub fn error(&self, msg: &str) {
        let styled = format!("{} {}",
            style(CROSS).red(),
            style(msg).red()
        );
        self.term.write_line(&styled).unwrap_or(());
    }

    /// 显示警告消息
    pub fn warning(&self, msg: &str) {
        let styled = format!("{} {}",
            style(WARNING).yellow(),
            style(msg).yellow()
        );
        self.term.write_line(&styled).unwrap_or(());
    }

    /// 显示信息消息
    pub fn info(&self, msg: &str) {
        let styled = format!("{} {}",
            style(INFO).blue(),
            style(msg).blue()
        );
        self.term.write_line(&styled).unwrap_or(());
    }

    /// 显示步骤消息
    pub fn step(&self, step: u32, total: u32, msg: &str) {
        let styled = format!("{} [{}/{}] {}",
            style(GEAR).cyan(),
            style(step).bold(),
            style(total).bold(),
            style(msg).cyan()
        );
        self.term.write_line(&styled).unwrap_or(());
    }
}

/// 交互式确认对话框
pub struct ConfirmDialog {
    term: Term,
}

impl ConfirmDialog {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    /// 显示确认对话框
    pub fn confirm(&self, message: &str, default: bool) -> bool {
        let default_text = if default { "Y/n" } else { "y/N" };
        let prompt = format!("{} {} [{}]: ",
            style("?").blue().bold(),
            style(message).bold(),
            style(default_text).dim()
        );

        loop {
            self.term.write_str(&prompt).unwrap_or(());

            if let Ok(input) = self.term.read_line() {
                let input = input.trim().to_lowercase();
                match input.as_str() {
                    "y" | "yes" => return true,
                    "n" | "no" => return false,
                    "" => return default,
                    _ => {
                        let warning_msg = format!("{} 请输入 y/yes 或 n/no",
                            console::style("⚠️").yellow());
                        self.term.write_line(&warning_msg).unwrap_or(());
                        continue;
                    }
                }
            }
        }
    }
}

/// 美化的表格显示
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    max_widths: Vec<usize>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        let headers: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
        let max_widths = headers.iter().map(|h| h.len()).collect();

        Self {
            headers,
            rows: Vec::new(),
            max_widths,
        }
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        let row: Vec<String> = row.iter().map(|c| c.to_string()).collect();

        // 更新最大宽度
        for (i, cell) in row.iter().enumerate() {
            if i < self.max_widths.len() {
                self.max_widths[i] = self.max_widths[i].max(cell.len());
            }
        }

        self.rows.push(row);
    }

    pub fn print(&self) {
        let term = Term::stdout();

        // 打印表头
        let header_line = self.headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:width$}", h, width = self.max_widths[i]))
            .collect::<Vec<_>>()
            .join(" │ ");

        term.write_line(&style(format!("│ {} │", header_line)).cyan().to_string()).unwrap_or(());

        // 打印分隔线
        let separator = self.max_widths
            .iter()
            .map(|&w| "─".repeat(w))
            .collect::<Vec<_>>()
            .join("─┼─");

        term.write_line(&style(format!("├─{}─┤", separator)).cyan().to_string()).unwrap_or(());

        // 打印数据行
        for row in &self.rows {
            let row_line = row
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let width = if i < self.max_widths.len() { self.max_widths[i] } else { c.len() };
                    format!("{:width$}", c, width = width)
                })
                .collect::<Vec<_>>()
                .join(" │ ");

            term.write_line(&format!("│ {} │", row_line)).unwrap_or(());
        }
    }
}

/// 工具函数
pub mod utils {
    use super::*;

    /// 显示加载动画
    pub fn show_loading<F, R>(message: &str, _duration: Duration, task: F) -> R
    where
        F: FnOnce() -> R,
    {
        let pb = ModernProgressBar::new_spinner();
        pb.set_message(message);
        pb.enable_steady_tick(Duration::from_millis(100));

        let result = task();

        pb.finish_with_message(&format!("{} {}", CHECK, "完成"));
        result
    }

    /// 模拟进度任务
    pub fn simulate_progress(message: &str, steps: u64) {
        let pb = ModernProgressBar::new(steps);
        pb.set_message(message);

        for _i in 0..steps {
            thread::sleep(Duration::from_millis(100));
            pb.inc(1);
        }

        pb.finish_with_message(&format!("{} 任务完成", CHECK));
    }
}
