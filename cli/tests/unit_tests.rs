// 单元测试 - 测试核心功能模块
use cunzhi_cli::config::{AppConfig, ReplyConfig, McpConfig};
use cunzhi_cli::cli::init::{validate_project_name, create_config_template, ProjectTemplate};
use cunzhi_cli::utils::{ModernProgressBar, StatusIndicator, Table, AppError, ErrorHandler};
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_app_config_creation() {
    let config = AppConfig::default();
    
    assert_eq!(config.version, env!("CARGO_PKG_VERSION"));
    assert!(config.reply_config.enable_continue_reply);
    assert_eq!(config.reply_config.auto_continue_threshold, 1000);
    assert!(!config.reply_config.continue_prompt.is_empty());
}

#[test]
fn test_app_config_serialization() {
    let config = AppConfig::default();
    
    // 测试序列化
    let json = serde_json::to_string(&config).expect("Should serialize to JSON");
    assert!(json.contains("version"));
    assert!(json.contains("reply_config"));
    assert!(json.contains("mcp_config"));
    
    // 测试反序列化
    let deserialized: AppConfig = serde_json::from_str(&json)
        .expect("Should deserialize from JSON");
    
    assert_eq!(config.version, deserialized.version);
    assert_eq!(config.reply_config.enable_continue_reply, 
               deserialized.reply_config.enable_continue_reply);
}

#[test]
fn test_reply_config() {
    let reply_config = ReplyConfig {
        enable_continue_reply: true,
        auto_continue_threshold: 500,
        continue_prompt: "继续".to_string(),
    };
    
    assert!(reply_config.enable_continue_reply);
    assert_eq!(reply_config.auto_continue_threshold, 500);
    assert_eq!(reply_config.continue_prompt, "继续");
}

#[test]
fn test_mcp_config() {
    let mut tools = HashMap::new();
    tools.insert("zhi".to_string(), true);
    tools.insert("ji".to_string(), false);
    
    let mcp_config = McpConfig { tools };
    
    assert_eq!(mcp_config.tools.get("zhi"), Some(&true));
    assert_eq!(mcp_config.tools.get("ji"), Some(&false));
    assert_eq!(mcp_config.tools.get("nonexistent"), None);
}

#[test]
fn test_project_name_validation() {
    // 有效的项目名称
    assert!(validate_project_name("valid-project").is_ok());
    assert!(validate_project_name("valid_project").is_ok());
    assert!(validate_project_name("ValidProject123").is_ok());
    assert!(validate_project_name("a").is_ok());
    
    // 无效的项目名称
    assert!(validate_project_name("").is_err());
    assert!(validate_project_name("project with spaces").is_err());
    assert!(validate_project_name("project@invalid").is_err());
    assert!(validate_project_name("project/invalid").is_err());
    assert!(validate_project_name(&"a".repeat(60)).is_err());
}

#[test]
fn test_project_templates() {
    let templates = ProjectTemplate::get_templates();
    
    assert!(!templates.is_empty());
    assert!(templates.len() >= 3); // 至少有基础、完整、自定义三个模板
    
    // 检查基础配置模板
    let basic = templates.iter().find(|t| t.name == "基础配置");
    assert!(basic.is_some());
    let basic = basic.unwrap();
    assert!(basic.mcp_tools.contains(&"zhi".to_string()));
    assert!(!basic.enable_continue_reply);
    
    // 检查完整配置模板
    let full = templates.iter().find(|t| t.name == "完整配置");
    assert!(full.is_some());
    let full = full.unwrap();
    assert!(full.mcp_tools.contains(&"zhi".to_string()));
    assert!(full.mcp_tools.contains(&"ji".to_string()));
    assert!(full.enable_continue_reply);
}

#[test]
fn test_create_config_template() {
    // 测试基础配置模板
    let config = create_config_template("基础配置").expect("Should create basic config");
    assert!(!config.reply_config.enable_continue_reply);
    assert_eq!(config.mcp_config.tools.get("zhi"), Some(&true));
    assert_eq!(config.mcp_config.tools.get("ji"), Some(&false));
    
    // 测试完整配置模板
    let config = create_config_template("完整配置").expect("Should create full config");
    assert!(config.reply_config.enable_continue_reply);
    assert_eq!(config.mcp_config.tools.get("zhi"), Some(&true));
    assert_eq!(config.mcp_config.tools.get("ji"), Some(&true));
    
    // 测试无效模板
    assert!(create_config_template("不存在的模板").is_err());
}

#[test]
fn test_modern_progress_bar() {
    // 测试进度条创建
    let pb = ModernProgressBar::new(100);
    pb.set_message("测试消息");
    pb.set_position(50);
    pb.finish_with_message("完成");
    
    // 测试旋转器创建
    let spinner = ModernProgressBar::new_spinner();
    spinner.set_message("旋转中");
    spinner.finish_with_message("旋转完成");
}

#[test]
fn test_status_indicator() {
    let indicator = StatusIndicator::new();
    
    // 这些方法主要是输出，我们只测试它们不会崩溃
    indicator.success("成功消息");
    indicator.error("错误消息");
    indicator.warning("警告消息");
    indicator.info("信息消息");
    indicator.step(1, 3, "步骤消息");
}

#[test]
fn test_table() {
    let mut table = Table::new(vec!["列1", "列2", "列3"]);
    table.add_row(vec!["值1", "值2", "值3"]);
    table.add_row(vec!["长一点的值", "短值", "中等长度的值"]);
    
    // 表格打印主要是输出，我们只测试它不会崩溃
    table.print();
}

#[test]
fn test_app_error_types() {
    let errors = vec![
        AppError::config("配置错误"),
        AppError::file_operation("文件错误"),
        AppError::network("网络错误"),
        AppError::mcp_server("服务器错误"),
        AppError::user_input("输入错误"),
        AppError::permission("权限错误"),
        AppError::dependency("依赖错误"),
        AppError::internal("内部错误"),
    ];
    
    for error in errors {
        let error_string = error.to_string();
        assert!(!error_string.is_empty());
        assert!(error_string.contains("错误"));
    }
}

#[test]
fn test_error_handler() {
    let handler = ErrorHandler::new(false);
    
    // 测试不同类型的错误处理
    let config_error = AppError::config("测试配置错误");
    let anyhow_error: anyhow::Error = config_error.into();
    let exit_code = handler.handle_error(&anyhow_error);
    assert_eq!(exit_code, 1); // 配置错误的退出码
    
    let file_error = AppError::file_operation("测试文件错误");
    let anyhow_error: anyhow::Error = file_error.into();
    let exit_code = handler.handle_error(&anyhow_error);
    assert_eq!(exit_code, 2); // 文件错误的退出码
}

#[test]
fn test_config_file_operations() {
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let config_path = temp_dir.path().join("config.json");
    
    // 测试配置保存
    let config = AppConfig::default();
    let json = serde_json::to_string_pretty(&config).expect("Should serialize config");
    std::fs::write(&config_path, json).expect("Should write config file");
    
    // 测试配置加载
    let loaded_content = std::fs::read_to_string(&config_path)
        .expect("Should read config file");
    let loaded_config: AppConfig = serde_json::from_str(&loaded_content)
        .expect("Should deserialize config");
    
    assert_eq!(config.version, loaded_config.version);
    assert_eq!(config.reply_config.enable_continue_reply, 
               loaded_config.reply_config.enable_continue_reply);
}

#[test]
fn test_config_validation() {
    // 测试有效配置
    let valid_config = AppConfig {
        version: "0.2.12".to_string(),
        reply_config: ReplyConfig {
            enable_continue_reply: true,
            auto_continue_threshold: 1000,
            continue_prompt: "继续".to_string(),
        },
        mcp_config: McpConfig {
            tools: {
                let mut tools = HashMap::new();
                tools.insert("zhi".to_string(), true);
                tools.insert("ji".to_string(), true);
                tools
            },
        },
    };
    
    // 配置应该能够序列化和反序列化
    let json = serde_json::to_string(&valid_config).expect("Should serialize");
    let _: AppConfig = serde_json::from_str(&json).expect("Should deserialize");
}

#[test]
fn test_mcp_tools_configuration() {
    let mut tools = HashMap::new();
    tools.insert("zhi".to_string(), true);
    tools.insert("ji".to_string(), false);
    tools.insert("custom_tool".to_string(), true);
    
    let mcp_config = McpConfig { tools };
    
    // 测试工具状态查询
    assert!(mcp_config.tools.get("zhi").copied().unwrap_or(false));
    assert!(!mcp_config.tools.get("ji").copied().unwrap_or(false));
    assert!(mcp_config.tools.get("custom_tool").copied().unwrap_or(false));
    assert!(!mcp_config.tools.get("nonexistent").copied().unwrap_or(false));
    
    // 测试启用的工具计数
    let enabled_count = mcp_config.tools.values().filter(|&&enabled| enabled).count();
    assert_eq!(enabled_count, 2);
}

#[test]
fn test_reply_config_edge_cases() {
    // 测试极端值
    let config = ReplyConfig {
        enable_continue_reply: false,
        auto_continue_threshold: 0,
        continue_prompt: "".to_string(),
    };
    
    assert!(!config.enable_continue_reply);
    assert_eq!(config.auto_continue_threshold, 0);
    assert!(config.continue_prompt.is_empty());
    
    // 测试大值
    let config = ReplyConfig {
        enable_continue_reply: true,
        auto_continue_threshold: u32::MAX,
        continue_prompt: "很长的提示词".repeat(100),
    };
    
    assert!(config.enable_continue_reply);
    assert_eq!(config.auto_continue_threshold, u32::MAX);
    assert!(config.continue_prompt.len() > 1000);
}

#[test]
fn test_concurrent_config_access() {
    use std::sync::Arc;
    use std::thread;
    
    let config = Arc::new(AppConfig::default());
    let handles: Vec<_> = (0..5).map(|_| {
        let config_clone = Arc::clone(&config);
        thread::spawn(move || {
            // 多线程读取配置
            assert!(!config_clone.version.is_empty());
            assert!(config_clone.reply_config.auto_continue_threshold > 0);
        })
    }).collect();
    
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
}
