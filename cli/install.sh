#!/bin/bash

# 寸止 CLI 安装脚本
# 支持 macOS 和 Linux 系统

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_header() {
    echo -e "${BLUE}"
    echo "🚀 寸止 CLI 安装程序"
    echo "===================="
    echo -e "${NC}"
}

# 检查系统要求
check_requirements() {
    print_info "检查系统要求..."
    
    # 检查 Rust
    if ! command -v rustc &> /dev/null; then
        print_error "未找到 Rust 编译器"
        print_info "请先安装 Rust: https://rustup.rs/"
        exit 1
    fi
    
    # 检查 Cargo
    if ! command -v cargo &> /dev/null; then
        print_error "未找到 Cargo 包管理器"
        exit 1
    fi
    
    # 显示 Rust 版本
    rust_version=$(rustc --version)
    print_success "找到 Rust: $rust_version"
    
    # 检查 Rust 版本是否满足要求
    rust_major=$(rustc --version | grep -oE '[0-9]+\.[0-9]+' | head -1 | cut -d. -f1)
    rust_minor=$(rustc --version | grep -oE '[0-9]+\.[0-9]+' | head -1 | cut -d. -f2)
    
    if [ "$rust_major" -lt 1 ] || ([ "$rust_major" -eq 1 ] && [ "$rust_minor" -lt 70 ]); then
        print_warning "Rust 版本可能过旧，建议使用 1.70.0 或更高版本"
        print_info "运行 'rustup update' 更新 Rust"
    fi
}

# 构建项目
build_project() {
    print_info "构建寸止 CLI..."
    
    # 进入项目目录
    if [ ! -f "Cargo.toml" ]; then
        print_error "未找到 Cargo.toml 文件，请确保在正确的目录中运行此脚本"
        exit 1
    fi
    
    # 构建发布版本
    if cargo build --release; then
        print_success "构建完成"
    else
        print_error "构建失败"
        exit 1
    fi
}

# 安装到系统
install_binary() {
    print_info "安装寸止 CLI 到系统..."
    
    local binary_path="target/release/cunzhi"
    
    if [ ! -f "$binary_path" ]; then
        print_error "未找到构建的二进制文件: $binary_path"
        exit 1
    fi
    
    # 检查安装目标目录
    local install_dir="/usr/local/bin"
    
    if [ ! -d "$install_dir" ]; then
        print_warning "目录 $install_dir 不存在，尝试创建..."
        sudo mkdir -p "$install_dir"
    fi
    
    # 复制二进制文件
    if sudo cp "$binary_path" "$install_dir/cunzhi"; then
        print_success "已安装到 $install_dir/cunzhi"
    else
        print_error "安装失败"
        exit 1
    fi
    
    # 设置执行权限
    sudo chmod +x "$install_dir/cunzhi"
    print_success "已设置执行权限"
}

# 验证安装
verify_installation() {
    print_info "验证安装..."
    
    if command -v cunzhi &> /dev/null; then
        local version=$(cunzhi --version 2>/dev/null || echo "未知版本")
        print_success "安装成功！版本: $version"
        
        print_info "可用命令:"
        echo "  cunzhi --help          # 查看帮助"
        echo "  cunzhi init             # 初始化项目"
        echo "  cunzhi server start     # 启动 MCP 服务器"
        echo "  cunzhi config show      # 查看配置"
        
    else
        print_error "安装验证失败，cunzhi 命令不可用"
        print_info "请检查 /usr/local/bin 是否在您的 PATH 中"
        print_info "运行: echo \$PATH | grep -o /usr/local/bin"
        exit 1
    fi
}

# 使用 cargo install 的替代方法
install_with_cargo() {
    print_info "使用 cargo install 安装..."
    
    if cargo install --path .; then
        print_success "通过 cargo install 安装成功"
        return 0
    else
        print_error "cargo install 失败"
        return 1
    fi
}

# 主安装流程
main() {
    print_header
    
    # 检查是否以 root 身份运行
    if [ "$EUID" -eq 0 ]; then
        print_warning "不建议以 root 身份运行此脚本"
        print_info "建议使用普通用户运行，需要时会提示输入密码"
    fi
    
    # 检查系统要求
    check_requirements
    
    # 询问安装方式
    echo ""
    print_info "选择安装方式:"
    echo "1) cargo install (推荐)"
    echo "2) 手动构建和安装"
    echo ""
    read -p "请选择 (1 或 2): " choice
    
    case $choice in
        1)
            print_info "使用 cargo install 方式..."
            if install_with_cargo; then
                verify_installation
            else
                print_warning "cargo install 失败，尝试手动安装..."
                build_project
                install_binary
                verify_installation
            fi
            ;;
        2)
            print_info "使用手动构建方式..."
            build_project
            install_binary
            verify_installation
            ;;
        *)
            print_error "无效选择"
            exit 1
            ;;
    esac
    
    echo ""
    print_success "🎉 寸止 CLI 安装完成！"
    print_info "运行 'cunzhi --help' 开始使用"
}

# 运行主程序
main "$@"
