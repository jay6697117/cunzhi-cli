# 寸止 CLI

<div align="center">

![寸止 CLI](https://img.shields.io/badge/寸止-CLI-blue?style=for-the-badge)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

**现代化的智能代码审查工具**

一个功能强大的命令行工具，提供智能代码审查、项目记忆管理和 MCP 服务器功能。

[安装指南](#安装) • [快速开始](#快速开始) • [命令参考](#命令参考) • [配置说明](#配置说明)

</div>

## ✨ 特性

- 🚀 **现代化 CLI 体验** - 类似 create-vue 的交互式初始化向导
- 🔧 **智能代码审查** - 基于 MCP 协议的代码质量分析
- 🧠 **项目记忆管理** - 智能存储和检索项目知识
- ⚙️ **灵活配置** - 支持多种配置模板和自定义选项
- 🎨 **美观界面** - 彩色输出、进度条、表格显示
- 🛡️ **错误处理** - 友好的错误消息和恢复建议
- 📊 **服务器管理** - 完整的 MCP 服务器生命周期管理

## 📦 安装

### 系统要求

- Rust 1.70.0 或更高版本
- 支持的操作系统：Windows、macOS、Linux

### 方法 1: 自动安装脚本（推荐）

**Linux/macOS:**
```bash
# 克隆仓库
git clone https://github.com/your-org/cunzhi-cli.git
cd cunzhi-cli/cunzhi-cli

# 运行安装脚本
./install.sh
```

**Windows:**
```powershell
# 克隆仓库
git clone https://github.com/your-org/cunzhi-cli.git
cd cunzhi-cli/cunzhi-cli

# 以管理员身份运行 PowerShell，然后执行
.\install.ps1
```

### 方法 2: 使用 Cargo 安装

```bash
# 从本地源码安装
cd cunzhi-cli/cunzhi-cli
cargo install --path .

# 从 crates.io 安装（即将支持）
cargo install cunzhi-cli

# 验证安装
cunzhi --version
```

### 方法 3: 手动安装

```bash
# 构建发布版本
cd cunzhi-cli/cunzhi-cli
cargo build --release

# 复制到系统路径
# Linux/macOS:
sudo cp target/release/cunzhi /usr/local/bin/

# Windows:
copy target\release\cunzhi.exe C:\Windows\System32\

# 验证安装
cunzhi --version
```

## 🚀 快速开始

### 1. 初始化项目

使用交互式向导快速配置：

```bash
# 交互式初始化
cunzhi init --name my-project

# 快速初始化（使用默认配置）
cunzhi init --name my-project --yes
```

### 2. 启动 MCP 服务器

```bash
# 启动服务器
cunzhi server start

# 查看服务器状态
cunzhi server status

# 停止服务器
cunzhi server stop
```

### 3. 管理配置

```bash
# 查看当前配置
cunzhi config show

# 编辑配置
cunzhi config edit

# 重置配置
cunzhi config reset
```

## 📖 命令参考

### 全局选项

```
cunzhi [OPTIONS] [COMMAND]

选项:
  -v, --verbose    显示详细输出
  -h, --help       显示帮助信息
  -V, --version    显示版本信息
```

### 初始化命令

```
cunzhi init [OPTIONS]

选项:
  -n, --name <NAME>    项目名称
  -y, --yes            使用默认配置，跳过交互式向导

示例:
  cunzhi init --name my-project          # 交互式初始化
  cunzhi init --name my-project --yes    # 快速初始化
```

### 服务器管理

```
cunzhi server <COMMAND>

命令:
  start     启动 MCP 服务器
  stop      停止 MCP 服务器
  status    查看服务器状态

示例:
  cunzhi server start     # 启动服务器
  cunzhi server status    # 查看状态
  cunzhi server stop      # 停止服务器
```

### 配置管理

```
cunzhi config <COMMAND>

命令:
  show      显示当前配置
  edit      编辑配置文件
  reset     重置为默认配置

示例:
  cunzhi config show      # 查看配置
  cunzhi config edit      # 编辑配置
  cunzhi config reset     # 重置配置
```

### 系统诊断

```
cunzhi doctor

检查系统环境和配置，诊断潜在问题。

示例:
  cunzhi doctor           # 运行系统诊断
```

## ⚙️ 配置说明

### 配置文件位置

- **Windows**: `%APPDATA%\cunzhi\config.json`
- **macOS**: `~/Library/Application Support/cunzhi/config.json`
- **Linux**: `~/.config/cunzhi/config.json`

### 配置文件格式

```json
{
  "version": "0.2.12",
  "reply_config": {
    "enable_continue_reply": true,
    "auto_continue_threshold": 1000,
    "continue_prompt": "请按照最佳实践继续"
  },
  "mcp_config": {
    "tools": {
      "zhi": true,
      "ji": true
    }
  }
}
```

### 配置选项说明

#### 回复配置 (reply_config)

- `enable_continue_reply`: 是否启用自动继续回复
- `auto_continue_threshold`: 自动继续的字符数阈值
- `continue_prompt`: 继续回复时使用的提示词

#### MCP 工具配置 (mcp_config)

- `zhi`: 智能代码审查工具
  - `true`: 启用
  - `false`: 禁用
- `ji`: 记忆管理工具
  - `true`: 启用
  - `false`: 禁用

### 配置模板

项目提供了三种预定义的配置模板：

1. **基础配置**: 最小化配置，只启用核心功能
2. **完整配置**: 启用所有功能，适合完整的代码审查工作流
3. **自定义配置**: 手动选择需要的功能和配置

## 🔧 MCP 工具说明

### zhi - 智能代码审查工具

提供代码质量分析和建议，包括：

- 代码风格检查
- 潜在问题识别
- 最佳实践建议
- 性能优化提示

### ji - 记忆管理工具

管理项目知识和最佳实践，包括：

- 项目规范存储
- 用户偏好记录
- 最佳实践模式
- 项目上下文管理

## 🧪 开发和测试

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --bin test-config
cargo test --bin test-init
cargo test --bin test-ui

# 运行集成测试
cargo test --test integration_tests
```

### 开发模式

```bash
# 以开发模式运行
cargo run -- --help

# 启用详细日志
RUST_LOG=debug cargo run -- server start

# 运行特定命令
cargo run -- init --name test-project
```

## 📚 使用示例

### 完整工作流示例

```bash
# 1. 初始化新项目
cunzhi init --name my-awesome-project

# 2. 启动 MCP 服务器
cunzhi server start

# 3. 查看配置
cunzhi config show

# 4. 运行系统诊断
cunzhi doctor

# 5. 查看服务器状态
cunzhi server status
```

### 自动化脚本示例

```bash
#!/bin/bash
# 自动化项目设置脚本

echo "🚀 开始设置寸止 CLI 项目..."

# 快速初始化
cunzhi init --name "$1" --yes

# 启动服务器
cunzhi server start

# 显示状态
cunzhi server status

echo "✅ 项目设置完成！"
```

## 🐛 故障排除

### 常见问题

1. **配置文件损坏**
   ```bash
   cunzhi config reset
   ```

2. **服务器启动失败**
   ```bash
   cunzhi doctor
   cunzhi server stop
   cunzhi server start
   ```

3. **权限问题**
   - 确保有配置目录的写入权限
   - 在 Windows 上可能需要管理员权限

### 获取帮助

- 运行 `cunzhi --help` 查看命令帮助
- 运行 `cunzhi doctor` 进行系统诊断
- 查看日志文件了解详细错误信息

## 🤝 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [inquire](https://github.com/mikaelmello/inquire) - 交互式命令行界面
- [console](https://github.com/console-rs/console) - 终端控制
- [indicatif](https://github.com/console-rs/indicatif) - 进度条显示
