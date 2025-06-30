#!/usr/bin/env python3
"""
简单的MCP测试 - 模拟AI工具调用
"""

import json
import subprocess
import sys
import time

def test_mcp_call():
    """测试MCP调用"""
    print("🧪 测试MCP调用功能")
    print("=" * 50)
    
    try:
        # 启动MCP服务器进程
        print("1. 连接到MCP服务器...")
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
        
        # 发送初始化请求
        print("2. 发送初始化请求...")
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
        
        server_process.stdin.write(json.dumps(init_request) + "\n")
        server_process.stdin.flush()
        
        # 读取初始化响应
        response = server_process.stdout.readline()
        if response:
            print("✅ 初始化成功")
        else:
            print("❌ 初始化失败")
            return False
        
        # 发送zhi工具调用
        print("3. 调用zhi工具...")
        print("   📝 这将启动一个交互界面，请完成用户交互")
        
        tool_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "zhi",
                "arguments": {
                    "message": "# 🎯 测试消息\n\n这是一个测试消息，用于验证CLI项目的MCP交互功能是否正常工作。\n\n## 测试内容\n- MCP协议通信\n- 独立UI进程启动\n- 用户交互处理\n- 响应返回机制",
                    "predefined_options": ["测试通过", "需要调试", "重新测试"],
                    "is_markdown": True,
                    "terminal_mode": False
                }
            }
        }
        
        server_process.stdin.write(json.dumps(tool_request) + "\n")
        server_process.stdin.flush()
        
        print("⏳ 等待用户完成交互...")
        print("   (请在弹出的交互界面中选择一个选项)")
        
        # 等待响应
        start_time = time.time()
        timeout = 30  # 30秒超时
        
        while time.time() - start_time < timeout:
            try:
                response = server_process.stdout.readline()
                if response.strip():
                    response_data = json.loads(response.strip())
                    print(f"✅ 收到响应: {response_data}")
                    
                    # 检查响应内容
                    if 'result' in response_data:
                        result = response_data['result']
                        if 'content' in result and result['content']:
                            user_response = result['content'][0].get('text', '')
                            print(f"📝 用户选择: {user_response}")
                            print("🎉 测试成功！MCP交互功能正常工作！")
                            return True
                    break
            except json.JSONDecodeError:
                continue
            except Exception as e:
                print(f"❌ 读取响应时出错: {e}")
                break
            
            time.sleep(0.1)
        
        print("❌ 测试超时或失败")
        return False
        
    except Exception as e:
        print(f"❌ 测试异常: {e}")
        return False
    finally:
        # 清理进程
        if 'server_process' in locals():
            server_process.terminate()
            server_process.wait()

def main():
    """主函数"""
    print("🚀 CLI项目MCP交互功能测试")
    print("=" * 60)
    
    success = test_mcp_call()
    
    print("\n" + "=" * 60)
    if success:
        print("🎉 测试结果: 成功！")
        print("✅ CLI项目的MCP交互功能已正常工作")
        print("✅ 独立UI进程方案修复有效")
        print("✅ 可以正常响应AI工具的调用")
    else:
        print("❌ 测试结果: 失败")
        print("需要进一步调试和修复")
    
    return success

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
