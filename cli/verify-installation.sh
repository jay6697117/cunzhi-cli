#!/bin/bash

# 寸止 CLI 安装验证脚本
# 验证全局安装是否正确工作

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
    echo "🔍 寸止 CLI 安装验证"
    echo "==================="
    echo -e "${NC}"
}

# 测试计数器
TESTS_TOTAL=0
TESTS_PASSED=0
TESTS_FAILED=0

# 运行测试
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    print_info "测试 $TESTS_TOTAL: $test_name"
    
    if eval "$test_command" >/dev/null 2>&1; then
        print_success "通过"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        print_error "失败"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# 运行带输出的测试
run_test_with_output() {
    local test_name="$1"
    local test_command="$2"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    print_info "测试 $TESTS_TOTAL: $test_name"
    
    local output
    if output=$(eval "$test_command" 2>&1); then
        print_success "通过"
        echo "  输出: $output"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        print_error "失败"
        echo "  错误: $output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# 主验证流程
main() {
    print_header
    
    # 基础命令测试
    print_info "🔧 基础命令测试"
    run_test "命令可用性" "command -v cunzhi"
    run_test_with_output "版本信息" "cunzhi --version"
    run_test "帮助信息" "cunzhi --help"
    
    echo ""
    
    # 子命令测试
    print_info "📋 子命令测试"
    run_test "init 命令帮助" "cunzhi init --help"
    run_test "server 命令帮助" "cunzhi server --help"
    run_test "config 命令帮助" "cunzhi config --help"
    run_test "doctor 命令" "cunzhi doctor"
    
    echo ""
    
    # 功能测试
    print_info "⚙️ 功能测试"
    
    # 创建临时目录进行测试
    local temp_dir=$(mktemp -d)
    local old_config_dir="$CUNZHI_CONFIG_DIR"
    export CUNZHI_CONFIG_DIR="$temp_dir"
    
    run_test "快速初始化" "cunzhi init --name test-verify --yes"
    run_test "配置显示" "cunzhi config show"
    run_test "服务器状态" "cunzhi server status"
    
    # 恢复环境
    export CUNZHI_CONFIG_DIR="$old_config_dir"
    rm -rf "$temp_dir"
    
    echo ""
    
    # 性能测试
    print_info "⚡ 性能测试"
    
    local start_time=$(date +%s%N)
    cunzhi --version >/dev/null 2>&1
    local end_time=$(date +%s%N)
    local duration=$(( (end_time - start_time) / 1000000 )) # 转换为毫秒
    
    if [ $duration -lt 1000 ]; then
        print_success "启动速度: ${duration}ms (良好)"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        print_warning "启动速度: ${duration}ms (较慢)"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    
    echo ""
    
    # 安装位置检查
    print_info "📍 安装位置检查"
    local cunzhi_path=$(which cunzhi 2>/dev/null || echo "未找到")
    print_info "安装位置: $cunzhi_path"
    
    if [ -f "$cunzhi_path" ]; then
        local file_size=$(ls -lh "$cunzhi_path" | awk '{print $5}')
        print_info "文件大小: $file_size"
        
        # 检查权限
        if [ -x "$cunzhi_path" ]; then
            print_success "执行权限: 正常"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            print_error "执行权限: 缺失"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
        TESTS_TOTAL=$((TESTS_TOTAL + 1))
    fi
    
    echo ""
    
    # 依赖检查
    print_info "🔗 依赖检查"
    
    # 检查动态链接库（Linux/macOS）
    if command -v ldd >/dev/null 2>&1; then
        print_info "动态链接库检查 (Linux):"
        ldd "$cunzhi_path" 2>/dev/null | head -5 || print_warning "无法检查动态链接库"
    elif command -v otool >/dev/null 2>&1; then
        print_info "动态链接库检查 (macOS):"
        otool -L "$cunzhi_path" 2>/dev/null | head -5 || print_warning "无法检查动态链接库"
    fi
    
    echo ""
    
    # 总结
    print_info "📊 测试总结"
    echo "  总测试数: $TESTS_TOTAL"
    echo "  通过: $TESTS_PASSED"
    echo "  失败: $TESTS_FAILED"
    
    local success_rate=$((TESTS_PASSED * 100 / TESTS_TOTAL))
    
    if [ $TESTS_FAILED -eq 0 ]; then
        print_success "🎉 所有测试通过！安装验证成功 (100%)"
        echo ""
        print_info "寸止 CLI 已正确安装并可以使用"
        print_info "运行 'cunzhi --help' 查看可用命令"
        exit 0
    elif [ $success_rate -ge 80 ]; then
        print_warning "⚠️  大部分测试通过 ($success_rate%)，但存在一些问题"
        echo ""
        print_info "寸止 CLI 基本可用，但建议检查失败的测试项"
        exit 1
    else
        print_error "❌ 多个测试失败 ($success_rate%)，安装可能有问题"
        echo ""
        print_info "建议重新安装或检查系统环境"
        exit 2
    fi
}

# 运行主程序
main "$@"
