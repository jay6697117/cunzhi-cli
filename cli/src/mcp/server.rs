// MCP 服务器实现 - 简化版本
use anyhow::Result;
use std::collections::HashMap;
use serde_json::Value;

use super::tools::{InteractionTool, MemoryTool};
use super::types::{ZhiRequest, JiyiRequest, McpError, CallToolResult};
use crate::config::load_standalone_config;
use crate::{log_important, log_debug};

/// 服务器运行模式
#[derive(Debug, Clone)]
pub enum ServerMode {
    /// 标准输入输出模式
    Stdio,
    /// HTTP 服务器模式
    Http { port: u16 },
}

/// 简化的MCP服务器
#[derive(Clone)]
pub struct ZhiServer {
    enabled_tools: HashMap<String, bool>,
}

impl Default for ZhiServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZhiServer {
    pub fn new() -> Self {
        // 从配置加载启用的工具
        let enabled_tools = match load_standalone_config() {
            Ok(config) => config.mcp_config.tools,
            Err(e) => {
                log_important!(warn, "无法加载配置文件，使用默认工具配置: {}", e);
                let mut tools = HashMap::new();
                tools.insert("zhi".to_string(), true);
                tools.insert("ji".to_string(), true);
                tools
            }
        };

        Self {
            enabled_tools,
        }
    }

    /// 检查工具是否启用
    pub fn is_tool_enabled(&self, tool_name: &str) -> bool {
        // 动态读取最新配置
        match load_standalone_config() {
            Ok(config) => {
                let enabled = config.mcp_config.tools.get(tool_name).copied().unwrap_or(true);
                log_debug!("工具 {} 当前状态: {}", tool_name, enabled);
                enabled
            }
            Err(e) => {
                log_important!(warn, "读取配置失败，使用缓存状态: {}", e);
                self.enabled_tools.get(tool_name).copied().unwrap_or(true)
            }
        }
    }

    /// 获取启用的工具列表
    pub fn get_enabled_tools(&self) -> Vec<String> {
        match load_standalone_config() {
            Ok(config) => {
                config.mcp_config.tools
                    .iter()
                    .filter_map(|(name, &enabled)| if enabled { Some(name.clone()) } else { None })
                    .collect()
            }
            Err(_) => {
                self.enabled_tools
                    .iter()
                    .filter_map(|(name, &enabled)| if enabled { Some(name.clone()) } else { None })
                    .collect()
            }
        }
    }

    /// 启动MCP协议服务器
    pub async fn start(&self) -> Result<()> {
        self.start_with_mode(ServerMode::Stdio).await
    }

    /// 启动MCP协议服务器（指定模式）
    pub async fn start_with_mode(&self, mode: ServerMode) -> Result<()> {
        log_important!(info, "启动 MCP 服务器...");

        // 检查启用的工具
        let enabled_tools = self.get_enabled_tools();
        if enabled_tools.is_empty() {
            return Err(anyhow::anyhow!("没有启用的 MCP 工具"));
        }

        log_important!(info, "启用的工具: {:?}", enabled_tools);
        log_important!(info, "MCP 服务器已启动，等待连接...");
        log_important!(info, "可用工具:");
        for tool in &enabled_tools {
            match tool.as_str() {
                "zhi" => log_important!(info, "  ✅ zhi - 智能代码审查工具"),
                "ji" => log_important!(info, "  ✅ ji - 记忆管理工具"),
                _ => log_important!(info, "  ❓ {} - 未知工具", tool),
            }
        }

        match mode {
            ServerMode::Stdio => {
                log_important!(info, "使用 stdio 模式");
                self.run_mcp_protocol().await
            }
            ServerMode::Http { port } => {
                log_important!(info, "HTTP 模式暂未实现，使用 stdio 模式，端口: {}", port);
                self.run_mcp_protocol().await
            }
        }
    }

    /// 运行MCP协议服务器
    async fn run_mcp_protocol(&self) -> Result<()> {
        use std::io::{self, BufRead, Write};

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        // 读取stdin的每一行作为JSON-RPC请求
        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            // 解析JSON-RPC请求
            match self.handle_jsonrpc_request(&line).await {
                Ok(response) => {
                    // 发送响应
                    writeln!(stdout, "{}", response)?;
                    stdout.flush()?;
                }
                Err(e) => {
                    // 发送错误响应
                    let error_response = serde_json::json!({
                        "jsonrpc": "2.0",
                        "error": {
                            "code": -32000,
                            "message": e.to_string()
                        },
                        "id": null
                    });
                    writeln!(stdout, "{}", error_response)?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }



    /// 处理HTTP请求
    async fn handle_http_request(&self, req: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<http_body_util::Full<hyper::body::Bytes>>, std::convert::Infallible> {
        use hyper::{Method, StatusCode, Response};
        use hyper::body::Bytes;
        use http_body_util::{BodyExt, Full};
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/mcp") => {
                // 处理 MCP JSON-RPC 请求
                let body = match req.collect().await {
                    Ok(collected) => collected.to_bytes(),
                    Err(_) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Full::new(Bytes::from("Failed to read request body")))
                            .unwrap());
                    }
                };

                let request_str = match String::from_utf8(body.to_vec()) {
                    Ok(s) => s,
                    Err(_) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Full::new(Bytes::from("Invalid UTF-8 in request body")))
                            .unwrap());
                    }
                };

                match self.handle_jsonrpc_request(&request_str).await {
                    Ok(response) => {
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .header("Access-Control-Allow-Origin", "*")
                            .header("Access-Control-Allow-Methods", "POST, OPTIONS")
                            .header("Access-Control-Allow-Headers", "Content-Type")
                            .body(Full::new(Bytes::from(response)))
                            .unwrap())
                    }
                    Err(e) => {
                        let error_response = serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32000,
                                "message": e.to_string()
                            },
                            "id": null
                        });
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .header("Content-Type", "application/json")
                            .body(Full::new(Bytes::from(error_response.to_string())))
                            .unwrap())
                    }
                }
            }
            (&Method::OPTIONS, "/mcp") => {
                // 处理 CORS 预检请求
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "POST, OPTIONS")
                    .header("Access-Control-Allow-Headers", "Content-Type")
                    .body(Full::new(Bytes::new()))
                    .unwrap())
            }
            (&Method::GET, "/health") => {
                // 健康检查端点
                let health_info = serde_json::json!({
                    "status": "healthy",
                    "server": "cunzhi-cli MCP Server",
                    "version": "0.2.12",
                    "tools": self.get_enabled_tools()
                });
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(health_info.to_string())))
                    .unwrap())
            }
            _ => {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Full::new(Bytes::from("Not Found")))
                    .unwrap())
            }
        }
    }

    /// 处理JSON-RPC请求
    async fn handle_jsonrpc_request(&self, request: &str) -> Result<String> {
        log_debug!("收到 JSON-RPC 请求: {}", request);

        let req: Value = serde_json::from_str(request)
            .map_err(|e| anyhow::anyhow!("JSON 解析失败: {}", e))?;

        let method = req["method"].as_str().unwrap_or("");
        let params = &req["params"];
        let id = &req["id"];

        log_debug!("处理方法: {}, ID: {:?}", method, id);

        let result = match method {
            "initialize" => {
                // MCP初始化
                serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "cunzhi-cli",
                        "version": "0.2.12"
                    }
                })
            }
            "tools/list" => {
                // 列出工具
                let tools = self.list_tools();
                let tool_list: Vec<Value> = tools.into_iter().map(|tool| {
                    let input_schema = match tool.name.as_str() {
                        "zhi" | "zhi_cunzhi" => serde_json::json!({
                            "type": "object",
                            "properties": {
                                "message": {
                                    "type": "string",
                                    "description": "要显示给用户的消息"
                                },
                                "predefined_options": {
                                    "type": "array",
                                    "items": {"type": "string"},
                                    "description": "预定义的选项列表（可选）"
                                },
                                "is_markdown": {
                                    "type": "boolean",
                                    "description": "消息是否为Markdown格式，默认为true",
                                    "default": true
                                },
                                "terminal_mode": {
                                    "type": "boolean",
                                    "description": "是否在新终端窗口中启动交互，默认为false",
                                    "default": false
                                }
                            },
                            "required": ["message"]
                        }),
                        "ji" | "ji_cunzhi" => serde_json::json!({
                            "type": "object",
                            "properties": {
                                "action": {
                                    "type": "string",
                                    "description": "操作类型：记忆(添加记忆), 回忆(获取项目信息)",
                                    "enum": ["记忆", "回忆"]
                                },
                                "project_path": {
                                    "type": "string",
                                    "description": "项目路径（必需）"
                                },
                                "content": {
                                    "type": "string",
                                    "description": "记忆内容（记忆操作时必需）"
                                },
                                "category": {
                                    "type": "string",
                                    "description": "记忆分类：rule(规范规则), preference(用户偏好), pattern(最佳实践), context(项目上下文)",
                                    "enum": ["rule", "preference", "pattern", "context"],
                                    "default": "context"
                                }
                            },
                            "required": ["action", "project_path"]
                        }),
                        _ => serde_json::json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        })
                    };

                    serde_json::json!({
                        "name": tool.name,
                        "description": tool.description,
                        "inputSchema": input_schema
                    })
                }).collect();

                serde_json::json!({
                    "tools": tool_list
                })
            }
            "tools/call" => {
                // 调用工具
                let tool_name = params["name"].as_str().unwrap_or("");
                let arguments = params["arguments"].clone();

                match self.call_tool(tool_name, arguments).await {
                    Ok(result) => {
                        serde_json::json!({
                            "content": result.content
                        })
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!("工具调用失败: {}", e));
                    }
                }
            }
            _ => {
                return Err(anyhow::anyhow!("未知方法: {}", method));
            }
        };

        // 构建JSON-RPC响应
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id
        });

        Ok(response.to_string())
    }

    /// 处理工具调用请求
    pub async fn call_tool(&self, tool_name: &str, arguments: Value) -> Result<CallToolResult, McpError> {
        log_debug!("收到工具调用请求: {}", tool_name);

        // 工具名称映射 - 支持 Augment 环境中的工具名称
        let normalized_tool_name = match tool_name {
            "zhi_cunzhi" => "zhi",
            "ji_cunzhi" => "ji",
            name => name,
        };

        log_debug!("标准化工具名称: {} -> {}", tool_name, normalized_tool_name);

        match normalized_tool_name {
            "zhi" => {
                let zhi_request: ZhiRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::invalid_params(format!("参数解析失败: {}", e), None))?;

                InteractionTool::zhi(zhi_request).await
            }
            "ji" => {
                // 检查记忆管理工具是否启用
                if !self.is_tool_enabled("ji") {
                    return Err(McpError::internal_error(
                        "记忆管理工具已被禁用".to_string(),
                        None
                    ));
                }

                let ji_request: JiyiRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::invalid_params(format!("参数解析失败: {}", e), None))?;

                MemoryTool::jiyi(ji_request).await
            }
            _ => {
                Err(McpError::invalid_request(
                    format!("未知的工具: {} (标准化后: {})", tool_name, normalized_tool_name),
                    None
                ))
            }
        }
    }

    /// 获取工具列表
    pub fn list_tools(&self) -> Vec<ToolInfo> {
        let mut tools = Vec::new();

        // zhi 工具始终可用 - 同时注册两个名称以支持不同环境
        tools.push(ToolInfo {
            name: "zhi".to_string(),
            description: "智能代码审查工具".to_string(),
            enabled: self.is_tool_enabled("zhi"),
        });

        tools.push(ToolInfo {
            name: "zhi_cunzhi".to_string(),
            description: "智能代码审查工具 (Augment 兼容)".to_string(),
            enabled: self.is_tool_enabled("zhi"),
        });

        // ji 工具根据配置启用
        if self.is_tool_enabled("ji") {
            tools.push(ToolInfo {
                name: "ji".to_string(),
                description: "记忆管理工具".to_string(),
                enabled: true,
            });

            tools.push(ToolInfo {
                name: "ji_cunzhi".to_string(),
                description: "记忆管理工具 (Augment 兼容)".to_string(),
                enabled: true,
            });
        }

        tools
    }

    /// 停止服务器
    pub async fn stop(&self) -> Result<String> {
        log_important!(info, "正在停止 MCP 服务器...");
        // 在简化实现中，我们只是返回成功消息
        Ok("服务器已停止".to_string())
    }

    /// 获取服务器状态
    pub async fn status(&self) -> Result<String> {
        let enabled_tools = self.get_enabled_tools();
        if enabled_tools.is_empty() {
            Ok("未运行 - 没有启用的工具".to_string())
        } else {
            Ok(format!("运行中 - {} 个工具已启用", enabled_tools.len()))
        }
    }
}

/// 启动 MCP 服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let server = ZhiServer::new();
    server.start().await?;
    Ok(())
}

/// 工具信息
#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

/// 构建 MCP 响应的辅助函数
pub fn build_mcp_response(content: &str) -> serde_json::Value {
    serde_json::json!({
        "content": [
            {
                "type": "text",
                "text": content
            }
        ]
    })
}
