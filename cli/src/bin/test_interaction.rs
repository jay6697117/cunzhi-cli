// 测试新的交互功能
use cunzhi_cli::mcp::tools::interaction::InteractionTool;
use cunzhi_cli::mcp::ZhiRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试增强的CLI交互功能");

    // 测试1: 模拟您截图中的Python项目分析场景
    println!("\n=== 测试1: Python学习项目分析报告 ===");
    let request_with_options = ZhiRequest {
        message: r#"# Python学习项目分析报告

## 项目概述
这是一个Python学习演示项目，位于 `/Users/zhangjinhui/Desktop/py-demo`，主要包含Python基础教学代码和一个OAuth 2.0 PKCE授权实现。

## 项目结构分析
### 📁 主要文件分类
- **01. Python基础教学文件**
  - `01.第一个Python程序.py` - 入门级Hello World示例
  - `02.第一个Python程序.py` - 基础语法演示
  - `03.Python语言中的变量.py` - 变量概念使用
  - `04.Python语言中的运算符.py` - 运算符使用
  - `05.分支结构.py` - 条件判断（三角形计算示例）
  - `06.循环结构.py` - 循环控制
  - `07.分支循环结构实战.py` - 综合练习

- **08-11系列** - 数据结构（列表、元组、字符串）
- **实用工具文件**
  - `augment_code.py` - OAuth 2.0 PKCE授权流程实现
  - `test1/1.py` - 全文件
  - `test2/2.py` - 全文件

### 🎯 代码质量评估
**优点：**
- ✅ **教学结构清晰** - 按知识点逐步式结构
- ✅ **注释详细** - 每个概念都有体现例子
- ✅ **渐进式计算** - 色彩统计等实际应用案例
- ✅ **有序计算** - 从基础语法到数据结构的工程水平

### ⚠️ 需要改进的地方
- **代码组织** - 大量注释代码未清理
- **安全标准** - 实现了PKCE安全扩展
- **代码规范** - 良好的函数设计和注释
- **实践导向** - 针对VSCode开发环境优化
- **文档完整** - 缺少单元测试

### 🔧 长期规划
1. 重构目录结构（如：tutorials/, utils/, tests/）
2. 添加配置文件管理
3. 集成CI/CD流程
4. 添加代码质量检查工具

## 总体评价
这是一个 **教学价值很高** 的Python项目，既包含了系统的基础教学内容，又有实际的工程应用代码。项目展现了从入门到实践的完整学习路径，特别适合Python初学者使用。

**推荐指数：⭐⭐⭐⭐☆**"#.to_string(),
        predefined_options: vec![
            "深入分析定制模块".to_string(),
            "优化代码结构".to_string(),
            "添加新功能".to_string(),
            "创建项目文档".to_string(),
        ],
        is_markdown: true,
        terminal_mode: Some(false),
    };

    match InteractionTool::zhi(request_with_options).await {
        Ok(result) => {
            println!("✅ 测试1成功: {:?}", result);
        }
        Err(e) => {
            println!("❌ 测试1失败: {}", e);
        }
    }

    // 测试2: 无预定义选项的自由输入
    println!("\n=== 测试2: 自由文本输入 ===");
    let request_free_input = ZhiRequest {
        message: r#"# 🎯 自由输入测试

请输入任何您想要的内容："#.to_string(),
        predefined_options: vec![],
        is_markdown: true,
        terminal_mode: Some(false),
    };

    match InteractionTool::zhi(request_free_input).await {
        Ok(result) => {
            println!("✅ 测试2成功: {:?}", result);
        }
        Err(e) => {
            println!("❌ 测试2失败: {}", e);
        }
    }

    println!("\n🎉 交互功能测试完成！");
    Ok(())
}
