# 智能文档生成工具 - 快速开始指南

## 项目概述

智能文档生成工具是一个基于 Rust 开发的高性能、智能化文档生成系统。它支持多种文档格式（DOCX、XLSX、PDF、HTML），具备模板引擎、AI辅助内容生成和高并发服务化能力。

## 环境要求

### 1. 系统要求
- **操作系统**：Linux、macOS 或 Windows（WSL2 推荐）
- **内存**：至少 8GB RAM
- **磁盘空间**：至少 2GB 可用空间

### 2. 开发工具
- **Rust 工具链**：1.70+ 版本
- **Git**：版本控制
- **Docker**（可选）：容器化部署
- **VS Code**（推荐）：代码编辑器

## 快速安装

### 1. 安装 Rust
```bash
# 使用 rustup 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 配置环境变量
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

### 2. 克隆项目
```bash
git clone https://github.com/your-org/smart-doc-core.git
cd smart-doc-core
```

### 3. 安装依赖
```bash
# 安装开发工具
cargo install cargo-watch  # 文件变化监控
cargo install cargo-tarpaulin  # 代码覆盖率
cargo install flamegraph  # 性能分析

# 构建项目
cargo build
```

## 项目结构

```
smart-doc-core/
├── Cargo.toml              # 工作区配置
├── README.md               # 项目说明
├── .github/workflows/      # CI/CD 配置
├── crates/                 # 核心模块
│   ├── core/              # 核心库
│   ├── ai/                # AI 模块
│   ├── server/            # 服务层
│   ├── cli/               # 命令行工具
│   └── wasm/              # WebAssembly（可选）
├── tests/                  # 集成测试
├── benches/                # 基准测试
├── examples/               # 使用示例
└── docs/                   # 文档
    ├── plan.md            # 开发方案
    ├── detailed_development_plan.md  # 详细开发计划
    ├── task_timeline.md   # 任务时间线
    ├── technical_implementation.md  # 技术实现
    └── getting_started.md # 本指南
```

## 快速开始示例

### 1. 使用 CLI 工具生成文档

```bash
# 进入项目目录
cd smart-doc-core

# 构建 CLI 工具
cargo build --release --bin smart-doc

# 查看帮助
./target/release/smart-doc --help

# 生成示例文档
./target/release/smart-doc generate \
  --template examples/templates/report_template.docx \
  --data examples/data/sample_report.json \
  --output my_report.docx \
  --format docx
```

### 2. 创建简单的模板

创建模板文件 `my_template.docx`，在文档中添加占位符：
```
# {{title}}

作者：{{author}}
日期：{{date}}

## 概述
{{overview}}

## 详细内容
{{content}}
```

### 3. 准备数据文件

创建数据文件 `data.json`：
```json
{
  "title": "项目进度报告",
  "author": "张三",
  "date": "2024-01-15",
  "overview": "本项目正在按计划进行，已完成基础架构搭建。",
  "content": "详细开发进度如下：\n1. 已完成核心模块设计\n2. 正在实现模板引擎\n3. 下一步计划集成AI功能"
}
```

### 4. 运行生成命令

```bash
./target/release/smart-doc generate \
  --template my_template.docx \
  --data data.json \
  --output output.docx
```

## 开发工作流

### 1. 开发环境设置

```bash
# 安装开发依赖
cargo install cargo-watch

# 启动开发服务器（监听文件变化）
cargo watch -x 'run --bin smart-doc-server'

# 运行测试
cargo test

# 运行特定测试
cargo test test_template_rendering

# 检查代码质量
cargo clippy
cargo fmt --check
```

### 2. 代码结构

```rust
// 主要模块说明
crates/core/src/
├── lib.rs              # 核心库入口
├── document.rs         # 文档结构定义
├── element.rs          # 文档元素定义
├── style.rs            # 样式系统
├── template/           # 模板引擎
│   ├── mod.rs
│   ├── engine.rs
│   └── extensions.rs
├── generator/          # 文档生成器
│   ├── mod.rs
│   ├── docx_generator.rs
│   ├── excel_generator.rs
│   └── pdf_generator.rs
└── utils/              # 工具函数
```

### 3. 添加新功能

#### 3.1 添加新的文档元素类型

```rust
// 在 crates/core/src/element.rs 中添加
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocElement {
    // ... 现有元素
    
    /// 图表元素
    Chart {
        id: String,
        chart_type: ChartType,
        data: Vec<ChartData>,
        options: ChartOptions,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Scatter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}
```

#### 3.2 实现新的生成器

```rust
// 创建 crates/core/src/generator/html_generator.rs
use crate::document::Document;
use crate::error::Result;

pub struct HtmlGenerator;

impl HtmlGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, document: &Document) -> Result<String> {
        let mut html = String::new();
        
        // 生成 HTML 内容
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<title>Generated Document</title>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        
        // 添加文档内容
        for element in &document.elements {
            html.push_str(&self.render_element(element)?);
        }
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        Ok(html)
    }
    
    fn render_element(&self, element: &DocElement) -> Result<String> {
        match element {
            DocElement::Paragraph { text, .. } => {
                Ok(format!("<p>{}</p>\n", html_escape::escape_text(text)))
            }
            DocElement::Table { rows, .. } => {
                let mut table_html = String::from("<table>\n");
                for row in rows {
                    table_html.push_str("<tr>\n");
                    for cell in row {
                        table_html.push_str(&format!("<td>{}</td>\n", cell.content));
                    }
                    table_html.push_str("</tr>\n");
                }
                table_html.push_str("</table>\n");
                Ok(table_html)
            }
            // ... 处理其他元素类型
            _ => Ok(String::new()),
        }
    }
}
```

## 配置说明

### 1. 配置文件

创建 `config.yaml`：

```yaml
# 应用配置
app:
  name: "smart-doc-generator"
  version: "0.1.0"
  log_level: "info"

# 模板配置
templates:
  directory: "./templates"
  default_template: "default.docx"
  cache_enabled: true
  cache_ttl: 3600  # 秒

# 输出配置
output:
  default_format: "docx"
  compression: true
  quality: "high"

# AI 配置（可选）
ai:
  enabled: false
  provider: "openai"
  api_key: "${OPENAI_API_KEY}"
  model: "gpt-3.5-turbo"
  temperature: 0.7

# 服务配置
server:
  host: "0.0.0.0"
  port: 8080
  workers: 4
  max_concurrent_requests: 100
  request_timeout: 30  # 秒
```

### 2. 环境变量

```bash
# 设置环境变量
export SMART_DOC_LOG_LEVEL=debug
export SMART_DOC_TEMPLATE_DIR=./my_templates
export OPENAI_API_KEY=your-api-key-here

# 或者使用 .env 文件
echo "SMART_DOC_LOG_LEVEL=debug" >> .env
echo "SMART_DOC_TEMPLATE_DIR=./my_templates" >> .env
```

## 测试指南

### 1. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test --package core

# 运行集成测试
cargo test --test integration

# 运行性能测试
cargo bench

# 生成测试覆盖率报告
cargo tarpaulin --ignore-tests --out Html
```

### 2. 编写测试

```rust
// 示例测试代码
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_document_creation() {
        let mut doc = Document::new();
        doc.add_paragraph("Hello, World!");
        
        assert_eq!(doc.elements.len(), 1);
        
        if let DocElement::Paragraph { text, .. } = &doc.elements[0] {
            assert_eq!(text, "Hello, World!");
        } else {
            panic!("Expected paragraph element");
        }
    }
    
    #[test]
    fn test_template_rendering() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let template_path = temp_dir.path().join("template.docx");
        
        // 创建测试模板
        create_test_template(&template_path)?;
        
        let mut engine = TemplateEngine::new(temp_dir.path().to_path_buf())?;
        let data = json!({
            "title": "Test Document",
            "content": "Test content"
        });
        
        let result = engine.render("template", &data)?;
        assert!(result.contains("Test Document"));
        assert!(result.contains("Test content"));
        
        Ok(())
    }
}
```

## 故障排除

### 常见问题

#### 1. Rust 编译错误
```bash
# 清理并重新构建
cargo clean
cargo build

# 更新依赖
cargo update

# 检查 Rust 版本
rustup update
```

#### 2. 模板渲染问题
```bash
# 启用调试日志
export RUST_LOG=debug

# 检查模板语法
cargo run --bin smart-doc -- check-template template.docx

# 验证数据格式
cargo run --bin smart-doc -- validate-data data.json
```

#### 3. 性能问题
```bash
# 生成性能分析报告
cargo flamegraph --bin smart-doc -- generate --template large_template.docx

# 内存使用分析
valgrind --tool=massif target/debug/smart-doc generate --template test.docx
```

### 调试技巧

```rust
// 添加调试日志
use tracing::{debug, info, error};

pub fn generate_document(&self) -> Result<()> {
    debug!("Starting document generation");
    
    // ... 生成逻辑
    
    info!("Document generated successfully");
    Ok(())
}

// 使用环境变量控制日志级别
// RUST_LOG=debug cargo run --bin smart-doc
```

## 下一步

### 学习资源
1. **Rust 官方文档**：https://doc.rust-lang.org/
2. **项目文档**：查看 `docs/` 目录
3. **示例代码**：查看 `examples/` 目录
4. **API 文档**：运行 `cargo doc --open`

### 贡献指南
1. 阅读 `CONTRIBUTING.md`
2. 创建功能分支
3. 编写测试用例
4. 提交 Pull Request

### 获取帮助
- 查看项目 Issues
- 加入社区讨论
- 阅读技术博客

---

*本指南将定期更新，最新版本请查看项目文档*