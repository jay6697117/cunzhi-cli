// 日志系统测试程序
use cunzhi_cli::utils::{auto_init_logger, ProgressIndicator, format_file_size, format_duration, print_boxed_message, confirm};
use cunzhi_cli::{log_important, log_debug, log_success, log_error, log_warning};
use std::time::Duration;
use std::thread;

fn main() -> anyhow::Result<()> {
    // 初始化日志系统
    auto_init_logger()?;

    print_boxed_message("日志系统测试", "测试 CLI 版本的日志功能和辅助工具");

    // 测试基本日志功能
    println!("\n🧪 测试基本日志功能:");
    log_important!(info, "这是一条信息日志");
    log_important!(warn, "这是一条警告日志");
    log_important!(error, "这是一条错误日志");
    log_debug!("这是一条调试日志（可能不显示，取决于日志级别）");

    // 测试 CLI 专用宏
    println!("\n🎨 测试 CLI 专用日志宏:");
    log_success!("操作成功完成");
    log_warning!("这是一个警告");
    log_error!("这是一个错误");

    // 测试进度指示器
    println!("\n⏳ 测试进度指示器:");
    let mut progress = ProgressIndicator::new("正在处理数据");
    for i in 0..20 {
        progress.tick();
        thread::sleep(Duration::from_millis(100));
        if i == 19 {
            progress.finish("数据处理完成");
        }
    }

    // 测试格式化工具
    println!("\n📊 测试格式化工具:");
    println!("文件大小格式化:");
    for size in &[0, 1023, 1024, 1536, 1048576, 1073741824] {
        println!("  {} bytes = {}", size, format_file_size(*size));
    }

    println!("\n时间格式化:");
    for millis in &[100, 1500, 65000, 3665000] {
        let duration = Duration::from_millis(*millis);
        println!("  {} ms = {}", millis, format_duration(duration));
    }

    // 测试环境检测
    println!("\n🔍 环境检测:");
    println!("  支持彩色输出: {}", cunzhi_cli::utils::supports_color());
    println!("  CI 环境: {}", cunzhi_cli::utils::is_ci_environment());
    println!("  终端环境: {}", atty::is(atty::Stream::Stderr));

    // 测试彩色文本
    println!("\n🌈 彩色文本测试:");
    use cunzhi_cli::utils::{colorize, colors};
    println!("  {}", colorize("红色文本", colors::RED));
    println!("  {}", colorize("绿色文本", colors::GREEN));
    println!("  {}", colorize("蓝色文本", colors::BLUE));
    println!("  {}", colorize("粗体文本", colors::BOLD));

    // 测试交互功能（可选）
    println!("\n💬 交互功能测试:");
    if confirm("是否测试交互功能", false) {
        if let Some(input) = cunzhi_cli::utils::prompt("请输入一些文本") {
            log_success!("您输入了: {}", input);
        } else {
            log_warning!("没有输入任何内容");
        }
    } else {
        println!("跳过交互功能测试");
    }

    print_boxed_message("测试完成", "所有日志和工具功能测试完成");

    Ok(())
}
