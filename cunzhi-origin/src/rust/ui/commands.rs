use crate::config::{save_config, AppState, ReplyConfig, WindowConfig};
use crate::constants::{window, ui, validation};
use crate::mcp::types::{build_continue_response, build_send_response, ImageAttachment};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn get_app_info() -> Result<String, String> {
    Ok(format!("寸止 v{}", env!("CARGO_PKG_VERSION")))
}

#[tauri::command]
pub async fn get_always_on_top(state: State<'_, AppState>) -> Result<bool, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.ui_config.always_on_top)
}

#[tauri::command]
pub async fn set_always_on_top(
    enabled: bool,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("获取配置失败: {}", e))?;
        config.ui_config.always_on_top = enabled;
    }

    // 保存配置到文件
    save_config(&state, &app)
        .await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    // 应用到当前窗口
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(enabled)
            .map_err(|e| format!("设置窗口置顶失败: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn sync_window_state(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 根据配置同步窗口状态
    let always_on_top = {
        let config = state
            .config
            .lock()
            .map_err(|e| format!("获取配置失败: {}", e))?;
        config.ui_config.always_on_top
    };

    // 应用到当前窗口
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(always_on_top)
            .map_err(|e| format!("同步窗口状态失败: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_theme(state: State<'_, AppState>) -> Result<String, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.ui_config.theme.clone())
}

#[tauri::command]
pub async fn set_theme(
    theme: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 验证主题值
    if !["light", "dark"].contains(&theme.as_str()) {
        return Err("无效的主题值，只支持 light、dark".to_string());
    }

    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("获取配置失败: {}", e))?;
        config.ui_config.theme = theme;
    }

    // 保存配置到文件
    save_config(&state, &app)
        .await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_window_config(state: State<'_, AppState>) -> Result<WindowConfig, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.ui_config.window_config.clone())
}

#[tauri::command]
pub async fn set_window_config(
    window_config: WindowConfig,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("获取配置失败: {}", e))?;
        config.ui_config.window_config = window_config;
    }

    // 保存配置到文件
    save_config(&state, &app)
        .await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_reply_config(state: State<'_, AppState>) -> Result<ReplyConfig, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.reply_config.clone())
}

#[tauri::command]
pub async fn set_reply_config(
    reply_config: ReplyConfig,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("获取配置失败: {}", e))?;
        config.reply_config = reply_config;
    }

    // 保存配置到文件
    save_config(&state, &app)
        .await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_window_settings(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("获取配置失败: {}", e))?;

    // 返回窗口设置，包含两种模式的独立尺寸
    let window_settings = serde_json::json!({
        "fixed": config.ui_config.window_config.fixed,
        "current_width": config.ui_config.window_config.current_width(),
        "current_height": config.ui_config.window_config.current_height(),
        "fixed_width": config.ui_config.window_config.fixed_width,
        "fixed_height": config.ui_config.window_config.fixed_height,
        "free_width": config.ui_config.window_config.free_width,
        "free_height": config.ui_config.window_config.free_height
    });

    Ok(window_settings)
}

#[tauri::command]
pub async fn get_window_settings_for_mode(
    fixed: bool,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("获取配置失败: {}", e))?;

    // 返回指定模式的窗口设置
    let (width, height) = if fixed {
        (
            config.ui_config.window_config.fixed_width,
            config.ui_config.window_config.fixed_height,
        )
    } else {
        (
            config.ui_config.window_config.free_width,
            config.ui_config.window_config.free_height,
        )
    };

    let window_settings = serde_json::json!({
        "width": width,
        "height": height,
        "fixed": fixed
    });

    Ok(window_settings)
}

#[tauri::command]
pub async fn get_window_constraints_cmd() -> Result<serde_json::Value, String> {
    let constraints = window::get_default_constraints();
    let ui_timings = ui::get_default_ui_timings();

    let mut result = constraints.to_json();
    if let serde_json::Value::Object(ref mut map) = result {
        if let serde_json::Value::Object(ui_map) = ui_timings.to_json() {
            map.extend(ui_map);
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_current_window_size(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        // 检查窗口是否最小化
        if let Ok(is_minimized) = window.is_minimized() {
            if is_minimized {
                return Err("窗口已最小化，跳过尺寸获取".to_string());
            }
        }

        // 获取逻辑尺寸而不是物理尺寸
        if let Ok(logical_size) = window.inner_size().map(|physical_size| {
            // 获取缩放因子
            let scale_factor = window.scale_factor().unwrap_or(1.0);

            // 转换为逻辑尺寸
            let logical_width = physical_size.width as f64 / scale_factor;
            let logical_height = physical_size.height as f64 / scale_factor;

            tauri::LogicalSize::new(logical_width, logical_height)
        }) {
            let width = logical_size.width.round() as u32;
            let height = logical_size.height.round() as u32;

            // 验证并调整尺寸到有效范围
            let (clamped_width, clamped_height) = crate::constants::window::clamp_window_size(width as f64, height as f64);
            let final_width = clamped_width as u32;
            let final_height = clamped_height as u32;

            if final_width != width || final_height != height {
                log::info!("窗口尺寸已调整: {}x{} -> {}x{}", width, height, final_width, final_height);
            }

            let window_size = serde_json::json!({
                "width": final_width,
                "height": final_height
            });
            return Ok(window_size);
        }
    }

    Err("无法获取当前窗口大小".to_string())
}

#[tauri::command]
pub async fn set_window_settings(
    window_settings: serde_json::Value,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("获取配置失败: {}", e))?;

        // 更新窗口配置
        if let Some(fixed) = window_settings.get("fixed").and_then(|v| v.as_bool()) {
            config.ui_config.window_config.fixed = fixed;
        }

        // 更新固定模式尺寸（添加尺寸验证）
        if let Some(width) = window_settings.get("fixed_width").and_then(|v| v.as_f64()) {
            if let Some(height) = window_settings.get("fixed_height").and_then(|v| v.as_f64()) {
                if validation::is_valid_window_size(width, height) {
                    config.ui_config.window_config.fixed_width = width;
                    config.ui_config.window_config.fixed_height = height;
                }
            } else if width >= window::MIN_WIDTH {
                config.ui_config.window_config.fixed_width = width;
            }
        } else if let Some(height) = window_settings.get("fixed_height").and_then(|v| v.as_f64()) {
            if height >= window::MIN_HEIGHT {
                config.ui_config.window_config.fixed_height = height;
            }
        }

        // 更新自由拉伸模式尺寸（添加尺寸验证）
        if let Some(width) = window_settings.get("free_width").and_then(|v| v.as_f64()) {
            if let Some(height) = window_settings.get("free_height").and_then(|v| v.as_f64()) {
                if validation::is_valid_window_size(width, height) {
                    config.ui_config.window_config.free_width = width;
                    config.ui_config.window_config.free_height = height;
                }
            } else if width >= window::MIN_WIDTH {
                config.ui_config.window_config.free_width = width;
            }
        } else if let Some(height) = window_settings.get("free_height").and_then(|v| v.as_f64()) {
            if height >= window::MIN_HEIGHT {
                config.ui_config.window_config.free_height = height;
            }
        }

        // 兼容旧的width/height参数，更新当前模式的尺寸（添加尺寸验证）
        if let (Some(width), Some(height)) = (
            window_settings.get("width").and_then(|v| v.as_f64()),
            window_settings.get("height").and_then(|v| v.as_f64()),
        ) {
            if validation::is_valid_window_size(width, height) {
                config
                    .ui_config
                    .window_config
                    .update_current_size(width, height);
            }
        }
    }

    // 保存配置到文件
    save_config(&state, &app)
        .await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn send_mcp_response(
    response: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 将响应序列化为JSON字符串
    let response_str =
        serde_json::to_string(&response).map_err(|e| format!("序列化响应失败: {}", e))?;

    if response_str.trim().is_empty() {
        return Err("响应内容不能为空".to_string());
    }

    // 检查是否为MCP模式
    let args: Vec<String> = std::env::args().collect();
    let is_mcp_mode = args.len() >= 3 && args[1] == "--mcp-request";

    if is_mcp_mode {
        // MCP模式：直接输出到stdout（MCP协议要求）
        println!("{}", response_str);
        std::io::Write::flush(&mut std::io::stdout())
            .map_err(|e| format!("刷新stdout失败: {}", e))?;
    } else {
        // 通过channel发送响应（如果有的话）
        let sender = {
            let mut channel = state
                .response_channel
                .lock()
                .map_err(|e| format!("获取响应通道失败: {}", e))?;
            channel.take()
        };

        if let Some(sender) = sender {
            let _ = sender.send(response_str);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_cli_args() -> Result<serde_json::Value, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut result = serde_json::Map::new();

    // 检查是否有 --mcp-request 参数
    if args.len() >= 3 && args[1] == "--mcp-request" {
        result.insert(
            "mcp_request".to_string(),
            serde_json::Value::String(args[2].clone()),
        );
    }

    Ok(serde_json::Value::Object(result))
}

#[tauri::command]
pub fn read_mcp_request(file_path: String) -> Result<serde_json::Value, String> {
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            if content.trim().is_empty() {
                return Err("文件内容为空".to_string());
            }
            match serde_json::from_str(&content) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("解析JSON失败: {}", e)),
            }
        }
        Err(e) => Err(format!("读取文件失败: {}", e)),
    }
}

#[tauri::command]
pub async fn select_image_files() -> Result<Vec<String>, String> {
    // 简化版本：返回测试图片数据
    // 在实际应用中，这里应该调用系统文件对话框
    let test_image_base64 = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCIgZmlsbD0iIzMzNzNkYyIvPgogIDx0ZXh0IHg9IjUwIiB5PSI1NSIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSJ3aGl0ZSIgdGV4dC1hbmNob3I9Im1pZGRsZSI+VGF1cmk8L3RleHQ+Cjwvc3ZnPg==";

    Ok(vec![test_image_base64.to_string()])
}

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), String> {
    use std::process::Command;

    // 移除不重要的调试信息

    // 根据操作系统选择合适的命令
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(&url)
            .spawn()
    } else {
        // Linux 和其他 Unix 系统
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("无法打开链接: {}", e))
    }
}

#[tauri::command]
pub async fn exit_app(app: AppHandle) -> Result<(), String> {
    // 直接调用强制退出，用于程序内部的退出操作（如MCP响应后退出）
    crate::ui::exit::force_exit_app(app).await
}



/// 处理应用退出请求（用于前端退出快捷键）
#[tauri::command]
pub async fn handle_app_exit_request(app: AppHandle) -> Result<bool, String> {
    crate::ui::exit_handler::handle_exit_request_internal(app).await
}

/// 构建发送操作的MCP响应
#[tauri::command]
pub fn build_mcp_send_response(
    user_input: Option<String>,
    selected_options: Vec<String>,
    images: Vec<ImageAttachment>,
    request_id: Option<String>,
    source: String,
) -> Result<String, String> {
    Ok(build_send_response(
        user_input,
        selected_options,
        images,
        request_id,
        &source,
    ))
}

/// 构建继续操作的MCP响应
#[tauri::command]
pub fn build_mcp_continue_response(
    request_id: Option<String>,
    source: String,
) -> Result<String, String> {
    Ok(build_continue_response(request_id, &source))
}
