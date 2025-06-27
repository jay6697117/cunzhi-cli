# 贡献指南

感谢您对寸止 CLI 项目的关注！我们欢迎各种形式的贡献，包括但不限于：

- 🐛 报告 Bug
- 💡 提出新功能建议
- 📝 改进文档
- 🔧 提交代码修复
- ✨ 添加新功能
- 🧪 编写测试

## 📋 开始之前

在开始贡献之前，请确保您已经：

1. 阅读了 [README.md](README.md) 和 [INSTALL.md](INSTALL.md)
2. 了解项目的基本架构和目标
3. 检查了现有的 Issues 和 Pull Requests，避免重复工作

## 🛠️ 开发环境设置

### 1. 克隆仓库

```bash
git clone https://github.com/your-org/cunzhi-cli.git
cd cunzhi-cli/cunzhi-cli
```

### 2. 安装依赖

确保您已安装 Rust 1.70.0 或更高版本：

```bash
rustc --version
cargo --version
```

### 3. 构建项目

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release
```

### 4. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --test integration_tests
cargo test --test unit_tests

# 运行测试程序
cargo run --bin test-config
cargo run --bin test-init
cargo run --bin test-ui
```

### 5. 代码格式化和检查

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 检查文档
cargo doc --no-deps --open
```

## 📝 代码规范

### Rust 代码风格

我们遵循标准的 Rust 代码风格：

- 使用 `cargo fmt` 格式化代码
- 遵循 `cargo clippy` 的建议
- 使用有意义的变量和函数名
- 添加适当的文档注释

### 提交信息格式

使用清晰、描述性的提交信息：

```
类型(范围): 简短描述

详细描述（可选）

关闭 #issue_number（如果适用）
```

类型包括：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

示例：
```
feat(cli): 添加项目初始化向导

实现了类似 create-vue 的交互式初始化体验，
包括模板选择、配置设置和进度显示。

关闭 #123
```

## 🐛 报告 Bug

在报告 Bug 时，请提供以下信息：

### Bug 报告模板

```markdown
## Bug 描述
简要描述遇到的问题。

## 重现步骤
1. 执行命令 `cunzhi ...`
2. 输入 `...`
3. 看到错误 `...`

## 预期行为
描述您期望发生的情况。

## 实际行为
描述实际发生的情况。

## 环境信息
- 操作系统: [例如 macOS 13.0]
- Rust 版本: [例如 1.75.0]
- 寸止 CLI 版本: [例如 0.2.12]

## 附加信息
- 错误日志
- 配置文件内容
- 截图（如果适用）
```

## 💡 功能建议

在提出新功能建议时，请：

1. 检查是否已有类似的建议
2. 清楚地描述功能的用途和价值
3. 提供具体的使用场景
4. 考虑实现的复杂性和维护成本

### 功能建议模板

```markdown
## 功能描述
简要描述建议的功能。

## 使用场景
描述什么情况下会使用这个功能。

## 详细设计
详细描述功能的工作方式。

## 替代方案
是否考虑过其他实现方式？

## 附加信息
任何其他相关信息。
```

## 🔧 代码贡献流程

### 1. Fork 仓库

点击 GitHub 页面右上角的 "Fork" 按钮。

### 2. 创建功能分支

```bash
git checkout -b feature/your-feature-name
```

分支命名规范：
- `feature/功能名` - 新功能
- `fix/问题描述` - Bug 修复
- `docs/文档类型` - 文档更新
- `refactor/重构内容` - 代码重构

### 3. 开发和测试

- 编写代码
- 添加或更新测试
- 确保所有测试通过
- 运行代码检查工具

### 4. 提交更改

```bash
git add .
git commit -m "feat(cli): 添加新功能"
git push origin feature/your-feature-name
```

### 5. 创建 Pull Request

1. 访问您的 Fork 仓库
2. 点击 "New Pull Request"
3. 填写 PR 描述
4. 等待代码审查

### Pull Request 模板

```markdown
## 更改描述
简要描述此 PR 的更改内容。

## 更改类型
- [ ] Bug 修复
- [ ] 新功能
- [ ] 文档更新
- [ ] 代码重构
- [ ] 性能优化
- [ ] 其他

## 测试
- [ ] 添加了新的测试
- [ ] 更新了现有测试
- [ ] 所有测试通过
- [ ] 手动测试通过

## 检查清单
- [ ] 代码遵循项目规范
- [ ] 自我审查了代码
- [ ] 添加了必要的注释
- [ ] 更新了相关文档
- [ ] 没有引入新的警告

## 关联 Issue
关闭 #issue_number
```

## 🧪 测试指南

### 测试类型

1. **单元测试** (`tests/unit_tests.rs`)
   - 测试单个函数或模块
   - 快速执行
   - 高覆盖率

2. **集成测试** (`tests/integration_tests.rs`)
   - 测试完整的命令流程
   - 模拟真实使用场景
   - 端到端验证

3. **测试程序** (`src/bin/test_*.rs`)
   - 手动验证特定功能
   - 用户体验测试
   - 性能测试

### 编写测试

```rust
#[test]
fn test_function_name() {
    // 准备测试数据
    let input = "test input";
    
    // 执行被测试的功能
    let result = function_to_test(input);
    
    // 验证结果
    assert_eq!(result, expected_output);
    assert!(result.is_ok());
}
```

### 测试最佳实践

- 测试名称应该清楚地描述测试内容
- 每个测试应该只测试一个功能点
- 使用有意义的断言消息
- 测试边界条件和错误情况
- 保持测试的独立性

## 📚 文档贡献

### 文档类型

- **README.md** - 项目概述和快速开始
- **INSTALL.md** - 详细安装指南
- **CONTRIBUTING.md** - 贡献指南（本文件）
- **代码注释** - 函数和模块文档
- **示例代码** - 使用示例

### 文档规范

- 使用清晰、简洁的语言
- 提供具体的示例
- 保持文档与代码同步
- 使用标准的 Markdown 格式

## 🔍 代码审查

所有的 Pull Request 都需要经过代码审查。审查重点包括：

- **功能正确性** - 代码是否按预期工作
- **代码质量** - 是否遵循最佳实践
- **测试覆盖** - 是否有足够的测试
- **文档完整性** - 是否更新了相关文档
- **性能影响** - 是否影响性能
- **安全性** - 是否引入安全风险

## 🎯 发布流程

项目维护者负责发布新版本：

1. 更新版本号
2. 更新 CHANGELOG.md
3. 创建 Git 标签
4. 发布到 crates.io
5. 创建 GitHub Release

## 📞 获取帮助

如果您在贡献过程中遇到问题：

1. 查看现有的 Issues 和文档
2. 在 GitHub 上创建新的 Issue
3. 参与社区讨论

## 🙏 致谢

感谢所有为寸止 CLI 项目做出贡献的开发者！

您的贡献让这个项目变得更好。
