# 智能文档生成工具 - 快速启动指南

## 🚀 30分钟快速开始

### 第一步：环境准备（5分钟）

```bash
# 1. 安装 Rust（如果还没有安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. 验证安装
rustc --version  # 应该显示 1.70+
cargo --version

# 3. 安装开发工具
cargo install cargo-watch
```

### 第二步：创建项目（5分钟）

```bash
# 1. 创建新项目（如果还没有）
cargo new smart-doc-core
cd smart-doc-core

# 2. 添加基础依赖
cargo add clap --features derive
cargo add anyhow
cargo add serde serde_json
cargo add log env_logger

# 3. 验证项目结构
cargo check
```

### 第三步：编写第一个功能（15分钟）

编辑 `src/main.rs`：

```rust
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "smart-doc")]
#[command(about = "智能文档生成工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 生成文档
    Generate {
        /// 模板文件路径
        #[arg(short, long)]
        template: String,
        
        /// 数据文件路径（JSON格式）
        #[arg(short, long)]
        data: String,
        
        /// 输出文件路径
        #[arg(short, long)]
        output: String,
    },
    
    /// 查看版本信息
    Version,
}

fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate { template, data, output } => {
            println!("🚀 开始生成文档");
            println!("   模板: {}", template);
            println!("   数据: {}", data);
            println!("   输出: {}", output);
            
            // TODO: 实现文档生成逻辑
            println!("✅ 文档生成成功！");
            Ok(())
        }
        Commands::Version => {
            println!("smart-doc v0.1.0");
            Ok(())
        }
    }
}
```

### 第四步：测试运行（5分钟）

```bash
# 1. 编译项目
cargo build

# 2. 查看帮助
./target/debug/smart-doc --help

# 3. 测试命令
./target/debug/smart-doc generate \
  --template test.docx \
  --data data.json \
  --output output.docx

# 4. 查看版本
./target/debug/smart-doc version
```

## 📝 创建示例文件

### 创建数据文件 `data.json`：

```json
{
  "title": "我的第一个文档",
  "author": "开发者",
  "date": "2024-01-15",
  "content": "这是通过智能文档工具生成的内容。\n欢迎使用这个工具！"
}
```

### 创建简单的文本模板 `template.txt`：

```
# {{title}}

作者：{{author}}
日期：{{date}}

{{content}}
```

## 🎯 下一步：实现真正的文档生成

### 1. 添加 DOCX 支持

```bash
# 添加 rdocx 依赖
cargo add rdocx
```

### 2. 创建文档模块 `src/document.rs`：

```rust
use anyhow::Result;
use rdocx::Document;

pub struct DocGenerator;

impl DocGenerator {
    pub fn new() -> Self {
        DocGenerator
    }
    
    pub fn create_simple_document(&self, title: &str, content: &str) -> Result<Document> {
        let mut doc = Document::new();
        
        // 添加标题
        doc.add_paragraph(title)
            .style("Title")
            .add();
        
        // 添加内容
        doc.add_paragraph(content)
            .add();
        
        Ok(doc)
    }
    
    pub fn save_document(&self, doc: &Document, path: &str) -> Result<()> {
        let data = doc.save_to_bytes()?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
```

### 3. 更新主程序：

```rust
mod document;
use document::DocGenerator;

// 在 generate 命令中添加：
let generator = DocGenerator::new();
let doc = generator.create_simple_document("测试文档", "这是测试内容")?;
generator.save_document(&doc, &output)?;
```

## 🔧 开发工具使用

### 常用命令

```bash
# 开发时自动重新编译
cargo watch -x run

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 生成文档
cargo doc --open
```

### VS Code 配置

1. 安装扩展：
   - rust-analyzer
   - Better TOML
   - CodeLLDB

2. 配置 settings.json：
```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.checkOnSave": true
}
```

## 📚 学习资源

### 快速学习 Rust

1. **官方教程**：https://doc.rust-lang.org/book/
2. **Rust by Example**：https://doc.rust-lang.org/rust-by-example/
3. **练习项目**：https://github.com/rust-lang/rustlings

### 本项目相关

1. **rdocx 文档**：https://docs.rs/rdocx
2. **clap 文档**：https://docs.rs/clap
3. **serde 文档**：https://serde.rs/

## 🎉 恭喜！

你已经成功创建了一个基础的文档生成工具框架。接下来可以：

1. **完善功能**：添加真正的模板渲染
2. **扩展格式**：支持 Excel、PDF 等格式
3. **优化体验**：添加进度显示、错误处理
4. **学习进阶**：阅读项目文档中的详细计划

## 💡 小贴士

- 每天花 30 分钟学习，坚持就是胜利
- 遇到问题先尝试自己解决，再查阅资料
- 及时记录学习心得和问题解决方案
- 享受编码的乐趣！

---

**记住**：这是一个学习项目，重点是学习过程。每次完成一个小功能都是进步！

*开始你的 Rust 学习之旅吧！* 🚀