// 集成测试 - 测试主要功能和命令
use std::process::Command;
use std::fs;

use tempfile::TempDir;

/// 测试辅助函数
struct TestHelper {
    temp_dir: TempDir,
}

impl TestHelper {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        Self { temp_dir }
    }

    fn run_command(&self, args: &[&str]) -> std::process::Output {
        // 获取当前工作目录（应该是项目根目录）
        let current_dir = std::env::current_dir().expect("Failed to get current directory");

        let mut cmd = Command::new("cargo");
        cmd.arg("run")
           .arg("--")
           .args(args)
           .current_dir(&current_dir)  // 在项目根目录运行
           .env("CUNZHI_CONFIG_DIR", self.temp_dir.path());

        cmd.output().expect("Failed to execute command")
    }

    fn config_path(&self) -> std::path::PathBuf {
        self.temp_dir.path().join("config.json")
    }
}

#[test]
fn test_version_command() {
    let helper = TestHelper::new();
    let output = helper.run_command(&["--version"]);

    assert!(output.status.success(), "Version command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cunzhi-cli"), "Output should contain program name");
    assert!(stdout.contains("0.2.12"), "Output should contain version number");
}

#[test]
fn test_help_command() {
    let helper = TestHelper::new();
    let output = helper.run_command(&["--help"]);

    assert!(output.status.success(), "Help command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("寸止 CLI"), "Help should contain program description");
    assert!(stdout.contains("COMMAND"), "Help should show available commands");
    assert!(stdout.contains("init"), "Help should mention init command");
    assert!(stdout.contains("server"), "Help should mention server command");
    assert!(stdout.contains("config"), "Help should mention config command");
}

#[test]
fn test_init_command_with_defaults() {
    let helper = TestHelper::new();
    let output = helper.run_command(&["init", "--name", "test-project", "--yes"]);

    assert!(output.status.success(), "Init command should succeed");

    // 检查配置文件是否创建
    assert!(helper.config_path().exists(), "Config file should be created");

    // 检查配置文件内容
    let config_content = fs::read_to_string(helper.config_path())
        .expect("Should be able to read config file");

    assert!(config_content.contains("version"), "Config should contain version");
    assert!(config_content.contains("reply_config"), "Config should contain reply_config");
    assert!(config_content.contains("mcp_config"), "Config should contain mcp_config");
}

#[test]
fn test_config_show_command() {
    let helper = TestHelper::new();

    // 首先初始化配置
    let init_output = helper.run_command(&["init", "--name", "test-project", "--yes"]);
    assert!(init_output.status.success(), "Init should succeed");

    // 然后显示配置
    let output = helper.run_command(&["config", "show"]);
    assert!(output.status.success(), "Config show should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("配置信息"), "Output should contain config info");
    assert!(stdout.contains("版本"), "Output should show version");
    assert!(stdout.contains("MCP 工具"), "Output should show MCP tools");
}

#[test]
fn test_config_reset_command() {
    let helper = TestHelper::new();

    // 首先初始化配置
    let init_output = helper.run_command(&["init", "--name", "test-project", "--yes"]);
    assert!(init_output.status.success(), "Init should succeed");

    // 重置配置
    let output = helper.run_command(&["config", "reset"]);
    assert!(output.status.success(), "Config reset should succeed");

    // 验证配置文件仍然存在且有效
    assert!(helper.config_path().exists(), "Config file should still exist");

    let config_content = fs::read_to_string(helper.config_path())
        .expect("Should be able to read config file");
    assert!(config_content.contains("version"), "Reset config should be valid");
}

#[test]
fn test_server_status_command() {
    let helper = TestHelper::new();

    // 首先初始化配置
    let init_output = helper.run_command(&["init", "--name", "test-project", "--yes"]);
    assert!(init_output.status.success(), "Init should succeed");

    // 检查服务器状态
    let output = helper.run_command(&["server", "status"]);
    assert!(output.status.success(), "Server status should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("服务器状态"), "Output should contain server status");
}

#[test]
fn test_server_lifecycle() {
    let helper = TestHelper::new();

    // 首先初始化配置
    let init_output = helper.run_command(&["init", "--name", "test-project", "--yes"]);
    assert!(init_output.status.success(), "Init should succeed");

    // 启动服务器
    let start_output = helper.run_command(&["server", "start"]);
    assert!(start_output.status.success(), "Server start should succeed");

    let start_stdout = String::from_utf8_lossy(&start_output.stdout);
    assert!(start_stdout.contains("启动成功") || start_stdout.contains("已启动"),
           "Start output should indicate success");

    // 检查状态
    let status_output = helper.run_command(&["server", "status"]);
    assert!(status_output.status.success(), "Server status should succeed");

    // 停止服务器
    let stop_output = helper.run_command(&["server", "stop"]);
    assert!(stop_output.status.success(), "Server stop should succeed");
}

#[test]
fn test_doctor_command() {
    let helper = TestHelper::new();

    let output = helper.run_command(&["doctor"]);
    assert!(output.status.success(), "Doctor command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("系统诊断") || stdout.contains("检查"),
           "Doctor output should contain diagnostic info");
}

#[test]
fn test_invalid_command() {
    let helper = TestHelper::new();

    let output = helper.run_command(&["invalid-command"]);
    assert!(!output.status.success(), "Invalid command should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("错误"),
           "Error output should contain error message");
}

#[test]
fn test_config_file_validation() {
    let helper = TestHelper::new();

    // 创建无效的配置文件
    fs::write(helper.config_path(), "invalid json content")
        .expect("Should be able to write invalid config");

    // 尝试显示配置，应该处理错误
    let output = helper.run_command(&["config", "show"]);
    // 命令可能成功（显示错误信息）或失败，但不应该崩溃

    let combined_output = format!("{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // 应该包含某种错误指示
    assert!(combined_output.contains("错误") ||
           combined_output.contains("error") ||
           combined_output.contains("invalid") ||
           combined_output.contains("配置"),
           "Should handle invalid config gracefully");
}

#[test]
fn test_init_project_name_validation() {
    let helper = TestHelper::new();

    // 测试无效的项目名称
    let output = helper.run_command(&["init", "--name", "invalid@name", "--yes"]);

    // 命令可能失败或成功但显示警告
    let combined_output = format!("{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // 如果命令失败，应该有相关错误信息
    if !output.status.success() {
        assert!(combined_output.contains("名称") ||
               combined_output.contains("name") ||
               combined_output.contains("invalid"),
               "Should provide feedback about invalid project name");
    }
}

#[test]
fn test_multiple_init_calls() {
    let helper = TestHelper::new();

    // 第一次初始化
    let output1 = helper.run_command(&["init", "--name", "test-project", "--yes"]);
    assert!(output1.status.success(), "First init should succeed");

    // 第二次初始化（应该检测到现有配置）
    let _output2 = helper.run_command(&["init", "--name", "test-project2", "--yes"]);
    // 可能成功（覆盖）或失败（保护现有配置）

    // 无论如何，配置文件应该存在
    assert!(helper.config_path().exists(), "Config file should exist after multiple inits");
}

#[test]
fn test_command_help_subcommands() {
    let helper = TestHelper::new();

    // 测试各个子命令的帮助
    let commands = ["init", "server", "config"];

    for cmd in &commands {
        let output = helper.run_command(&[cmd, "--help"]);
        assert!(output.status.success(), "Help for {} should succeed", cmd);

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.len() > 0, "Help for {} should produce output", cmd);
    }
}

// 性能测试
#[test]
fn test_command_performance() {
    let helper = TestHelper::new();

    let start = std::time::Instant::now();
    let output = helper.run_command(&["--version"]);
    let duration = start.elapsed();

    assert!(output.status.success(), "Version command should succeed");
    assert!(duration.as_secs() < 5, "Version command should complete within 5 seconds");
}

// 并发测试
#[test]
fn test_concurrent_operations() {
    use std::thread;

    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            let helper = TestHelper::new();
            let output = helper.run_command(&["init", "--name", &format!("test-{}", i), "--yes"]);
            assert!(output.status.success(), "Concurrent init {} should succeed", i);
        })
    }).collect();

    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}
