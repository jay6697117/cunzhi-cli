// 用户体验和错误处理测试程序
use anyhow::Result;
use cunzhi_cli::utils::{
    ModernProgressBar, StatusIndicator, TaskProgressManager, Table,
    ConfirmDialog, ErrorHandler, AppError, utils
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // 设置 panic 处理器
    cunzhi_cli::utils::setup_panic_handler();

    println!("🎨 用户体验和错误处理测试\n");

    // 测试状态指示器
    test_status_indicators().await;

    // 测试进度条
    test_progress_bars().await;

    // 测试多任务进度管理
    test_multi_task_progress().await;

    // 测试表格显示
    test_table_display().await;

    // 测试确认对话框
    test_confirm_dialog().await;

    // 测试错误处理
    test_error_handling().await;

    // 测试工具函数
    test_utility_functions().await;

    println!("\n✅ 所有用户体验测试完成！");
    Ok(())
}

async fn test_status_indicators() {
    println!("🧪 测试状态指示器:");
    let indicator = StatusIndicator::new();

    indicator.success("这是一个成功消息");
    indicator.error("这是一个错误消息");
    indicator.warning("这是一个警告消息");
    indicator.info("这是一个信息消息");

    for i in 1..=3 {
        indicator.step(i, 3, &format!("执行步骤 {}", i));
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    println!();
}

async fn test_progress_bars() {
    println!("🧪 测试进度条:");

    // 测试旋转器
    println!("  测试旋转器:");
    let spinner = ModernProgressBar::new_spinner();
    spinner.set_message("正在处理数据...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    tokio::time::sleep(Duration::from_millis(2000)).await;
    spinner.finish_with_message("数据处理完成！");

    // 测试进度条
    println!("  测试进度条:");
    let pb = ModernProgressBar::new(100);
    pb.set_message("正在下载文件...");

    for i in 0..=100 {
        pb.set_position(i);
        if i % 20 == 0 {
            pb.set_message(&format!("正在下载文件... {}%", i));
        }
        tokio::time::sleep(Duration::from_millis(30)).await;
    }

    pb.finish_with_message("文件下载完成！");
    println!();
}

async fn test_multi_task_progress() {
    println!("🧪 测试多任务进度管理:");

    let mut manager = TaskProgressManager::new();

    let task1 = manager.add_task("任务1", 50);
    let task2 = manager.add_task("任务2", 30);
    let task3 = manager.add_task("任务3", 80);

    // 模拟并发任务执行
    for i in 0..=50 {
        if i <= 30 {
            manager.update_task(task2, i as u64, "处理数据");
        }
        if i <= 50 {
            manager.update_task(task1, i as u64, "编译代码");
        }
        if i <= 80 {
            let progress = (i as f64 * 1.6) as u64;
            manager.update_task(task3, progress.min(80), "运行测试");
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    manager.finish_task(task2, "数据处理完成");
    manager.finish_task(task1, "代码编译完成");
    manager.finish_task(task3, "测试运行完成");

    manager.join();
    println!();
}

async fn test_table_display() {
    println!("🧪 测试表格显示:");

    let mut table = Table::new(vec!["工具", "状态", "描述"]);
    table.add_row(vec!["zhi", "✅ 启用", "智能代码审查工具"]);
    table.add_row(vec!["ji", "✅ 启用", "记忆管理工具"]);
    table.add_row(vec!["telegram", "❌ 禁用", "Telegram 集成"]);

    table.print();
    println!();
}

async fn test_confirm_dialog() {
    println!("🧪 测试确认对话框:");

    let _dialog = ConfirmDialog::new();

    // 在测试环境中，我们跳过实际的用户输入
    println!("  模拟确认对话框（在实际使用中会等待用户输入）");
    println!("  ? 是否继续执行操作？ [Y/n]: y");
    println!("  > 用户选择: 是");

    println!();
}

async fn test_error_handling() {
    println!("🧪 测试错误处理:");

    let handler = ErrorHandler::new(true);

    // 测试不同类型的应用程序错误
    let test_cases = vec![
        ("配置文件格式错误", "config"),
        ("无法写入文件", "file_operation"),
        ("连接超时", "network"),
        ("服务器启动失败", "mcp_server"),
        ("无效的项目名称", "user_input"),
        ("权限不足", "permission"),
        ("缺少依赖库", "dependency"),
        ("内部处理错误", "internal"),
    ];

    for (i, (msg, error_type)) in test_cases.iter().enumerate() {
        println!("  测试错误类型 {}:", i + 1);
        let error = match *error_type {
            "config" => AppError::config(msg),
            "file_operation" => AppError::file_operation(msg),
            "network" => AppError::network(msg),
            "mcp_server" => AppError::mcp_server(msg),
            "user_input" => AppError::user_input(msg),
            "permission" => AppError::permission(msg),
            "dependency" => AppError::dependency(msg),
            "internal" => AppError::internal(msg),
            _ => AppError::internal(msg),
        };
        let error_result: anyhow::Error = error.into();
        let exit_code = handler.handle_error(&error_result);
        println!("    退出码: {}\n", exit_code);

        // 添加延迟以便观察输出
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

async fn test_utility_functions() {
    println!("🧪 测试工具函数:");

    // 测试加载动画
    println!("  测试加载动画:");
    let result = utils::show_loading("正在执行任务", Duration::from_secs(1), || {
        std::thread::sleep(Duration::from_millis(1000));
        "任务完成"
    });
    println!("  结果: {}", result);

    // 测试模拟进度
    println!("  测试模拟进度:");
    utils::simulate_progress("正在安装依赖", 20);

    println!();
}

// 测试错误恢复策略
#[allow(dead_code)]
async fn test_recovery_strategy() {
    use cunzhi_cli::utils::RecoveryStrategy;

    println!("🧪 测试错误恢复策略:");

    let strategy = RecoveryStrategy::new(3, Duration::from_millis(500));

    let mut attempt_count = 0;
    let result = strategy.execute_with_retry(|| {
        attempt_count += 1;
        if attempt_count < 3 {
            Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "模拟失败"))
        } else {
            Ok("成功!")
        }
    }).await;

    match result {
        Ok(msg) => println!("  恢复成功: {}", msg),
        Err(e) => println!("  恢复失败: {}", e),
    }

    println!();
}
