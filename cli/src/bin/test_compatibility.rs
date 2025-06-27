// 配置兼容性测试程序
use cunzhi_cli::config::{load_standalone_config, save_standalone_config, get_standalone_config_path};
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("🔄 测试配置向后兼容性");

    let config_path = get_standalone_config_path()?;
    
    // 备份当前配置
    let backup_content = if config_path.exists() {
        Some(fs::read_to_string(&config_path)?)
    } else {
        None
    };

    // 测试1: 空配置文件
    println!("\n1. 测试空配置文件处理");
    fs::write(&config_path, "{}")?;
    match load_standalone_config() {
        Ok(config) => {
            println!("✅ 空配置文件处理成功");
            println!("默认配置摘要:\n{}", config.get_summary());
        }
        Err(e) => println!("❌ 空配置文件处理失败: {}", e),
    }

    // 测试2: 部分配置文件
    println!("\n2. 测试部分配置文件处理");
    let partial_config = r#"{
        "mcp_config": {
            "tools": {
                "zhi": true
            }
        }
    }"#;
    fs::write(&config_path, partial_config)?;
    match load_standalone_config() {
        Ok(config) => {
            println!("✅ 部分配置文件处理成功");
            println!("配置摘要:\n{}", config.get_summary());
        }
        Err(e) => println!("❌ 部分配置文件处理失败: {}", e),
    }

    // 测试3: 包含未知字段的配置文件
    println!("\n3. 测试包含未知字段的配置文件");
    let unknown_fields_config = r#"{
        "reply_config": {
            "enable_continue_reply": false,
            "auto_continue_threshold": 500,
            "continue_prompt": "继续",
            "unknown_field": "should_be_ignored"
        },
        "mcp_config": {
            "tools": {
                "zhi": true,
                "ji": false,
                "unknown_tool": true
            }
        },
        "telegram_config": {
            "enabled": false,
            "bot_token": "",
            "chat_id": "",
            "hide_frontend_popup": false,
            "api_base_url": "https://api.telegram.org/bot",
            "unknown_telegram_field": "ignored"
        },
        "version": "0.1.0",
        "unknown_top_level": "ignored"
    }"#;
    fs::write(&config_path, unknown_fields_config)?;
    match load_standalone_config() {
        Ok(config) => {
            println!("✅ 包含未知字段的配置文件处理成功");
            println!("配置摘要:\n{}", config.get_summary());
        }
        Err(e) => println!("❌ 包含未知字段的配置文件处理失败: {}", e),
    }

    // 测试4: 无效配置验证
    println!("\n4. 测试无效配置验证");
    let invalid_config = r#"{
        "telegram_config": {
            "enabled": true,
            "bot_token": "invalid_token_without_colon",
            "chat_id": "",
            "hide_frontend_popup": false,
            "api_base_url": "https://api.telegram.org/bot"
        }
    }"#;
    fs::write(&config_path, invalid_config)?;
    match load_standalone_config() {
        Ok(_) => println!("❌ 应该检测到无效配置"),
        Err(e) => println!("✅ 正确检测到无效配置: {}", e),
    }

    // 恢复备份
    if let Some(backup) = backup_content {
        fs::write(&config_path, backup)?;
        println!("\n✅ 配置文件已恢复");
    } else {
        // 如果没有备份，创建默认配置
        let default_config = cunzhi_cli::config::AppConfig::default();
        save_standalone_config(&default_config)?;
        println!("\n✅ 创建了默认配置文件");
    }

    println!("\n🎉 配置兼容性测试完成！");
    Ok(())
}
