// 项目初始化向导 - 类似 create-vue 的体验
use anyhow::Result;
use inquire::{Confirm, Select, Text, MultiSelect};
use std::collections::HashMap;
use std::path::Path;

use crate::config::{AppConfig, ReplyConfig, McpConfig, save_standalone_config};
use crate::utils::{print_boxed_message, colorize, colors, ModernProgressBar, StatusIndicator, theme};
use console;
use crate::{log_success, log_warning};

/// 项目模板类型
#[derive(Debug, Clone)]
pub struct ProjectTemplate {
    pub name: String,
    pub description: String,
    pub mcp_tools: Vec<String>,
    pub enable_continue_reply: bool,
    pub continue_threshold: u32,
}

impl ProjectTemplate {
    pub fn get_templates() -> Vec<Self> {
        vec![
            Self {
                name: "基础配置".to_string(),
                description: "最小化配置，只启用核心功能".to_string(),
                mcp_tools: vec!["zhi".to_string()],
                enable_continue_reply: false,
                continue_threshold: 1000,
            },
            Self {
                name: "完整配置".to_string(),
                description: "启用所有功能，适合完整的代码审查工作流".to_string(),
                mcp_tools: vec!["zhi".to_string(), "ji".to_string()],
                enable_continue_reply: true,
                continue_threshold: 1000,
            },
            Self {
                name: "自定义配置".to_string(),
                description: "手动选择需要的功能和配置".to_string(),
                mcp_tools: vec![],
                enable_continue_reply: true,
                continue_threshold: 1000,
            },
        ]
    }
}

/// 运行项目初始化向导
pub async fn run_project_init_wizard(project_name: Option<String>, use_defaults: bool) -> Result<()> {
    // 显示欢迎界面
    show_welcome_screen(&project_name);

    if use_defaults {
        return quick_setup(project_name).await;
    }

    // 检查现有配置
    if check_existing_config().await? {
        return Ok(());
    }

    // 选择项目模板
    let template = select_project_template().await?;

    // 根据模板配置项目
    let config = if template.name == "自定义配置" {
        configure_custom_project(project_name).await?
    } else {
        configure_from_template(template, project_name).await?
    };

    // 保存配置
    save_project_config(&config).await?;

    // 显示完成信息
    show_completion_info(&config).await?;

    Ok(())
}

/// 显示欢迎界面
fn show_welcome_screen(project_name: &Option<String>) {
    println!("\n{}", colorize("🚀 寸止 CLI 项目初始化向导", colors::BOLD));
    println!("{}", colorize("━".repeat(50).as_str(), colors::DIM));

    if let Some(name) = project_name {
        println!("📁 项目名称: {}", colorize(name, colors::GREEN));
    }

    println!("\n欢迎使用寸止 CLI！");
    println!("这个向导将帮助您快速配置智能代码审查工具。");
    println!("整个过程只需要几分钟时间。\n");
}

/// 快速设置（使用默认配置）
async fn quick_setup(project_name: Option<String>) -> Result<()> {
    let name = project_name.unwrap_or_else(|| "cunzhi-project".to_string());

    print_boxed_message(
        "快速设置",
        &format!("为项目 '{}' 创建默认配置", name)
    );

    let pb = ModernProgressBar::new_spinner();
    pb.set_message("正在创建配置...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // 模拟配置创建过程
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    let config = AppConfig::default();
    save_standalone_config(&config)?;

    pb.finish_with_message("配置创建完成！");

    println!("\n✨ 快速设置完成！");
    println!("运行 {} 查看配置详情", colorize("cunzhi config show", colors::CYAN));

    Ok(())
}

/// 检查现有配置
async fn check_existing_config() -> Result<bool> {
    match crate::config::load_standalone_config() {
        Ok(_) => {
            log_warning!("检测到现有配置文件");

            let overwrite = Confirm::new("是否要重新配置？")
                .with_default(false)
                .with_help_message("选择 'no' 将保持现有配置不变")
                .prompt()?;

            if !overwrite {
                log_success!("保持现有配置不变");
                return Ok(true);
            }
            Ok(false)
        }
        Err(_) => Ok(false),
    }
}

/// 选择项目模板
async fn select_project_template() -> Result<ProjectTemplate> {
    println!("{}", colorize("📋 选择项目模板", colors::CYAN));
    println!("请选择最适合您项目的配置模板：\n");

    let templates = ProjectTemplate::get_templates();
    let template_options: Vec<String> = templates
        .iter()
        .map(|t| format!("{} - {}", t.name, t.description))
        .collect();

    let selection = Select::new("选择模板:", template_options.clone())
        .with_help_message("使用方向键选择，回车确认")
        .prompt()?;

    // 找到对应的模板
    let selected_index = template_options
        .iter()
        .position(|opt| opt == &selection)
        .unwrap();

    Ok(templates[selected_index].clone())
}

/// 根据模板配置项目
async fn configure_from_template(template: ProjectTemplate, _project_name: Option<String>) -> Result<AppConfig> {
    println!("\n{}", colorize(&format!("⚙️ 配置 '{}'", template.name), colors::CYAN));

    // 创建 MCP 配置
    let mut mcp_tools = HashMap::new();
    for tool in &["zhi", "ji"] {
        mcp_tools.insert(tool.to_string(), template.mcp_tools.contains(&tool.to_string()));
    }

    let mcp_config = McpConfig { tools: mcp_tools };

    // 创建回复配置
    let reply_config = ReplyConfig {
        enable_continue_reply: template.enable_continue_reply,
        auto_continue_threshold: template.continue_threshold,
        continue_prompt: "请按照最佳实践继续".to_string(),
    };

    let config = AppConfig {
        reply_config,
        mcp_config,
        terminal_config: crate::config::default_terminal_config(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(config)
}

/// 自定义项目配置
async fn configure_custom_project(_project_name: Option<String>) -> Result<AppConfig> {
    println!("\n{}", colorize("🛠️ 自定义配置", colors::CYAN));
    println!("让我们逐步配置您的项目：\n");

    // 配置 MCP 工具
    let mcp_config = configure_mcp_tools_interactive().await?;

    // 配置回复设置
    let reply_config = configure_reply_settings_interactive().await?;

    let config = AppConfig {
        reply_config,
        mcp_config,
        terminal_config: crate::config::default_terminal_config(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(config)
}

/// 交互式配置 MCP 工具
async fn configure_mcp_tools_interactive() -> Result<McpConfig> {
    println!("🔧 MCP 工具配置");
    println!("选择您需要启用的智能代码审查工具：");

    let available_tools = vec![
        "zhi - 智能代码审查工具，提供代码质量分析和建议",
        "ji - 记忆管理工具，管理项目知识和最佳实践",
    ];

    let selected = MultiSelect::new("选择工具:", available_tools)
        .with_help_message("使用空格键选择/取消选择，回车确认")
        .prompt()?;

    let mut tools = HashMap::new();
    tools.insert("zhi".to_string(), selected.iter().any(|s| s.starts_with("zhi")));
    tools.insert("ji".to_string(), selected.iter().any(|s| s.starts_with("ji")));

    println!("✅ MCP 工具配置完成\n");
    Ok(McpConfig { tools })
}

/// 交互式配置回复设置
async fn configure_reply_settings_interactive() -> Result<ReplyConfig> {
    println!("💬 回复设置配置");

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

    println!("✅ 回复设置配置完成\n");

    Ok(ReplyConfig {
        enable_continue_reply,
        auto_continue_threshold,
        continue_prompt,
    })
}

/// 保存项目配置
async fn save_project_config(config: &AppConfig) -> Result<()> {
    println!("{}", colorize("💾 保存配置", colors::CYAN));

    let pb = ModernProgressBar::new_spinner();
    pb.set_message("正在保存配置文件...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // 模拟保存过程
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

    save_standalone_config(config)?;
    pb.finish_with_message("配置文件保存成功！");

    Ok(())
}

/// 显示完成信息
async fn show_completion_info(config: &AppConfig) -> Result<()> {
    println!("\n{}", colorize("🎉 初始化完成！", colors::BOLD));
    println!("{}", colorize("━".repeat(50).as_str(), colors::DIM));

    // 显示配置摘要
    println!("\n📋 配置摘要:");

    // MCP 工具
    let enabled_tools: Vec<String> = config.mcp_config.tools
        .iter()
        .filter_map(|(name, &enabled)| if enabled { Some(name.clone()) } else { None })
        .collect();

    if !enabled_tools.is_empty() {
        println!("  🔧 启用的 MCP 工具: {}", enabled_tools.join(", "));
    } else {
        println!("  🔧 MCP 工具: 无");
    }

    // 回复设置
    if config.reply_config.enable_continue_reply {
        println!("  💬 自动继续回复: 启用 (阈值: {} 字符)",
            config.reply_config.auto_continue_threshold);
    } else {
        println!("  💬 自动继续回复: 禁用");
    }

    // 下一步提示
    println!("\n🚀 下一步操作:");
    println!("  {} - 查看完整配置", colorize("cunzhi config show", colors::CYAN));
    println!("  {} - 启动 MCP 服务器", colorize("cunzhi server start", colors::CYAN));
    println!("  {} - 查看帮助信息", colorize("cunzhi --help", colors::CYAN));

    // 显示配置文件位置
    match crate::config::get_standalone_config_path() {
        Ok(path) => {
            println!("\n📁 配置文件位置: {}", colorize(&path.display().to_string(), colors::DIM));
        }
        Err(_) => {}
    }

    println!("\n感谢使用寸止 CLI！🎯");

    Ok(())
}

/// 验证项目名称
pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("项目名称不能为空"));
    }

    if name.len() > 50 {
        return Err(anyhow::anyhow!("项目名称不能超过 50 个字符"));
    }

    // 检查是否包含非法字符
    if name.chars().any(|c| !c.is_alphanumeric() && c != '-' && c != '_') {
        return Err(anyhow::anyhow!("项目名称只能包含字母、数字、连字符和下划线"));
    }

    Ok(())
}

/// 检查项目目录
pub fn check_project_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("目录不存在: {}", path.display()));
    }

    if !path.is_dir() {
        return Err(anyhow::anyhow!("路径不是目录: {}", path.display()));
    }

    // 检查是否有写入权限
    if path.metadata()?.permissions().readonly() {
        return Err(anyhow::anyhow!("目录没有写入权限: {}", path.display()));
    }

    Ok(())
}

/// 创建项目配置模板
pub fn create_config_template(template_name: &str) -> Result<AppConfig> {
    let templates = ProjectTemplate::get_templates();

    let template = templates
        .iter()
        .find(|t| t.name == template_name)
        .ok_or_else(|| anyhow::anyhow!("未知的模板: {}", template_name))?;

    let mut mcp_tools = HashMap::new();
    for tool in &["zhi", "ji"] {
        mcp_tools.insert(tool.to_string(), template.mcp_tools.contains(&tool.to_string()));
    }

    let config = AppConfig {
        reply_config: ReplyConfig {
            enable_continue_reply: template.enable_continue_reply,
            auto_continue_threshold: template.continue_threshold,
            continue_prompt: "请按照最佳实践继续".to_string(),
        },
        mcp_config: McpConfig { tools: mcp_tools },
        terminal_config: crate::config::default_terminal_config(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(config)
}
