// 配置存储管理 - CLI 版本
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use super::settings::AppConfig;

/// 独立加载配置文件（用于CLI和MCP服务器）
pub fn load_standalone_config() -> Result<AppConfig> {
    let config_path = get_standalone_config_path()?;

    if config_path.exists() {
        let config_json = fs::read_to_string(&config_path)?;
        let config: AppConfig = serde_json::from_str(&config_json)
            .map_err(|e| anyhow::anyhow!("配置文件格式错误: {}", e))?;

        // 验证配置
        config.validate()?;

        Ok(config)
    } else {
        // 如果配置文件不存在，创建默认配置并保存
        let default_config = AppConfig::default();
        save_standalone_config(&default_config)?;
        Ok(default_config)
    }
}

/// 保存配置文件
pub fn save_standalone_config(config: &AppConfig) -> Result<()> {
    let config_path = get_standalone_config_path()?;

    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 验证配置
    config.validate()?;

    let config_json = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, config_json)?;

    Ok(())
}

/// 获取独立配置文件路径（不依赖GUI框架）
pub fn get_standalone_config_path() -> Result<PathBuf> {
    // 使用标准的配置目录
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("无法获取配置目录"))?
        .join("cunzhi");

    // 确保目录存在
    fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("config.json"))
}

/// 检查配置文件是否存在
pub fn config_exists() -> bool {
    get_standalone_config_path()
        .map(|path| path.exists())
        .unwrap_or(false)
}

/// 备份当前配置
pub fn backup_config() -> Result<PathBuf> {
    let config_path = get_standalone_config_path()?;

    if !config_path.exists() {
        return Err(anyhow::anyhow!("配置文件不存在，无法备份"));
    }

    let backup_path = config_path.with_extension("json.backup");
    fs::copy(&config_path, &backup_path)?;

    Ok(backup_path)
}

/// 从备份恢复配置
pub fn restore_from_backup() -> Result<()> {
    let config_path = get_standalone_config_path()?;
    let backup_path = config_path.with_extension("json.backup");

    if !backup_path.exists() {
        return Err(anyhow::anyhow!("备份文件不存在"));
    }

    fs::copy(&backup_path, &config_path)?;

    Ok(())
}

// 兼容性函数，保持与原项目的接口一致
pub fn load_config() -> Result<AppConfig> {
    load_standalone_config()
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    save_standalone_config(config)
}
