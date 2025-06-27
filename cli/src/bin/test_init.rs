// 初始化向导测试程序
use anyhow::Result;
use cunzhi_cli::cli::init::{
    validate_project_name,
    create_config_template,
    ProjectTemplate
};
use cunzhi_cli::utils::print_boxed_message;

#[tokio::main]
async fn main() -> Result<()> {
    print_boxed_message("初始化向导测试", "测试项目初始化向导的各项功能");

    // 测试项目名称验证
    println!("\n🧪 测试项目名称验证:");
    test_project_name_validation();

    // 测试配置模板
    println!("\n🧪 测试配置模板:");
    test_config_templates()?;

    // 测试项目模板
    println!("\n🧪 测试项目模板:");
    test_project_templates();

    println!("\n✅ 所有测试完成！");
    Ok(())
}

fn test_project_name_validation() {
    let long_name = "a".repeat(60);
    let test_cases = vec![
        ("valid-project", true),
        ("valid_project", true),
        ("ValidProject123", true),
        ("", false),
        ("project with spaces", false),
        ("project@invalid", false),
        (long_name.as_str(), false),
    ];

    for (name, should_pass) in test_cases {
        match validate_project_name(name) {
            Ok(_) => {
                if should_pass {
                    println!("  ✅ '{}' - 验证通过", name);
                } else {
                    println!("  ❌ '{}' - 应该失败但通过了", name);
                }
            }
            Err(e) => {
                if !should_pass {
                    println!("  ✅ '{}' - 正确拒绝: {}", name, e);
                } else {
                    println!("  ❌ '{}' - 应该通过但失败了: {}", name, e);
                }
            }
        }
    }
}

fn test_config_templates() -> Result<()> {
    let templates = vec!["基础配置", "完整配置"];

    for template_name in templates {
        match create_config_template(template_name) {
            Ok(config) => {
                println!("  ✅ 模板 '{}' 创建成功", template_name);
                println!("    - 版本: {}", config.version);
                println!("    - 启用工具数: {}",
                    config.mcp_config.tools.values().filter(|&&v| v).count());
                println!("    - 自动继续回复: {}",
                    if config.reply_config.enable_continue_reply { "启用" } else { "禁用" });
            }
            Err(e) => {
                println!("  ❌ 模板 '{}' 创建失败: {}", template_name, e);
            }
        }
    }

    // 测试无效模板
    match create_config_template("不存在的模板") {
        Ok(_) => println!("  ❌ 无效模板应该失败但成功了"),
        Err(_) => println!("  ✅ 无效模板正确拒绝"),
    }

    Ok(())
}

fn test_project_templates() {
    let templates = ProjectTemplate::get_templates();

    println!("  可用模板数量: {}", templates.len());

    for template in templates {
        println!("  📋 模板: {}", template.name);
        println!("    - 描述: {}", template.description);
        println!("    - MCP 工具: {:?}", template.mcp_tools);
        println!("    - 自动继续回复: {}", template.enable_continue_reply);
        println!("    - 继续阈值: {}", template.continue_threshold);
    }
}
