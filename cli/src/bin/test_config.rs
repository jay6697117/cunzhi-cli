// 配置系统测试程序
use cunzhi_cli::config::{load_standalone_config, save_standalone_config, AppConfig};

fn main() -> anyhow::Result<()> {
    println!("🧪 测试配置管理系统");

    // 测试默认配置
    println!("\n1. 测试默认配置");
    let default_config = AppConfig::default();
    println!("默认配置摘要:\n{}", default_config.get_summary());

    // 测试配置保存
    println!("\n2. 测试配置保存");
    save_standalone_config(&default_config)?;
    println!("✅ 配置保存成功");

    // 测试配置加载
    println!("\n3. 测试配置加载");
    let loaded_config = load_standalone_config()?;
    println!("加载的配置摘要:\n{}", loaded_config.get_summary());

    // 测试配置修改
    println!("\n4. 测试配置修改");
    let mut modified_config = loaded_config.clone();
    modified_config.mcp_config.set_tool_enabled("ji", false);
    // Telegram 功能已移除
    // modified_config.telegram_config.enabled = true;
    // modified_config.telegram_config.bot_token = "123456:test_token".to_string();
    // modified_config.telegram_config.chat_id = "123456789".to_string();

    println!("修改后的配置摘要:\n{}", modified_config.get_summary());

    // 测试配置验证
    println!("\n5. 测试配置验证");
    match modified_config.validate() {
        Ok(_) => println!("✅ 配置验证通过"),
        Err(e) => println!("❌ 配置验证失败: {}", e),
    }

    // 保存修改后的配置
    save_standalone_config(&modified_config)?;
    println!("✅ 修改后的配置保存成功");

    // 重新加载验证
    let reloaded_config = load_standalone_config()?;
    println!("\n6. 重新加载验证");
    println!("重新加载的配置摘要:\n{}", reloaded_config.get_summary());

    println!("\n🎉 配置管理系统测试完成！");
    Ok(())
}
