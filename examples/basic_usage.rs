//! 智能文档生成工具基础使用示例
//!
//! 这个示例展示了如何使用 smart-doc-core 库创建和生成文档。

use smart_doc_core::core::document::{Document, DocumentBuilder};
use smart_doc_core::core::{OutputFormat, GenerateOptions, PageSettings, PageSize, Margins};
use smart_doc_core::error::Result;
use smart_doc_core::cli;

fn main() -> Result<()> {
    println!("智能文档生成工具 - 基础使用示例");
    println!("================================\n");

    // 示例1: 创建简单文档
    example_create_simple_document()?;

    // 示例2: 使用文档构建器
    example_use_document_builder()?;

    // 示例3: 添加样式和页面设置
    example_with_styles_and_pages()?;

    // 示例4: 使用CLI功能
    example_cli_functionality()?;

    println!("\n所有示例执行完成！");
    Ok(())
}

/// 示例1: 创建简单文档
fn example_create_simple_document() -> Result<()> {
    println!("示例1: 创建简单文档");
    println!("------------------");

    // 创建新文档
    let mut doc = Document::with_title("我的第一个文档", "开发者");

    // 添加段落
    doc.add_paragraph("欢迎使用智能文档生成工具！")
        .add_paragraph("这是一个基于Rust开发的文档生成库。")
        .add_paragraph("支持多种文档格式和丰富的样式选项。");

    // 添加表格
    doc.add_table(vec![
        vec!["功能".to_string(), "状态".to_string()],
        vec!["DOCX生成".to_string(), "✅ 已完成".to_string()],
        vec!["PDF生成".to_string(), "🔄 开发中".to_string()],
        vec!["Excel生成".to_string(), "📅 计划中".to_string()],
    ]);

    // 添加列表
    doc.add_unordered_list(vec![
        "高性能".to_string(),
        "内存安全".to_string(),
        "易于使用".to_string(),
    ]);

    // 添加元数据
    doc.add_keyword("文档生成")
        .add_keyword("Rust")
        .add_keyword("自动化")
        .set_subject("技术文档生成工具");

    // 显示文档信息
    println!("文档标题: {}", doc.title());
    println!("文档作者: {}", doc.author());
    println!("文档元素数量: {}", doc.len());
    println!("关键词: {:?}", doc.metadata.keywords);
    println!("主题: {:?}", doc.metadata.subject);

    // 保存文档到文件（这里只是演示，实际需要实现生成器）
    println!("文档创建成功！\n");
    Ok(())
}

/// 示例2: 使用文档构建器
fn example_use_document_builder() -> Result<()> {
    println!("示例2: 使用文档构建器（流式API）");
    println!("-----------------------------");

    // 使用构建器创建文档
    let doc = DocumentBuilder::new()
        .title("项目进度报告")
        .author("项目经理")
        .paragraph("项目概述")
        .paragraph_with_style("当前状态：按计划进行", "强调")
        .table_with_headers(
            vec!["任务".to_string(), "负责人".to_string(), "进度".to_string()],
            vec![
                vec!["需求分析".to_string(), "张三".to_string(), "100%".to_string()],
                vec!["设计阶段".to_string(), "李四".to_string(), "80%".to_string()],
                vec!["开发阶段".to_string(), "王五".to_string(), "60%".to_string()],
                vec!["测试阶段".to_string(), "赵六".to_string(), "20%".to_string()],
            ],
        )
        .paragraph("下一步计划：")
        .add_ordered_list(vec![
            "完成核心功能开发".to_string(),
            "进行集成测试".to_string(),
            "编写用户文档".to_string(),
            "发布v1.0版本".to_string(),
        ])
        .build();

    println!("文档标题: {}", doc.title());
    println!("文档作者: {}", doc.author());
    println!("表格行数: {}", {
        if let Some(smart_doc_core::core::document::DocElement::Table { rows, .. }) = doc.elements.get(2) {
            rows.len()
        } else {
            0
        }
    });
    println!("文档构建完成！\n");

    Ok(())
}

/// 示例3: 添加样式和页面设置
fn example_with_styles_and_pages() -> Result<()> {
    println!("示例3: 添加样式和页面设置");
    println!("------------------------");

    let mut doc = Document::with_title("带样式的文档", "设计师");

    // 添加页面设置
    let page_settings = PageSettings {
        size: PageSize::A4,
        orientation: smart_doc_core::core::PageOrientation::Portrait,
        margins: Margins::symmetric(25.0, 20.0), // 对称页边距
    };
    doc.set_page_settings(page_settings);

    // 添加段落（实际使用中会应用样式）
    doc.add_paragraph("这是一个带样式的文档。")
        .add_paragraph_with_style("这是强调段落", "强调样式")
        .add_paragraph_with_style("这是标题段落", "标题样式");

    // 创建生成选项
    let options = GenerateOptions {
        compress: true,
        quality: 90,
        include_metadata: true,
        include_styles: true,
        page_settings: Some(PageSettings {
            size: PageSize::Letter,
            orientation: smart_doc_core::core::PageOrientation::Portrait,
            margins: Margins::all(20.0), // 所有边距相同
        }),
    };

    println!("页面设置: {:?}", doc.page_settings);
    println!("生成选项 - 压缩: {}", options.compress);
    println!("生成选项 - 质量: {}", options.quality);
    println!("生成选项 - 包含元数据: {}", options.include_metadata);
    println!("样式功能已准备就绪！\n");

    Ok(())
}

/// 示例4: 使用CLI功能
fn example_cli_functionality() -> Result<()> {
    println!("示例4: 使用CLI功能");
    println!("-----------------");

    // 创建示例数据文件
    let data_content = r#"
{
    "title": "CLI测试文档",
    "author": "CLI用户",
    "date": "2024-01-15",
    "content": "这是一个通过CLI生成的测试文档。\n展示了命令行工具的基本功能。",
    "sections": [
        "介绍",
        "功能说明",
        "使用示例",
        "总结"
    ]
}
"#;

    let data_path = "example_data.json";
    std::fs::write(data_path, data_content)?;

    // 创建示例模板文件
    let template_content = r#"
# {{title}}

作者: {{author}}
日期: {{date}}

{{content}}

## 章节列表
{% for section in sections %}
- {{section}}
{% endfor %}

---
生成时间: {{now}}
"#;

    let template_path = "example_template.txt";
    std::fs::write(template_path, template_content)?;

    println!("已创建示例文件:");
    println!("- 数据文件: {}", data_path);
    println!("- 模板文件: {}", template_path);

    // 演示CLI功能（这里只是打印信息，实际调用需要实现）
    println!("\nCLI命令示例:");
    println!("1. 生成文档: smart-doc generate --template {} --data {} --output output.txt",
             template_path, data_path);
    println!("2. 验证模板: smart-doc validate --template {}", template_path);
    println!("3. 列出模板: smart-doc list");
    println!("4. 查看信息: smart-doc info --template {}", template_path);

    // 清理临时文件
    std::fs::remove_file(data_path)?;
    std::fs::remove_file(template_path)?;

    println!("\nCLI功能演示完成！");
    Ok(())
}

/// 运行示例
///
/// 使用方法:
/// ```bash
/// cargo run --example basic_usage
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() -> Result<()> {
        // 测试各个示例函数
        example_create_simple_document()?;
        example_use_document_builder()?;
        example_with_styles_and_pages()?;

        // 注意：example_cli_functionality 会创建和删除文件，
        // 在测试环境中可能需要特殊处理
        Ok(())
    }
}
