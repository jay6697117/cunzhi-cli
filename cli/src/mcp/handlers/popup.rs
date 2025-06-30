use anyhow::Result;
use std::process::Command;
use std::fs;
use std::path::Path;

use crate::mcp::types::PopupRequest;

/// 创建 CLI 交互弹窗
///
/// 优先调用与 MCP 服务器同目录的 UI 命令，找不到时使用全局版本
pub fn create_cli_popup(request: &PopupRequest) -> Result<String> {
    // 1. 优先尝试开发环境中的cargo运行方式
    if test_cargo_ui_available() {
        return create_cli_popup_with_cargo(request);
    }

    // 2. 尝试与当前 MCP 服务器同目录的cunzhi-ui命令
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            let local_ui_path = exe_dir.join("cunzhi-ui");
            if local_ui_path.exists() && is_executable(&local_ui_path) {
                return create_cli_popup_with_command(request, &local_ui_path.to_string_lossy());
            }
        }
    }

    // 3. 尝试全局命令
    if test_command_available("cunzhi-ui") {
        return create_cli_popup_with_command(request, "cunzhi-ui");
    }

    // 4. 如果都找不到，返回详细错误信息
    anyhow::bail!(
        "找不到cunzhi-ui命令。请确保：\n\
         1. 已编译项目：cargo build --release\n\
         2. 或已全局安装：cargo install --path .\n\
         3. 或cunzhi-ui命令在同目录下"
    )
}

/// 使用指定命令创建CLI弹窗
fn create_cli_popup_with_command(request: &PopupRequest, command: &str) -> Result<String> {
    // 创建临时请求文件 - 跨平台适配
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("mcp_request_{}.json", request.id));
    let request_json = serde_json::to_string_pretty(request)?;
    fs::write(&temp_file, request_json)?;

    // 调用cunzhi-ui命令
    let output = Command::new(command)
        .arg("--mcp-request")
        .arg(temp_file.to_string_lossy().to_string())
        .output()?;

    // 清理临时文件
    let _ = fs::remove_file(&temp_file);

    if output.status.success() {
        let response = String::from_utf8(output.stdout)?;
        Ok(response.trim().to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("UI进程失败: {}", error);
    }
}

/// 使用cargo运行UI命令（开发环境）
fn create_cli_popup_with_cargo(request: &PopupRequest) -> Result<String> {
    // 创建临时请求文件
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("mcp_request_{}.json", request.id));
    let request_json = serde_json::to_string_pretty(request)?;
    fs::write(&temp_file, request_json)?;

    // 使用cargo运行UI
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "cunzhi-ui",
            "--",
            "--mcp-request",
            &temp_file.to_string_lossy()
        ])
        .output()?;

    // 清理临时文件
    let _ = fs::remove_file(&temp_file);

    if output.status.success() {
        let response = String::from_utf8(output.stdout)?;
        Ok(response.trim().to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Cargo UI进程失败: {}", error);
    }
}

/// 测试命令是否可用
fn test_command_available(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 测试cargo运行UI是否可用
fn test_cargo_ui_available() -> bool {
    // 检查是否在开发环境中
    if let Ok(current_dir) = std::env::current_dir() {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            // 尝试运行cargo run --bin cunzhi-ui --version
            return Command::new("cargo")
                .args(&["run", "--bin", "cunzhi-ui", "--", "--version"])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false);
        }
    }
    false
}

/// 检查文件是否可执行
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
        return false;
    }

    #[cfg(windows)]
    {
        // Windows上，如果文件存在且是.exe文件，认为是可执行的
        return path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("exe"))
            .unwrap_or(false);
    }

    #[cfg(not(any(unix, windows)))]
    {
        // 其他平台，简单检查文件是否存在
        path.exists()
    }
}
