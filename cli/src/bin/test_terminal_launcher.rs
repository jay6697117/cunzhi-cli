use cunzhi_cli::utils::terminal_launcher::{
    TerminalLauncher, TerminalLauncherConfig, TerminalType,
    detect_system_terminal, launch_terminal_with_default_config
};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 寸止 CLI - 终端启动器测试程序");
    println!("═══════════════════════════════════════");

    // 测试 1: 检测系统终端
    println!("\n📋 测试 1: 检测当前系统可用的终端");
    match detect_system_terminal() {
        Ok(terminal) => {
            println!("✅ 检测到最佳终端: {:?}", terminal);
        }
        Err(e) => {
            println!("❌ 检测终端失败: {}", e);
        }
    }

    // 测试 2: 列出所有可能的终端
    println!("\n📋 测试 2: 检查各平台终端的可用性");
    let launcher = TerminalLauncher::new(TerminalLauncherConfig::default());
    let candidates = if cfg!(target_os = "macos") {
        vec![
            TerminalType::ITerm2,
            TerminalType::TerminalApp,
            TerminalType::Alacritty,
        ]
    } else if cfg!(target_os = "linux") {
        vec![
            TerminalType::GnomeTerminal,
            TerminalType::Konsole,
            TerminalType::Alacritty,
            TerminalType::Xterm,
        ]
    } else if cfg!(target_os = "windows") {
        vec![
            TerminalType::WindowsTerminal,
            TerminalType::PowerShell,
            TerminalType::Cmd,
        ]
    } else {
        vec![TerminalType::Xterm]
    };

    for terminal in candidates {
        let available = launcher.is_terminal_available(&terminal);
        let status = if available { "✅" } else { "❌" };
        println!("  {} {:?}", status, terminal);
    }

    // 测试 3: 获取用户确认是否测试终端启动
    println!("\n📋 测试 3: 终端启动测试");
    println!("是否要测试在新终端窗口中启动交互？(y/N): ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        println!("🚀 正在启动新终端窗口...");

        // 创建测试脚本
        let test_script = r#"
echo "🎉 终端启动测试成功！"
echo "这是一个新的终端窗口"
echo "═══════════════════════════════════════"
echo "平台信息: $(uname -s 2>/dev/null || echo Windows)"
echo "时间: $(date 2>/dev/null || echo %date% %time%)"
echo "═══════════════════════════════════════"
echo ""
echo "请按任意键退出..."
read -n 1
"#;

        let temp_dir = env::temp_dir();
        let script_path = temp_dir.join("cunzhi_terminal_test.sh");
        std::fs::write(&script_path, test_script)?;

        // 在 Unix 系统上设置执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&script_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&script_path, perms)?;
        }

        // 启动终端
        match launch_terminal_with_default_config("bash", &[script_path.to_string_lossy().to_string()]).await {
            Ok(_) => {
                println!("✅ 终端启动成功！");
                println!("💡 如果您看到新的终端窗口打开，说明终端启动器工作正常。");
            }
            Err(e) => {
                println!("❌ 终端启动失败: {}", e);
                println!("💡 这可能意味着：");
                println!("   - 系统上没有可用的终端程序");
                println!("   - 终端程序路径不在 PATH 中");
                println!("   - 权限问题");
            }
        }

        // 清理临时文件
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let _ = std::fs::remove_file(&script_path);
    } else {
        println!("跳过终端启动测试");
    }

    // 测试 4: 配置验证
    println!("\n📋 测试 4: 配置功能验证");

    // 测试默认配置
    let default_config = TerminalLauncherConfig::default();
    println!("✅ 默认配置创建成功");
    println!("   - 首选终端: {:?}", default_config.preferred_terminal);
    println!("   - 回退到CLI: {}", default_config.fallback_to_cli);
    println!("   - 窗口标题: {:?}", default_config.window_title);

    // 测试自定义配置
    let mut custom_config = TerminalLauncherConfig::default();
    custom_config.preferred_terminal = Some(TerminalType::Custom("echo".to_string()));
    custom_config.window_title = Some("测试窗口".to_string());
    println!("✅ 自定义配置创建成功");

    println!("\n🎉 终端启动器测试完成！");
    println!("═══════════════════════════════════════");

    Ok(())
}
