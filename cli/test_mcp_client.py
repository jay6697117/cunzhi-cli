#!/usr/bin/env python3
"""
简单的MCP客户端测试脚本
用于测试与cunzhi-cli MCP服务器的连接
"""

import json
import subprocess
import sys
import time

def test_mcp_connection():
    """测试MCP连接"""
    print("🧪 测试MCP客户端连接")
    print("=" * 50)

    # 启动MCP服务器进程
    print("1. 启动MCP服务器...")
    try:
        # 使用cargo运行MCP服务器
        server_process = subprocess.Popen(
            ["cargo", "run", "--bin", "cunzhi-server"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=0
        )

        # 等待服务器启动
        time.sleep(2)

        print("2. 发送初始化请求...")

        # 构建初始化请求
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "roots": {
                        "listChanged": True
                    },
                    "sampling": {}
                },
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            }
        }

        # 发送请求
        request_json = json.dumps(init_request) + "\n"
        server_process.stdin.write(request_json)
        server_process.stdin.flush()

        # 读取响应
        response_line = server_process.stdout.readline()
        if response_line:
            response = json.loads(response_line.strip())
            print(f"✅ 初始化响应: {response}")
        else:
            print("❌ 没有收到初始化响应")
            return False

        print("3. 发送工具列表请求...")

        # 构建工具列表请求
        tools_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }

        request_json = json.dumps(tools_request) + "\n"
        server_process.stdin.write(request_json)
        server_process.stdin.flush()

        # 读取响应
        response_line = server_process.stdout.readline()
        if response_line:
            response = json.loads(response_line.strip())
            print(f"✅ 工具列表响应: {response}")

            # 检查是否有zhi工具
            if 'result' in response and 'tools' in response['result']:
                tools = response['result']['tools']
                zhi_tool = next((tool for tool in tools if tool['name'] == 'zhi'), None)
                if zhi_tool:
                    print("✅ 找到zhi工具")
                    return test_zhi_tool(server_process)
                else:
                    print("❌ 没有找到zhi工具")
                    return False
        else:
            print("❌ 没有收到工具列表响应")
            return False

    except Exception as e:
        print(f"❌ 测试失败: {e}")
        return False
    finally:
        # 清理进程
        if 'server_process' in locals():
            server_process.terminate()
            server_process.wait()

def test_zhi_tool(server_process):
    """测试zhi工具调用"""
    print("4. 测试zhi工具调用...")

    # 构建zhi工具调用请求
    zhi_request = {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "zhi",
            "arguments": {
                "message": "这是一个测试消息",
                "predefined_options": ["选项1", "选项2", "取消"],
                "is_markdown": False,
                "terminal_mode": False
            }
        }
    }

    request_json = json.dumps(zhi_request) + "\n"
    server_process.stdin.write(request_json)
    server_process.stdin.flush()

    # 等待一段时间让交互完成
    print("等待用户交互完成...")
    time.sleep(3)

    # 尝试读取多行响应
    try:
        response_line = server_process.stdout.readline()
        if response_line.strip():
            response = json.loads(response_line.strip())
            print(f"✅ zhi工具响应: {response}")
            return True
        else:
            print("❌ 没有收到zhi工具响应")
            # 检查stderr
            error_line = server_process.stderr.readline()
            if error_line:
                print(f"错误信息: {error_line.strip()}")
            return False
    except json.JSONDecodeError as e:
        print(f"❌ JSON解析错误: {e}")
        print(f"原始响应: {response_line}")
        return False

if __name__ == "__main__":
    success = test_mcp_connection()
    if success:
        print("\n🎉 MCP连接测试成功！")
        sys.exit(0)
    else:
        print("\n❌ MCP连接测试失败！")
        sys.exit(1)
