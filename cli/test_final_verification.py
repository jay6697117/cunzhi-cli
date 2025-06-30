#!/usr/bin/env python3
"""
最终验证测试 - 验证CLI项目的MCP交互功能
"""

import json
import subprocess
import sys
import time
import threading
import os

def test_direct_ui_process():
    """直接测试独立UI进程"""
    print("🧪 测试1: 直接测试独立UI进程")
    print("=" * 50)
    
    # 创建测试请求文件
    test_request = {
        "id": "test-final-verification",
        "message": "# 🎯 最终验证测试\n\n这是最终的验证测试，确认CLI项目的MCP交互功能正常工作。",
        "predefined_options": ["验证成功", "需要调试"],
        "is_markdown": True
    }
    
    with open("test_final_request.json", "w") as f:
        json.dump(test_request, f, indent=2)
    
    try:
        # 直接调用cunzhi-ui
        print("启动独立UI进程...")
        result = subprocess.run([
            "cargo", "run", "--bin", "cunzhi-ui", "--", 
            "--mcp-request", "test_final_request.json"
        ], capture_output=True, text=True, timeout=30)
        
        if result.returncode == 0:
            print(f"✅ UI进程成功返回: {result.stdout.strip()}")
            return True
        else:
            print(f"❌ UI进程失败: {result.stderr}")
            return False
    except subprocess.TimeoutExpired:
        print("❌ UI进程超时")
        return False
    except Exception as e:
        print(f"❌ UI进程异常: {e}")
        return False
    finally:
        # 清理临时文件
        if os.path.exists("test_final_request.json"):
            os.remove("test_final_request.json")

def test_mcp_integration():
    """测试MCP集成"""
    print("\n🧪 测试2: MCP集成测试")
    print("=" * 50)
    
    try:
        # 启动MCP服务器
        print("启动MCP服务器...")
        server_process = subprocess.Popen(
            ["cargo", "run", "--bin", "cunzhi-server"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=0
        )
        
        # 等待服务器启动
        time.sleep(3)
        
        # 发送初始化请求
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "test-client", "version": "1.0.0"}
            }
        }
        
        print("发送初始化请求...")
        server_process.stdin.write(json.dumps(init_request) + "\n")
        server_process.stdin.flush()
        
        # 读取初始化响应
        init_response = server_process.stdout.readline()
        if init_response:
            print("✅ 初始化成功")
        else:
            print("❌ 初始化失败")
            return False
        
        # 发送工具调用请求
        tool_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "zhi",
                "arguments": {
                    "message": "# 🔧 MCP集成测试\n\n这是通过MCP协议调用的测试。如果您看到这个消息，说明集成成功！",
                    "predefined_options": ["集成成功", "需要调试"],
                    "is_markdown": True,
                    "terminal_mode": False
                }
            }
        }
        
        print("发送工具调用请求...")
        print("⚠️  注意：这将启动一个交互界面，请在新窗口中完成交互")
        
        server_process.stdin.write(json.dumps(tool_request) + "\n")
        server_process.stdin.flush()
        
        # 等待响应（给用户足够时间进行交互）
        print("等待用户交互完成...")
        time.sleep(15)
        
        # 尝试读取响应
        try:
            response = server_process.stdout.readline()
            if response.strip():
                response_data = json.loads(response.strip())
                print(f"✅ 收到MCP响应: {response_data}")
                return True
            else:
                print("❌ 没有收到MCP响应")
                return False
        except json.JSONDecodeError as e:
            print(f"❌ 响应解析失败: {e}")
            return False
        
    except Exception as e:
        print(f"❌ MCP集成测试失败: {e}")
        return False
    finally:
        # 清理进程
        if 'server_process' in locals():
            server_process.terminate()
            server_process.wait()

def main():
    """主测试函数"""
    print("🚀 CLI项目MCP交互功能最终验证")
    print("=" * 60)
    
    # 测试1: 直接UI进程测试
    ui_test_passed = test_direct_ui_process()
    
    # 测试2: MCP集成测试
    mcp_test_passed = test_mcp_integration()
    
    # 总结
    print("\n" + "=" * 60)
    print("📊 测试结果总结:")
    print(f"  独立UI进程测试: {'✅ 通过' if ui_test_passed else '❌ 失败'}")
    print(f"  MCP集成测试: {'✅ 通过' if mcp_test_passed else '❌ 失败'}")
    
    if ui_test_passed and mcp_test_passed:
        print("\n🎉 所有测试通过！CLI项目的MCP交互功能已成功修复！")
        print("\n📋 修复总结:")
        print("  ✅ 实现了独立UI进程方案")
        print("  ✅ 解决了stdio模式下无法交互的问题")
        print("  ✅ 保持了与AI工具的兼容性")
        print("  ✅ 提供了现代化的命令行交互界面")
        return True
    else:
        print("\n❌ 部分测试失败，需要进一步调试")
        return False

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
