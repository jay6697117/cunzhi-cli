#!/bin/bash

# 寸止 CLI 卸载脚本
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
    echo "🗑️  寸止 CLI 卸载程序"
    echo "===================="
    echo -e "${NC}"
}

# 检查是否安装
check_installation() {
    print_info "检查寸止 CLI 安装状态..."
    
    if command -v cunzhi &> /dev/null; then
        local version=$(cunzhi --version 2>/dev/null || echo "未知版本")
        print_info "找到已安装的寸止 CLI: $version"
        return 0
    else
        print_warning "未找到已安装的寸止 CLI"
        return 1
    fi
}

# 查找安装位置
find_installation_paths() {
    local paths=()
    
    # 检查常见安装位置
    local common_paths=(
        "/usr/local/bin/cunzhi"
        "$HOME/.cargo/bin/cunzhi"
        "/usr/bin/cunzhi"
    )
    
    for path in "${common_paths[@]}"; do
        if [ -f "$path" ]; then
            paths+=("$path")
        fi
    done
    
    # 使用 which 命令查找
    if command -v cunzhi &> /dev/null; then
        local which_path=$(which cunzhi 2>/dev/null)
        if [ -n "$which_path" ] && [ -f "$which_path" ]; then
            # 检查是否已在列表中
            local found=false
            for existing_path in "${paths[@]}"; do
                if [ "$existing_path" = "$which_path" ]; then
                    found=true
                    break
                fi
            done
            if [ "$found" = false ]; then
                paths+=("$which_path")
            fi
        fi
    fi
    
    echo "${paths[@]}"
}

# 移除二进制文件
remove_binaries() {
    local paths=($(find_installation_paths))
    
    if [ ${#paths[@]} -eq 0 ]; then
        print_warning "未找到寸止 CLI 二进制文件"
        return 0
    fi
    
    print_info "找到以下安装位置:"
    for path in "${paths[@]}"; do
        echo "  - $path"
    done
    
    echo ""
    read -p "是否删除这些文件？(y/N): " confirm
    
    if [[ $confirm =~ ^[Yy]$ ]]; then
        for path in "${paths[@]}"; do
            if [ -f "$path" ]; then
                # 检查是否需要 sudo
                if [ -w "$(dirname "$path")" ]; then
                    rm "$path"
                    print_success "已删除: $path"
                else
                    sudo rm "$path"
                    print_success "已删除: $path (需要管理员权限)"
                fi
            fi
        done
    else
        print_info "跳过删除二进制文件"
    fi
}

# 移除配置文件
remove_config() {
    print_info "检查配置文件..."
    
    local config_paths=(
        "$HOME/.config/cunzhi"
        "$HOME/Library/Application Support/cunzhi"  # macOS
        "$HOME/.cunzhi"
    )
    
    local found_configs=()
    for path in "${config_paths[@]}"; do
        if [ -d "$path" ] || [ -f "$path" ]; then
            found_configs+=("$path")
        fi
    done
    
    if [ ${#found_configs[@]} -eq 0 ]; then
        print_info "未找到配置文件"
        return 0
    fi
    
    print_warning "找到以下配置文件/目录:"
    for path in "${found_configs[@]}"; do
        echo "  - $path"
    done
    
    echo ""
    read -p "是否删除配置文件？(y/N): " confirm
    
    if [[ $confirm =~ ^[Yy]$ ]]; then
        for path in "${found_configs[@]}"; do
            if [ -d "$path" ]; then
                rm -rf "$path"
                print_success "已删除目录: $path"
            elif [ -f "$path" ]; then
                rm "$path"
                print_success "已删除文件: $path"
            fi
        done
    else
        print_info "保留配置文件"
    fi
}

# 使用 cargo uninstall
uninstall_with_cargo() {
    print_info "尝试使用 cargo uninstall 卸载..."
    
    if command -v cargo &> /dev/null; then
        if cargo uninstall cunzhi-cli 2>/dev/null; then
            print_success "通过 cargo uninstall 卸载成功"
            return 0
        else
            print_warning "cargo uninstall 未找到安装的包"
            return 1
        fi
    else
        print_warning "未找到 cargo 命令"
        return 1
    fi
}

# 验证卸载
verify_uninstallation() {
    print_info "验证卸载..."
    
    if command -v cunzhi &> /dev/null; then
        print_warning "cunzhi 命令仍然可用，可能存在其他安装位置"
        local remaining_path=$(which cunzhi 2>/dev/null)
        if [ -n "$remaining_path" ]; then
            print_info "剩余安装位置: $remaining_path"
        fi
        return 1
    else
        print_success "卸载验证成功，cunzhi 命令不再可用"
        return 0
    fi
}

# 主卸载流程
main() {
    print_header
    
    # 检查安装状态
    if ! check_installation; then
        print_info "寸止 CLI 似乎未安装，但仍会检查残留文件"
    fi
    
    echo ""
    print_warning "此操作将卸载寸止 CLI 及其相关文件"
    read -p "确定要继续吗？(y/N): " confirm
    
    if [[ ! $confirm =~ ^[Yy]$ ]]; then
        print_info "取消卸载"
        exit 0
    fi
    
    echo ""
    print_info "开始卸载过程..."
    
    # 尝试使用 cargo uninstall
    if ! uninstall_with_cargo; then
        print_info "使用手动卸载方式..."
        remove_binaries
    fi
    
    # 询问是否删除配置文件
    echo ""
    remove_config
    
    # 验证卸载
    echo ""
    verify_uninstallation
    
    echo ""
    print_success "🎉 寸止 CLI 卸载完成！"
    print_info "感谢您使用寸止 CLI"
}

# 运行主程序
main "$@"
