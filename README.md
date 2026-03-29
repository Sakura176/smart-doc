# Smart Doc Core - 智能文档生成工具

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![Status](https://img.shields.io/badge/Status-开发中-yellow)

一个基于 Rust 开发的智能文档生成工具，支持多种文档格式（DOCX、XLSX、PDF、HTML），具备模板引擎和 AI 辅助功能。

## 📋 项目简介

这是一个个人学习项目，旨在通过实践掌握 Rust 编程语言，同时开发一个实用的文档生成工具。

### 主要特性
- ✅ **多格式支持**：DOCX、XLSX、PDF、HTML
- ✅ **模板引擎**：支持变量替换、循环、条件判断
- ✅ **高性能**：基于 Rust 的高性能实现
- ✅ **易于使用**：提供 CLI 和 API 两种使用方式
- 🔄 **AI 集成**：计划集成 AI 辅助内容生成（开发中）

### 学习目标
- 掌握 Rust 编程语言
- 学习现代文档处理技术
- 实践微服务和 AI 集成
- 建立完整的项目开发经验

## 🚀 快速开始

### 环境要求
- Rust 1.70+ ([安装指南](https://www.rust-lang.org/tools/install))
- Git

### 安装和运行
```bash
# 克隆项目
git clone https://github.com/Sakura176/smart-doc.git
cd smart-doc

# 构建项目
cargo build --release

# 运行 CLI 工具
./target/release/smart-doc --help
```

### 简单示例
```bash
# 生成文档
./target/release/smart-doc generate \
  --template examples/template.docx \
  --data examples/data.json \
  --output result.docx
```

## 📁 项目结构

```
smart-doc-core/
├── src/                    # 源代码
│   ├── lib.rs             # 核心库
│   ├── cli/               # 命令行接口
│   ├── core/              # 核心功能
│   ├── template/          # 模板引擎
│   └── generator/         # 文档生成器
├── docs/                  # 项目文档
│   ├── personal_project_plan.md  # 个人开发计划
│   ├── weekly_template.md        # 每周总结模板
│   └── technical_implementation.md # 技术实现
├── learning_notes/        # 学习笔记
├── examples/              # 使用示例
├── tests/                 # 测试文件
├── Cargo.toml            # 项目配置
└── README.md             # 本文件
```

## 📚 学习计划

### 12周学习路线
1. **第1-4周**：Rust基础与核心功能
   - 环境搭建、数据结构设计、DOCX生成、模板引擎
2. **第5-8周**：功能扩展与工程化
   - 多格式支持、配置管理、错误处理、测试文档
3. **第9-12周**：高级功能与部署
   - Web服务、AI集成、容器化部署、项目优化

### 每日学习时间
- **工作日**：1-2小时/天
- **周末**：3-4小时/天

## 🛠️ 开发指南

### 环境设置
```bash
# 安装开发工具
rustup component add rust-analyzer clippy rustfmt
cargo install cargo-watch cargo-tarpaulin

# 配置 Git
git config --local user.name "Your Name"
git config --local user.email "your.email@example.com"
```

### 开发工作流
```bash
# 1. 检查代码
cargo check

# 2. 运行测试
cargo test

# 3. 代码格式化
cargo fmt

# 4. 代码检查
cargo clippy

# 5. 构建发布版本
cargo build --release
```

### 添加新功能
1. 创建功能分支：`git checkout -b feature/new-feature`
2. 实现功能并编写测试
3. 提交代码：`git commit -m "feat: add new feature"`
4. 推送到远程：`git push origin feature/new-feature`
5. 创建 Pull Request

## 📊 进度跟踪

### 当前状态
- [x] 项目初始化
- [x] Rust 基础学习
- [ ] 核心数据结构设计
- [ ] DOCX 文档生成
- [ ] 模板引擎集成
- [ ] 多格式支持
- [ ] Web API 服务
- [ ] AI 集成

### 学习笔记
每周的学习总结记录在 `learning_notes/` 目录中，使用 `docs/weekly_template.md` 作为模板。

## 🤝 贡献指南

虽然这是个人项目，但欢迎建议和反馈：

1. Fork 本仓库
2. 创建功能分支
3. 提交更改
4. 创建 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Rust 社区](https://www.rust-lang.org/) - 优秀的编程语言和工具链
- [rdocx](https://crates.io/crates/rdocx) - DOCX 处理库
- [tera](https://crates.io/crates/tera) - 模板引擎
- 所有 Rust 开源项目的贡献者

## 📞 联系

- GitHub: [@your-username](https://github.com/your-username)
- 博客: [个人技术博客链接]
- Email: your.email@example.com

---

**提示**：这是一个学习项目，重点是学习过程而不是最终结果。享受学习 Rust 的乐趣！

*开始你的 Rust 学习之旅吧！🚀*
