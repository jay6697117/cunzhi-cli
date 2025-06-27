// 配置管理命令实现
use anyhow::Result;
use crate::cli::ConfigAction;
use crate::config::{load_standalone_config, save_standalone_config, get_standalone_config_path, backup_config};
// Telegram 功能已移除

pub async fn handle_config_command(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Set => {
            println!("📝 交互式配置设置功能将在后续任务中实现");
            println!("当前可以直接编辑配置文件: {:?}", get_standalone_config_path()?);
            Ok(())
        }
        ConfigAction::Show => {
            show_config().await
        }
        ConfigAction::Validate => {
            validate_config().await
        }
    }
}

async fn show_config() -> Result<()> {
    println!("📋 当前配置信息");
    println!("配置文件位置: {:?}", get_standalone_config_path()?);

    match load_standalone_config() {
        Ok(config) => {
            println!("\n配置摘要:");
            println!("{}", config.get_summary());

            println!("\n详细配置:");
            let config_json = serde_json::to_string_pretty(&config)?;
            println!("{}", config_json);
        }
        Err(e) => {
            println!("❌ 加载配置失败: {}", e);
            println!("将创建默认配置...");

            let default_config = crate::config::AppConfig::default();
            save_standalone_config(&default_config)?;
            println!("✅ 默认配置已创建");
        }
    }

    Ok(())
}

async fn validate_config() -> Result<()> {
    println!("✅ 验证配置文件");

    match load_standalone_config() {
        Ok(config) => {
            match config.validate() {
                Ok(_) => {
                    println!("✅ 配置验证通过");
                    println!("配置摘要:\n{}", config.get_summary());
                }
                Err(e) => {
                    println!("❌ 配置验证失败: {}", e);
                    println!("请检查配置文件: {:?}", get_standalone_config_path()?);

                    // 提供备份选项
                    println!("\n是否要备份当前配置并重置为默认配置？");
                    println!("(这需要手动确认，当前仅显示建议)");

                    if let Ok(backup_path) = backup_config() {
                        println!("可以运行以下命令备份配置: cp {:?} {:?}",
                                get_standalone_config_path()?, backup_path);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 加载配置失败: {}", e);
            println!("配置文件可能损坏或不存在");

            println!("\n将创建默认配置...");
            let default_config = crate::config::AppConfig::default();
            save_standalone_config(&default_config)?;
            println!("✅ 默认配置已创建");
        }
    }

    Ok(())
}
