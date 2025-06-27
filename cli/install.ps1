# 寸止 CLI Windows 安装脚本
# 需要以管理员身份运行 PowerShell

param(
    [switch]$Force,
    [string]$InstallPath = "$env:LOCALAPPDATA\cunzhi-cli\bin"
)

# 颜色函数
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

function Write-Info($message) {
    Write-ColorOutput Blue "ℹ️  $message"
}

function Write-Success($message) {
    Write-ColorOutput Green "✅ $message"
}

function Write-Warning($message) {
    Write-ColorOutput Yellow "⚠️  $message"
}

function Write-Error($message) {
    Write-ColorOutput Red "❌ $message"
}

function Write-Header() {
    Write-ColorOutput Blue @"
🚀 寸止 CLI Windows 安装程序
============================
"@
}

# 检查管理员权限
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# 检查系统要求
function Test-Requirements {
    Write-Info "检查系统要求..."
    
    # 检查 Rust
    try {
        $rustVersion = & rustc --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "找到 Rust: $rustVersion"
        } else {
            throw "Rust not found"
        }
    } catch {
        Write-Error "未找到 Rust 编译器"
        Write-Info "请先安装 Rust: https://rustup.rs/"
        exit 1
    }
    
    # 检查 Cargo
    try {
        $cargoVersion = & cargo --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "找到 Cargo: $cargoVersion"
        } else {
            throw "Cargo not found"
        }
    } catch {
        Write-Error "未找到 Cargo 包管理器"
        exit 1
    }
    
    # 检查 Rust 版本
    $versionMatch = $rustVersion -match "rustc (\d+)\.(\d+)"
    if ($versionMatch) {
        $major = [int]$matches[1]
        $minor = [int]$matches[2]
        
        if ($major -lt 1 -or ($major -eq 1 -and $minor -lt 70)) {
            Write-Warning "Rust 版本可能过旧，建议使用 1.70.0 或更高版本"
            Write-Info "运行 'rustup update' 更新 Rust"
        }
    }
}

# 构建项目
function Build-Project {
    Write-Info "构建寸止 CLI..."
    
    if (-not (Test-Path "Cargo.toml")) {
        Write-Error "未找到 Cargo.toml 文件，请确保在正确的目录中运行此脚本"
        exit 1
    }
    
    try {
        & cargo build --release
        if ($LASTEXITCODE -eq 0) {
            Write-Success "构建完成"
        } else {
            throw "Build failed"
        }
    } catch {
        Write-Error "构建失败"
        exit 1
    }
}

# 安装二进制文件
function Install-Binary {
    param([string]$TargetPath)
    
    Write-Info "安装寸止 CLI 到系统..."
    
    $binaryPath = "target\release\cunzhi.exe"
    
    if (-not (Test-Path $binaryPath)) {
        Write-Error "未找到构建的二进制文件: $binaryPath"
        exit 1
    }
    
    # 创建安装目录
    if (-not (Test-Path $TargetPath)) {
        Write-Info "创建安装目录: $TargetPath"
        New-Item -ItemType Directory -Path $TargetPath -Force | Out-Null
    }
    
    # 复制二进制文件
    $targetFile = Join-Path $TargetPath "cunzhi.exe"
    try {
        Copy-Item $binaryPath $targetFile -Force
        Write-Success "已安装到 $targetFile"
    } catch {
        Write-Error "安装失败: $_"
        exit 1
    }
    
    return $TargetPath
}

# 更新 PATH 环境变量
function Update-Path {
    param([string]$NewPath)
    
    Write-Info "更新 PATH 环境变量..."
    
    # 获取当前用户的 PATH
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    # 检查路径是否已存在
    if ($currentPath -split ";" -contains $NewPath) {
        Write-Info "PATH 中已包含 $NewPath"
        return
    }
    
    # 添加新路径
    $newPathValue = if ($currentPath) { "$currentPath;$NewPath" } else { $NewPath }
    
    try {
        [Environment]::SetEnvironmentVariable("PATH", $newPathValue, "User")
        Write-Success "已将 $NewPath 添加到 PATH"
        Write-Warning "请重新启动命令提示符或 PowerShell 以使更改生效"
    } catch {
        Write-Error "更新 PATH 失败: $_"
        Write-Info "请手动将 $NewPath 添加到系统 PATH 环境变量"
    }
}

# 验证安装
function Test-Installation {
    Write-Info "验证安装..."
    
    # 刷新当前会话的环境变量
    $env:PATH = [Environment]::GetEnvironmentVariable("PATH", "User") + ";" + [Environment]::GetEnvironmentVariable("PATH", "Machine")
    
    try {
        $version = & cunzhi --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "安装成功！版本: $version"
            
            Write-Info "可用命令:"
            Write-Output "  cunzhi --help          # 查看帮助"
            Write-Output "  cunzhi init             # 初始化项目"
            Write-Output "  cunzhi server start     # 启动 MCP 服务器"
            Write-Output "  cunzhi config show      # 查看配置"
        } else {
            throw "Command not found"
        }
    } catch {
        Write-Warning "安装验证失败，cunzhi 命令不可用"
        Write-Info "请重新启动命令提示符或 PowerShell，然后重试"
        Write-Info "或者手动检查 PATH 环境变量是否包含安装目录"
    }
}

# 使用 cargo install 的替代方法
function Install-WithCargo {
    Write-Info "使用 cargo install 安装..."
    
    try {
        & cargo install --path .
        if ($LASTEXITCODE -eq 0) {
            Write-Success "通过 cargo install 安装成功"
            return $true
        } else {
            throw "cargo install failed"
        }
    } catch {
        Write-Error "cargo install 失败"
        return $false
    }
}

# 主安装流程
function Main {
    Write-Header
    
    # 检查系统要求
    Test-Requirements
    
    # 询问安装方式
    Write-Output ""
    Write-Info "选择安装方式:"
    Write-Output "1) cargo install (推荐)"
    Write-Output "2) 手动构建和安装到用户目录"
    Write-Output "3) 手动构建和安装到系统目录 (需要管理员权限)"
    Write-Output ""
    
    $choice = Read-Host "请选择 (1, 2 或 3)"
    
    switch ($choice) {
        "1" {
            Write-Info "使用 cargo install 方式..."
            if (Install-WithCargo) {
                Test-Installation
            } else {
                Write-Warning "cargo install 失败，尝试手动安装..."
                Build-Project
                $installDir = Install-Binary $InstallPath
                Update-Path $installDir
                Test-Installation
            }
        }
        "2" {
            Write-Info "使用手动构建方式（用户目录）..."
            Build-Project
            $installDir = Install-Binary $InstallPath
            Update-Path $installDir
            Test-Installation
        }
        "3" {
            if (-not (Test-Administrator)) {
                Write-Error "需要管理员权限才能安装到系统目录"
                Write-Info "请以管理员身份运行 PowerShell"
                exit 1
            }
            Write-Info "使用手动构建方式（系统目录）..."
            Build-Project
            $systemPath = "$env:ProgramFiles\cunzhi-cli\bin"
            $installDir = Install-Binary $systemPath
            # 系统目录通常已在 PATH 中，或需要管理员权限修改
            Write-Info "已安装到系统目录: $installDir"
            Test-Installation
        }
        default {
            Write-Error "无效选择"
            exit 1
        }
    }
    
    Write-Output ""
    Write-Success "🎉 寸止 CLI 安装完成！"
    Write-Info "运行 'cunzhi --help' 开始使用"
}

# 运行主程序
Main
