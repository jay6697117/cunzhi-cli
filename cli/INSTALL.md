# 寸止 CLI 安装指南

本指南将帮助您在不同操作系统上安装和配置寸止 CLI 工具。

## 📋 系统要求

### 最低要求

- **Rust**: 1.70.0 或更高版本
- **内存**: 至少 512MB 可用内存
- **磁盘空间**: 至少 100MB 可用空间
- **网络**: 用于下载依赖（仅安装时需要）

### 支持的操作系统

- ✅ **Windows 10/11** (x64)
- ✅ **macOS 10.15+** (Intel/Apple Silicon)
- ✅ **Linux** (x64, ARM64)
  - Ubuntu 18.04+
  - Debian 10+
  - CentOS 7+
  - Fedora 30+
  - Arch Linux

## 🛠️ 安装方法

### 方法 1: 自动安装脚本（推荐）

这是最简单的安装方法，脚本会自动处理构建、安装和配置。

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

安装脚本提供两种安装方式：
1. **cargo install** - 自动安装到 Cargo 的 bin 目录
2. **手动安装** - 构建后复制到系统目录

### 方法 2: 从源码手动安装

这是最可靠的安装方法，适用于所有支持的平台。

#### 步骤 1: 安装 Rust

如果您还没有安装 Rust，请访问 [rustup.rs](https://rustup.rs/) 并按照说明安装。

**Windows:**
```powershell
# 下载并运行 rustup-init.exe
# 或使用 winget
winget install Rust.Rustup
```

**macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### 步骤 2: 验证 Rust 安装

```bash
rustc --version
cargo --version
```

您应该看到类似以下的输出：
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

#### 步骤 3: 克隆仓库

```bash
git clone https://github.com/your-org/cunzhi-cli.git
cd cunzhi-cli/cunzhi-cli
```

#### 步骤 4: 构建和安装

```bash
# 构建项目
cargo build --release

# 安装到系统
cargo install --path .
```

#### 步骤 5: 验证安装

```bash
cunzhi --version
```

您应该看到版本信息：
```
cunzhi-cli 0.2.12
```

### 方法 2: 使用 Cargo 安装

```bash
# 从 crates.io 安装（即将支持）
cargo install cunzhi-cli

# 验证安装
cunzhi --version
```

### 方法 3: 下载预编译二进制文件

访问 [Releases 页面](https://github.com/your-org/cunzhi-cli/releases) 下载适合您系统的预编译二进制文件。

**Windows:**
1. 下载 `cunzhi-cli-windows-x64.zip`
2. 解压到您选择的目录
3. 将目录添加到 PATH 环境变量

**macOS:**
```bash
# 下载并解压
curl -L https://github.com/your-org/cunzhi-cli/releases/latest/download/cunzhi-cli-macos.tar.gz | tar xz

# 移动到系统路径
sudo mv cunzhi /usr/local/bin/

# 验证安装
cunzhi --version
```

**Linux:**
```bash
# 下载并解压
curl -L https://github.com/your-org/cunzhi-cli/releases/latest/download/cunzhi-cli-linux-x64.tar.gz | tar xz

# 移动到系统路径
sudo mv cunzhi /usr/local/bin/

# 验证安装
cunzhi --version
```

## ⚙️ 初始配置

### 1. 运行初始化向导

安装完成后，运行初始化向导来配置您的环境：

```bash
cunzhi init --name my-first-project
```

这将启动交互式向导，帮助您：
- 选择配置模板
- 配置 MCP 工具
- 设置回复选项

### 2. 验证配置

```bash
# 查看配置
cunzhi config show

# 启动服务器测试
cunzhi server start
cunzhi server status
cunzhi server stop
```

### 3. 运行系统诊断

```bash
cunzhi doctor
```

这将检查您的系统环境并报告任何潜在问题。

## 🔧 高级安装选项

### 开发版本安装

如果您想使用最新的开发版本：

```bash
# 克隆开发分支
git clone -b develop https://github.com/your-org/cunzhi-cli.git
cd cunzhi-cli/cunzhi-cli

# 安装开发版本
cargo install --path . --force
```

### 自定义安装位置

```bash
# 安装到自定义目录
cargo install --path . --root /custom/path

# 确保将 /custom/path/bin 添加到 PATH
export PATH="/custom/path/bin:$PATH"
```

### 仅安装特定功能

```bash
# 安装时禁用某些功能
cargo install --path . --no-default-features --features "core,mcp"
```

## 🐛 安装故障排除

### 常见问题和解决方案

#### 1. Rust 版本过旧

**错误信息:**
```
error: package `cunzhi-cli v0.2.12` cannot be built because it requires rustc 1.70.0 or newer
```

**解决方案:**
```bash
rustup update
```

#### 2. 编译错误

**错误信息:**
```
error: linking with `cc` failed: exit status: 1
```

**解决方案:**

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

**CentOS/RHEL:**
```bash
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel
```

**macOS:**
```bash
xcode-select --install
```

#### 3. 权限问题

**错误信息:**
```
error: failed to create directory `/usr/local/bin`
```

**解决方案:**
```bash
# 使用 sudo 安装
sudo cargo install --path .

# 或安装到用户目录
cargo install --path . --root ~/.local
export PATH="$HOME/.local/bin:$PATH"
```

#### 4. 网络连接问题

如果在中国大陆遇到网络问题，可以使用镜像源：

```bash
# 设置 Cargo 镜像
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << EOF
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
EOF
```

#### 5. 磁盘空间不足

**解决方案:**
```bash
# 清理 Cargo 缓存
cargo clean

# 清理全局缓存
rm -rf ~/.cargo/registry/cache
```

## 📦 包管理器安装

### Homebrew (macOS)

```bash
# 添加 tap（即将支持）
brew tap your-org/cunzhi-cli
brew install cunzhi-cli
```

### Chocolatey (Windows)

```powershell
# 安装（即将支持）
choco install cunzhi-cli
```

### Snap (Linux)

```bash
# 安装（即将支持）
sudo snap install cunzhi-cli
```

## 🔄 更新和卸载

### 更新

```bash
# 如果从源码安装
git pull
cargo install --path . --force

# 如果使用 Cargo 安装
cargo install cunzhi-cli --force
```

### 卸载

#### 自动卸载（推荐）

**Linux/macOS:**
```bash
cd cunzhi-cli/cunzhi-cli
./uninstall.sh
```

**Windows:**
```powershell
cd cunzhi-cli/cunzhi-cli
.\uninstall.ps1  # (即将提供)
```

#### 手动卸载

```bash
# 卸载二进制文件
cargo uninstall cunzhi-cli

# 或手动删除
sudo rm /usr/local/bin/cunzhi  # Linux/macOS
# del C:\Windows\System32\cunzhi.exe  # Windows

# 删除配置文件（可选）
# Windows: rmdir /s "%APPDATA%\cunzhi"
# macOS: rm -rf ~/Library/Application\ Support/cunzhi
# Linux: rm -rf ~/.config/cunzhi
```

## 🆘 获取帮助

如果您在安装过程中遇到问题：

1. **查看文档**: 阅读 [README.md](README.md) 和本安装指南
2. **运行诊断**: `cunzhi doctor`
3. **查看日志**: 检查错误消息和日志文件
4. **搜索问题**: 在 GitHub Issues 中搜索类似问题
5. **报告问题**: 如果问题仍然存在，请创建新的 Issue

## ✅ 安装验证清单

安装完成后，请验证以下功能：

- [ ] `cunzhi --version` 显示正确版本
- [ ] `cunzhi --help` 显示帮助信息
- [ ] `cunzhi init --name test --yes` 成功创建配置
- [ ] `cunzhi config show` 显示配置信息
- [ ] `cunzhi server start` 成功启动服务器
- [ ] `cunzhi server status` 显示服务器状态
- [ ] `cunzhi server stop` 成功停止服务器
- [ ] `cunzhi doctor` 通过所有检查

如果所有项目都通过，恭喜您！寸止 CLI 已成功安装并可以使用。
