// 交互式界面实现 - 类似 create-vue 的体验
use anyhow::Result;
use inquire::{Confirm, Text};
use crate::config::{AppConfig, ReplyConfig, McpConfig, load_standalone_config, save_standalone_config};
use crate::utils::{print_boxed_message, colorize, colors, ProgressIndicator};
use crate::log_success;
use std::collections::HashMap;

pub async fn run_init_wizard(name: Option<String>, yes: bool) -> Result<()> {
    if yes {
        println!("🚀 使用默认配置快速初始化");
        quick_init(name).await
    } else {
        println!("🚀 欢迎使用寸止 CLI 初始化向导");
        interactive_init(name).await
    }
}

async fn quick_init(name: Option<String>) -> Result<()> {
    let project_name = name.unwrap_or_else(|| "cunzhi-project".to_string());

    print_boxed_message(
        "快速初始化",
        &format!("为项目 '{}' 创建默认配置", project_name)
    );

    // 创建默认配置
    let default_config = crate::config::AppConfig::default();
    crate::config::save_standalone_config(&default_config)?;

    log_success!("默认配置已创建");
    println!("运行 {} 查看配置", colorize("cunzhi config show", colors::CYAN));

    Ok(())
}

async fn interactive_init(name: Option<String>) -> Result<()> {
    print_boxed_message(
        "🚀 寸止 CLI 配置向导",
        "让我们一起配置您的智能代码审查工具"
    );

    if let Some(ref project_name) = name {
        println!("📁 项目名称: {}", colorize(project_name, colors::GREEN));
        println!();
    }

    // 欢迎信息
    println!("{}",colorize("欢迎使用寸止 CLI！", colors::BOLD));
    println!("我们将引导您完成初始配置，这只需要几分钟时间。");
    println!();

    // 检查是否存在现有配置
    let existing_config = load_standalone_config().ok();
    if existing_config.is_some() {
        let overwrite = Confirm::new("检测到现有配置文件，是否要重新配置？")
            .with_default(false)
            .prompt()?;

        if !overwrite {
            log_success!("保持现有配置不变");
            return Ok(());
        }
    }

    // 开始配置向导
    let config = run_configuration_wizard().await?;

    // 保存配置
    let mut progress = ProgressIndicator::new("正在保存配置");
    for _ in 0..10 {
        progress.tick();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    save_standalone_config(&config)?;
    progress.finish("配置保存成功！");

    // 显示配置摘要
    display_configuration_summary(&config);

    // 下一步提示
    println!("\n🎉 配置完成！您现在可以：");
    println!("  {} - 查看完整配置", colorize("cunzhi config show", colors::CYAN));
    // Telegram 功能已移除
    println!("  {} - 启动 MCP 服务器", colorize("cunzhi server start", colors::CYAN));

    Ok(())
}

/// 运行配置向导
async fn run_configuration_wizard() -> Result<AppConfig> {
    println!("{}", colorize("📋 配置向导", colors::BOLD));
    println!("我们将逐步配置各个功能模块。\n");

    // 1. MCP 工具配置
    let mcp_config = configure_mcp_tools().await?;

    // 2. 回复配置（Telegram 功能已移除）
    let reply_config = configure_reply_settings().await?;

    // 构建最终配置
    let config = AppConfig {
        reply_config,
        mcp_config,
        terminal_config: crate::config::default_terminal_config(),
        // Telegram 功能已移除
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(config)
}

/// 配置 MCP 工具
async fn configure_mcp_tools() -> Result<McpConfig> {
    println!("{}", colorize("🔧 MCP 工具配置", colors::CYAN));
    println!("MCP (Model Context Protocol) 工具提供智能代码审查功能。");
    println!();

    let available_tools = vec![
        ("zhi", "智能代码审查工具 - 提供代码质量分析和建议"),
        ("ji", "记忆管理工具 - 管理项目知识和最佳实践"),
    ];

    let mut tools = HashMap::new();

    for (tool_name, description) in available_tools {
        println!("🛠️  {}: {}", colorize(tool_name, colors::GREEN), description);
        let enabled = Confirm::new(&format!("是否启用 {} 工具？", tool_name))
            .with_default(true)
            .prompt()?;

        tools.insert(tool_name.to_string(), enabled);

        if enabled {
            println!("   ✅ {} 已启用", tool_name);
        } else {
            println!("   ⏸️  {} 已禁用", tool_name);
        }
        println!();
    }

    Ok(McpConfig { tools })
}

// Telegram 功能已移除

/// 配置回复设置
async fn configure_reply_settings() -> Result<ReplyConfig> {
    println!("{}", colorize("💬 回复设置配置", colors::CYAN));
    println!("配置自动继续回复功能的行为。");
    println!();

    let enable_continue_reply = Confirm::new("是否启用自动继续回复？")
        .with_default(true)
        .with_help_message("当回复内容过长时自动继续生成")
        .prompt()?;

    let auto_continue_threshold = if enable_continue_reply {
        let threshold_str = Text::new("自动继续阈值（字符数）:")
            .with_default("1000")
            .with_help_message("当回复超过此字符数时触发自动继续")
            .prompt()?;

        threshold_str.parse::<u32>().unwrap_or(1000)
    } else {
        1000
    };

    let continue_prompt = Text::new("继续提示词:")
        .with_default("请按照最佳实践继续")
        .with_help_message("触发自动继续时使用的提示词")
        .prompt()?;

    if enable_continue_reply {
        println!("   ✅ 自动继续回复已启用（阈值: {} 字符）", auto_continue_threshold);
    } else {
        println!("   ⏸️  自动继续回复已禁用");
    }

    Ok(ReplyConfig {
        enable_continue_reply,
        auto_continue_threshold,
        continue_prompt,
    })
}

/// 显示配置摘要
fn display_configuration_summary(config: &AppConfig) {
    println!("\n{}", colorize("📋 配置摘要", colors::BOLD));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // MCP 工具
    println!("🔧 MCP 工具:");
    for (tool, enabled) in &config.mcp_config.tools {
        let status = if *enabled { "✅ 启用" } else { "⏸️  禁用" };
        println!("   {} - {}", tool, status);
    }

    // Telegram 功能已移除
    println!("\n📱 Telegram Bot:");
    println!("   ⏸️  已移除（功能已禁用）");

    // 回复设置
    println!("\n💬 回复设置:");
    if config.reply_config.enable_continue_reply {
        println!("   ✅ 自动继续回复已启用");
        println!("   📏 阈值: {} 字符", config.reply_config.auto_continue_threshold);
    } else {
        println!("   ⏸️  自动继续回复已禁用");
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
