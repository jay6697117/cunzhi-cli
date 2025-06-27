#!/bin/bash

# 寸止 MCP 工具安装脚本 - 支持 macOS、Linux
# 只需要构建和安装两个CLI工具即可运行MCP

set -e

echo "🚀 开始安装 寸止 MCP 工具..."

# 检测操作系统
OS="unknown"
case "$OSTYPE" in
    darwin*)  OS="macos" ;;
    linux*)   OS="linux" ;;
    msys*|cygwin*|mingw*) OS="windows" ;;
    *)        echo "❌ 不支持的操作系统: $OSTYPE"; exit 1 ;;
esac

echo "🔍 检测到操作系统: $OS"

# 检查必要的工具
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo "❌ 错误: 未找到 $1 命令"
        echo "请先安装 $1"
        exit 1
    fi
}

echo "🔧 检查必要工具..."
check_command "cargo"
check_command "pnpm"

# 构建前端资源（MCP弹窗界面需要）
echo "📦 构建前端资源..."
pnpm build

# 构建MCP CLI工具
echo "🔨 构建 MCP CLI 工具..."
cargo build --release

# 检查构建结果
if [[ ! -f "target/release/等一下" ]] || [[ ! -f "target/release/寸止" ]]; then
    echo "❌ CLI 工具构建失败"
    echo "请检查构建错误并重试"
    exit 1
fi

echo "✅ CLI 工具构建成功"

# 根据操作系统安装CLI工具
if [[ "$OS" == "macos" ]]; then
    echo "🍎 macOS 安装模式..."

    # 安装到 /usr/local/bin
    INSTALL_DIR="/usr/local/bin"

    echo "📋 安装 CLI 工具到 $INSTALL_DIR..."
    sudo cp "target/release/等一下" "$INSTALL_DIR/"
    sudo cp "target/release/寸止" "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/等一下"
    sudo chmod +x "$INSTALL_DIR/寸止"

    echo "✅ CLI 工具已安装到 $INSTALL_DIR"

elif [[ "$OS" == "linux" ]]; then
    echo "🐧 Linux 安装模式..."

    # 创建用户本地目录
    LOCAL_DIR="$HOME/.local"
    BIN_DIR="$LOCAL_DIR/bin"

    mkdir -p "$BIN_DIR"

    # 复制CLI工具
    cp "target/release/等一下" "$BIN_DIR/"
    cp "target/release/寸止" "$BIN_DIR/"
    chmod +x "$BIN_DIR/等一下"
    chmod +x "$BIN_DIR/寸止"

    echo "✅ CLI 工具已安装到 $BIN_DIR"

    # 检查PATH
    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        echo ""
        echo "💡 请将以下内容添加到您的 shell 配置文件中 (~/.bashrc 或 ~/.zshrc):"
        echo "export PATH=\"\$PATH:$BIN_DIR\""
        echo ""
        echo "然后运行: source ~/.bashrc (或 source ~/.zshrc)"
    fi

else
    echo "❌ Windows 平台请使用 Windows 专用安装程序"
    exit 1
fi

echo ""
echo "🎉 寸止 MCP 工具安装完成！"
echo ""
echo "📋 使用方法："
echo "  💻 MCP 服务器模式:"
echo "    寸止                            - 启动 MCP 服务器"
echo ""
echo "  🎨 弹窗界面模式:"
echo "    等一下                          - 启动设置界面"
echo "    等一下 --mcp-request file       - MCP 弹窗模式"
echo ""
echo "📝 配置 MCP 客户端："
echo "将以下内容添加到您的 MCP 客户端配置中："
echo ""
cat << 'EOF'
{
  "mcpServers": {
    "寸止": {
      "command": "寸止"
    }
  }
}
EOF
echo ""
echo "💡 重要说明："
echo "  • 两个CLI工具必须在同一目录下才能正常工作"
echo "  • '寸止' 是MCP服务器，'等一下' 是弹窗界面"
echo "  • 无需安装完整应用，只需要这两个CLI工具即可"
echo ""

if [[ "$OS" == "macos" ]]; then
    echo "🔗 CLI 工具已安装到 /usr/local/bin/"
elif [[ "$OS" == "linux" ]]; then
    echo "🔗 CLI 工具已安装到 $BIN_DIR"
fi
