//! 基础测试模块
//!
//! 测试智能文档生成工具的核心功能。

use smart_doc_core::core::document::{Document, DocumentBuilder};
use smart_doc_core::core::{GenerateOptions, OutputFormat};
use smart_doc_core::error::Result;
use tempfile::NamedTempFile;

/// 测试文档创建
#[test]
fn test_document_creation() {
    let mut doc = Document::new();
    doc.add_paragraph("这是一个测试段落");

    assert_eq!(doc.title(), "");
    assert_eq!(doc.author(), "");
    assert_eq!(doc.len(), 1);
    assert!(!doc.is_empty());
}

/// 测试带标题的文档
#[test]
fn test_document_with_title() {
    let doc = Document::with_title("测试文档", "测试作者");

    assert_eq!(doc.title(), "测试文档");
    assert_eq!(doc.author(), "测试作者");
    assert_eq!(doc.len(), 0);
    assert!(doc.is_empty());
}

/// 测试文档构建器
#[test]
fn test_document_builder() {
    let doc = DocumentBuilder::new()
        .title("构建器测试文档")
        .author("构建器作者")
        .paragraph("第一段")
        .paragraph("第二段")
        .table(vec![
            vec!["A1".to_string(), "B1".to_string()],
            vec!["A2".to_string(), "B2".to_string()],
        ])
        .build();

    assert_eq!(doc.title(), "构建器测试文档");
    assert_eq!(doc.author(), "构建器作者");
    assert_eq!(doc.len(), 3); // 两个段落 + 一个表格
}

/// 测试输出格式解析
#[test]
fn test_output_format_parsing() {
    // 测试有效的格式
    assert_eq!(OutputFormat::from_str("docx"), Some(OutputFormat::Docx));
    assert_eq!(OutputFormat::from_str("DOCX"), Some(OutputFormat::Docx));
    assert_eq!(OutputFormat::from_str("xlsx"), Some(OutputFormat::Xlsx));
    assert_eq!(OutputFormat::from_str("pdf"), Some(OutputFormat::Pdf));
    assert_eq!(OutputFormat::from_str("html"), Some(OutputFormat::Html));
    assert_eq!(OutputFormat::from_str("txt"), Some(OutputFormat::Text));
    assert_eq!(OutputFormat::from_str("text"), Some(OutputFormat::Text));

    // 测试无效的格式
    assert_eq!(OutputFormat::from_str("invalid"), None);
    assert_eq!(OutputFormat::from_str(""), None);
}

/// 测试输出格式扩展名
#[test]
fn test_output_format_extension() {
    assert_eq!(OutputFormat::Docx.extension(), "docx");
    assert_eq!(OutputFormat::Xlsx.extension(), "xlsx");
    assert_eq!(OutputFormat::Pdf.extension(), "pdf");
    assert_eq!(OutputFormat::Html.extension(), "html");
    assert_eq!(OutputFormat::Text.extension(), "txt");
}

/// 测试输出格式显示
#[test]
fn test_output_format_display() {
    assert_eq!(format!("{}", OutputFormat::Docx), "docx");
    assert_eq!(format!("{}", OutputFormat::Xlsx), "xlsx");
    assert_eq!(format!("{}", OutputFormat::Pdf), "pdf");
    assert_eq!(format!("{}", OutputFormat::Html), "html");
    assert_eq!(format!("{}", OutputFormat::Text), "txt");
}

/// 测试从字符串解析输出格式
#[test]
fn test_output_format_from_str() {
    use std::str::FromStr;

    assert_eq!(OutputFormat::from_str("docx"), Ok(OutputFormat::Docx));
    assert_eq!(OutputFormat::from_str("xlsx"), Ok(OutputFormat::Xlsx));
    assert_eq!(OutputFormat::from_str("pdf"), Ok(OutputFormat::Pdf));
    assert_eq!(OutputFormat::from_str("html"), Ok(OutputFormat::Html));
    assert_eq!(OutputFormat::from_str("txt"), Ok(OutputFormat::Text));

    // 测试错误情况
    assert!(OutputFormat::from_str("invalid").is_err());
    assert!(OutputFormat::from_str("").is_err());
}

/// 测试生成选项默认值
#[test]
fn test_generate_options_default() {
    let options = GenerateOptions::default();

    assert_eq!(options.compress, false);
    assert_eq!(options.quality, 0);
    assert_eq!(options.include_metadata, false);
    assert_eq!(options.include_styles, false);
    assert_eq!(options.page_settings, None);
}

/// 测试文档元数据
#[test]
fn test_document_metadata() {
    let mut doc = Document::with_title("测试文档", "测试作者");

    // 测试默认值
    assert_eq!(doc.metadata.title, "测试文档");
    assert_eq!(doc.metadata.author, "测试作者");
    assert_eq!(doc.metadata.subject, None);
    assert!(doc.metadata.keywords.is_empty());
    assert!(!doc.metadata.custom.is_empty()); // 创建时间等会被设置

    // 测试添加关键词
    doc.add_keyword("测试");
    doc.add_keyword("文档");
    assert_eq!(doc.metadata.keywords.len(), 2);
    assert_eq!(doc.metadata.keywords[0], "测试");
    assert_eq!(doc.metadata.keywords[1], "文档");

    // 测试设置主题
    doc.set_subject("这是一个测试主题");
    assert_eq!(doc.metadata.subject, Some("这是一个测试主题".to_string()));

    // 测试添加自定义元数据
    doc.add_custom_metadata("custom_key", "custom_value");
    assert_eq!(
        doc.metadata.custom.get("custom_key"),
        Some(&"custom_value".to_string())
    );
}

/// 测试文档元素操作
#[test]
fn test_document_elements() {
    let mut doc = Document::new();

    // 添加段落
    doc.add_paragraph("段落1");
    doc.add_paragraph_with_style("段落2", "强调");

    // 添加表格
    doc.add_table(vec![
        vec!["A1".to_string(), "B1".to_string()],
        vec!["A2".to_string(), "B2".to_string()],
    ]);

    // 添加带表头的表格
    doc.add_table_with_headers(
        vec!["列1".to_string(), "列2".to_string()],
        vec![
            vec!["数据1".to_string(), "数据2".to_string()],
            vec!["数据3".to_string(), "数据4".to_string()],
        ],
    );

    // 添加列表
    doc.add_unordered_list(vec![
        "项目1".to_string(),
        "项目2".to_string(),
        "项目3".to_string(),
    ]);

    doc.add_ordered_list(vec![
        "第一步".to_string(),
        "第二步".to_string(),
        "第三步".to_string(),
    ]);

    assert_eq!(doc.len(), 6);
}

/// 测试错误处理宏
#[test]
fn test_error_macros() {
    use smart_doc_core::{ensure, ensure_eq};

    // 测试 ensure 宏
    let result: Result<()> = (|| {
        ensure!(true, "不应该失败");
        Ok(())
    })();
    assert!(result.is_ok());

    let result: Result<()> = (|| {
        ensure!(false, "测试错误");
        Ok(())
    })();
    assert!(result.is_err());

    // 测试 ensure_eq 宏
    let result: Result<()> = (|| {
        ensure_eq!(1, 1, "相等不应该失败");
        Ok(())
    })();
    assert!(result.is_ok());

    let result: Result<()> = (|| {
        ensure_eq!(1, 2, "不相等应该失败");
        Ok(())
    })();
    assert!(result.is_err());
}

/// 测试 CLI 模块的基本功能
#[test]
fn test_cli_basic_functions() -> Result<()> {
    use smart_doc_core::cli;

    // 创建临时文件用于测试
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap();

    // 测试文本文档生成
    let data = serde_json::json!({
        "title": "测试文档",
        "author": "测试作者",
        "date": "2024-01-15",
        "content": "这是测试内容"
    });

    // 将数据写入临时文件
    std::fs::write(temp_path, serde_json::to_string_pretty(&data)?)?;

    // 测试生成文本文档
    let output_path = "test_output.txt";
    let result = cli::generate_document("fake_template.txt", temp_path, output_path, "txt");

    // 清理测试文件
    let _ = std::fs::remove_file(output_path);

    // 由于模板文件不存在，应该返回错误
    assert!(result.is_err());

    Ok(())
}

/// 测试文件类型检测
#[test]
fn test_file_type_detection() {
    use smart_doc_core::cli;
    use std::path::Path;

    // 注意：这里我们无法直接测试私有函数，所以这个测试主要是演示
    // 在实际项目中，可以将这些函数设为pub(crate)以便测试

    // 测试路径扩展名提取
    let path = Path::new("test.docx");
    assert_eq!(path.extension().unwrap().to_str().unwrap(), "docx");

    let path = Path::new("test.txt");
    assert_eq!(path.extension().unwrap().to_str().unwrap(), "txt");
}

/// 集成测试：完整的文档创建流程
#[test]
fn test_integration_document_workflow() {
    // 创建文档
    let mut doc = Document::with_title("集成测试文档", "集成测试作者");

    // 添加内容
    doc.add_paragraph("这是文档的第一段内容。")
        .add_paragraph("这是文档的第二段内容。")
        .add_keyword("集成测试")
        .add_keyword("文档生成")
        .set_subject("集成测试主题");

    // 验证文档状态
    assert_eq!(doc.title(), "集成测试文档");
    assert_eq!(doc.author(), "集成测试作者");
    assert_eq!(doc.len(), 2);
    assert_eq!(doc.metadata.keywords.len(), 2);
    assert_eq!(doc.metadata.subject, Some("集成测试主题".to_string()));

    // 更新修改时间
    let old_modified = doc.metadata.modified_at;
    doc.update_modified_time();
    assert!(doc.metadata.modified_at > old_modified);
}

/// 测试文档序列化和反序列化
#[test]
fn test_document_serialization() -> Result<()> {
    use serde_json;

    // 创建文档
    let mut doc = Document::with_title("序列化测试", "测试作者");
    doc.add_paragraph("测试段落")
        .add_keyword("序列化")
        .add_keyword("测试");

    // 序列化为JSON
    let json = serde_json::to_string(&doc.metadata)?;

    // 反序列化
    let metadata: smart_doc_core::core::document::DocumentMetadata = serde_json::from_str(&json)?;

    // 验证
    assert_eq!(metadata.title, "序列化测试");
    assert_eq!(metadata.author, "测试作者");
    assert_eq!(metadata.keywords.len(), 2);

    Ok(())
}

/// 测试错误类型
#[test]
fn test_error_types() {
    use smart_doc_core::error::Error;

    // 测试错误创建
    let template_error = Error::template("模板错误");
    assert!(matches!(template_error, Error::Template(_)));

    let data_parse_error = Error::data_parse("数据解析错误");
    assert!(matches!(data_parse_error, Error::DataParse(_)));

    let unsupported_format_error = Error::unsupported_format("xyz");
    assert!(matches!(
        unsupported_format_error,
        Error::UnsupportedFormat(_)
    ));

    // 测试错误显示
    assert!(template_error.to_string().contains("模板错误"));
    assert!(data_parse_error.to_string().contains("数据解析错误"));
    assert!(
        unsupported_format_error
            .to_string()
            .contains("不支持的格式")
    );
}
