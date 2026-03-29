# 智能文档生成工具 - 示例文件说明

## 📁 示例文件目录

本目录包含智能文档生成工具的示例文件，用于演示工具的使用方法和功能。

## 📋 文件说明

### 1. 基础示例文件

#### `basic_usage.rs` - Rust代码示例
- **作用**: 展示如何使用`smart-doc-core`库的API
- **内容**: 包含4个示例函数，演示文档创建、构建器使用、样式设置和CLI功能
- **运行方式**: `cargo run --example basic_usage`

#### `config.yaml` - 配置文件示例
- **作用**: 展示完整的配置选项
- **内容**: 包含应用配置、模板配置、输出配置、AI集成、服务配置等
- **使用方式**: 可作为项目配置模板，根据需求修改使用

### 2. 模板和数据文件

#### `simple_template.txt` - 复杂模板示例
- **格式**: Tera模板语法（Jinja2风格）
- **特点**: 包含变量替换、循环、条件判断等高级功能
- **适用场景**: 需要生成结构化文档时使用
- **模板变量**:
  - `{{title}}` - 文档标题
  - `{{author}}` - 作者
  - `{{date}}` - 日期
  - `{{content}}` - 主要内容
  - `{{features}}` - 功能列表（数组）
  - `{{technologies}}` - 技术栈列表（对象数组）
  - `{{modules}}` - 模块状态表格数据
  - `{{next_tasks}}` - 下一步任务列表

#### `sample_data.json` - 复杂数据示例
- **格式**: JSON
- **特点**: 包含嵌套结构和数组数据
- **对应模板**: `simple_template.txt`
- **数据结构**: 包含项目文档所需的所有字段

#### `basic_template.txt` - 简单模板示例
- **格式**: 简单文本模板
- **特点**: 只包含基本变量替换
- **适用场景**: 快速测试和简单文档生成
- **模板变量**:
  - `{{title}}` - 文档标题
  - `{{author}}` - 作者
  - `{{date}}` - 日期
  - `{{content}}` - 内容
  - `{{current_time}}` - 当前时间
  - `{{version}}` - 版本号

#### `basic_data.json` - 简单数据示例
- **格式**: JSON
- **特点**: 只包含基本字段
- **对应模板**: `basic_template.txt`
- **数据结构**: 简单的键值对

## 🚀 使用示例

### 1. 运行Rust代码示例

```bash
# 进入项目根目录
cd smart-doc-core

# 运行基础使用示例
cargo run --example basic_usage
```

### 2. 使用CLI工具生成文档

```bash
# 生成简单文档（文本格式）
cargo run -- generate \
  --template examples/basic_template.txt \
  --data examples/basic_data.json \
  --output output/simple_doc.txt \
  --format txt

# 生成复杂文档（DOCX格式）
cargo run -- generate \
  --template examples/simple_template.txt \
  --data examples/sample_data.json \
  --output output/project_doc.docx \
  --format docx
```

### 3. 查看模板信息

```bash
# 查看模板信息
cargo run -- info --template examples/basic_template.txt

# 验证模板语法
cargo run -- validate --template examples/simple_template.txt
```

### 4. 其他CLI命令

```bash
# 列出可用模板（需要实现模板发现功能）
cargo run -- list

# 查看版本信息
cargo run -- version
```

## 📝 模板语法说明

### 变量替换
```
{{variable_name}}
```
- 从数据文件中读取对应的值进行替换
- 支持嵌套访问：`{{user.name}}`

### 循环语句
```
{% for item in items %}
  {{item}}
{% endfor %}
```
- 遍历数组或列表
- 支持循环索引：`{{loop.index}}`

### 条件判断
```
{% if condition %}
  内容
{% elif other_condition %}
  其他内容
{% else %}
  默认内容
{% endif %}
```

### 注释
```
{# 这是注释，不会出现在输出中 #}
```

### 过滤器
```
{{variable | upper}}      {# 转换为大写 #}
{{variable | lower}}      {# 转换为小写 #}
{{variable | length}}     {# 获取长度 #}
{{variable | default("默认值")}}  {# 设置默认值 #}
```

## 🔧 数据文件格式

### 基本格式
```json
{
  "title": "文档标题",
  "author": "作者名称",
  "content": "文档内容",
  "date": "2024-03-29"
}
```

### 数组格式
```json
{
  "features": ["功能1", "功能2", "功能3"]
}
```

### 嵌套对象
```json
{
  "project": {
    "name": "项目名称",
    "status": "进行中",
    "members": ["成员1", "成员2"]
  }
}
```

## 🎯 示例输出

### 简单文档输出示例
```
# 简单文档示例

作者: 测试用户
日期: 2024-03-29

这是一个简单的文档生成示例。通过这个示例，可以了解如何使用智能文档生成工具创建文档。

文档生成工具支持多种格式，包括DOCX、PDF、Excel等。用户可以通过命令行界面或API调用来生成文档。

---

这是一个简单的文档模板示例。
当前时间: 2024-03-29 10:45:00
文档版本: 1.0.0
```

### 复杂文档输出示例
```
# 智能文档生成工具项目文档

## 文档信息
- **作者**: Rust学习者
- **日期**: 2024-03-29
- **版本**: 0.1.0

## 内容摘要
这是一个基于Rust开发的智能文档生成工具项目文档。该项目旨在通过实践学习Rust编程，同时开发一个实用的文档生成工具。

## 详细内容

### 项目概述
智能文档生成工具是一个个人学习项目，旨在通过开发一个支持多种格式的文档生成工具来掌握Rust编程。项目计划在12周内完成，从基础语法学习到完整工具开发。

### 主要功能
- 支持DOCX文档生成
- 支持Excel表格生成
- 支持PDF文档生成
- 支持HTML文档生成
- 模板引擎支持
- 命令行界面
- 配置管理
- 错误处理系统

### 技术栈
- **Rust**: 系统编程语言，提供内存安全和高性能
- **rdocx**: DOCX文档处理库
- **tera**: 模板引擎，支持Jinja2语法
- **clap**: 命令行参数解析库
- **serde**: 序列化/反序列化框架

### 项目状态
| 模块 | 状态 | 进度 |
|------|------|------|
| 核心引擎 | 开发中 | 70% |
| 文档数据结构 | 已完成 | 100% |
| 错误处理 | 已完成 | 100% |
| 命令行接口 | 开发中 | 80% |
| 模板系统 | 待开始 | 0% |
| 多格式支持 | 待开始 | 0% |

### 下一步计划
1. 修复编译错误，确保项目可以正常构建
2. 实现基本的DOCX文档生成功能
3. 完善模板引擎集成
4. 添加单元测试和集成测试
5. 编写用户文档和API文档
6. 优化性能和内存使用

## 总结
智能文档生成工具项目是一个很好的Rust学习项目，通过实际开发文档生成工具，可以系统性地掌握Rust编程的各个方面。目前项目处于初期阶段，核心数据结构已基本完成，接下来需要解决编译问题并实现核心功能。

---
*文档生成时间: 2024-03-29 10:30:00*
*生成工具: smart-doc v0.1.0*
```

## 🔍 调试和测试

### 1. 验证数据文件
```bash
# 检查JSON语法
python -m json.tool examples/sample_data.json
```

### 2. 测试模板渲染
```bash
# 手动测试模板渲染（需要安装tera-cli）
tera --template examples/simple_template.txt --var-file examples/sample_data.json
```

### 3. 查看生成的文件
```bash
# 查看文本文件
cat output/simple_doc.txt

# 查看DOCX文件内容（需要安装工具）
unzip -l output/project_doc.docx
```

## 📚 学习资源

### Rust相关
- [Rust官方文档](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [rustlings练习](https://github.com/rust-lang/rustlings)

### 模板引擎
- [Tera文档](https://tera.netlify.app/docs/)
- [Jinja2语法参考](https://jinja.palletsprojects.com/en/3.1.x/templates/)

### 文档处理
- [rdocx文档](https://docs.rs/rdocx)
- [Office Open XML规范](https://en.wikipedia.org/wiki/Office_Open_XML)

## 🐛 常见问题

### Q1: 模板变量没有被替换
**可能原因**:
1. 数据文件中没有对应的键
2. 模板语法错误
3. 变量名拼写错误

**解决方案**:
1. 检查数据文件是否包含所有需要的键
2. 验证模板语法是否正确
3. 使用`cargo run -- validate`命令验证模板

### Q2: 生成的文档格式不正确
**可能原因**:
1. 输出格式不支持
2. 模板内容包含特殊字符
3. 编码问题

**解决方案**:
1. 确保使用支持的格式（txt, docx等）
2. 在模板中正确处理特殊字符
3. 检查文件编码（推荐使用UTF-8）

### Q3: 编译错误
**可能原因**:
1. 依赖版本不匹配
2. 代码语法错误
3. 缺少必要的模块

**解决方案**:
1. 运行`cargo update`更新依赖
2. 检查错误信息，修复代码问题
3. 确保所有必要的模块都已实现

## 📞 支持与反馈

如果在使用示例文件时遇到问题，可以：

1. **查看项目文档**: 阅读`../doc/`目录下的文档
2. **检查编译错误**: 运行`cargo check`查看详细错误信息
3. **查看日志**: 设置`RUST_LOG=debug`环境变量查看详细日志
4. **提交问题**: 在项目仓库中提交issue

---

*祝您使用愉快！通过实践这些示例，您可以更好地理解智能文档生成工具的工作原理和使用方法。*