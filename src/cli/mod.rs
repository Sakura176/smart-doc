//! 命令行接口模块
//!
//! 提供命令行工具的功能实现。

use crate::core::DocumentBuilder;
use crate::error::{Error, Result};
use log::{error, info, warn};
use std::path::Path;

/// 生成文档
pub fn generate_document(
    template_path: &str,
    data_path: &str,
    output_path: &str,
    format: &str,
) -> Result<()> {
    info!("开始生成文档");

    // 检查模板文件是否存在
    let template_path = Path::new(template_path);
    if !template_path.exists() {
        return Err(Error::TemplateNotFound(template_path.to_path_buf()));
    }

    // 检查数据文件是否存在
    let data_path = Path::new(data_path);
    if !data_path.exists() {
        return Err(Error::DataNotFound(data_path.to_path_buf()));
    }

    // 读取数据文件
    let data_content = std::fs::read_to_string(data_path).map_err(|e| Error::Io(e))?;

    // 解析JSON数据
    let data: serde_json::Value =
        serde_json::from_str(&data_content).map_err(|e| Error::Json(e))?;

    info!("数据加载成功: {:?}", data);

    // 根据格式选择生成器
    match format.to_lowercase().as_str() {
        "docx" => {
            info!("生成DOCX文档");
            generate_docx_document(template_path, &data, output_path)
        }
        "txt" | "text" => {
            info!("生成文本文档");
            generate_text_document(&data, output_path)
        }
        _ => {
            warn!("不支持的格式: {}，使用文本格式", format);
            generate_text_document(&data, output_path)
        }
    }
}

/// 生成DOCX文档（简单实现）
fn generate_docx_document(
    template_path: &Path,
    data: &serde_json::Value,
    output_path: &str,
) -> Result<()> {
    // 这里先实现一个简单的版本
    // 实际应该使用rdocx库生成真正的DOCX文档

    info!("使用模板: {}", template_path.display());

    let builder = DocumentBuilder::new();
    builder.title(
        data.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("未命名文档"),
    );
    // 创建简单的文档内容
    let title = data
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("未命名文档");

    let author = data
        .get("author")
        .and_then(|v| v.as_str())
        .unwrap_or("未知作者");

    let content = data
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("暂无内容");

    // 创建简单的文本内容（暂时替代DOCX）
    let document_content = format!(
        "标题: {}\n作者: {}\n\n{}\n\n生成时间: {}",
        title,
        author,
        content,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    // 保存文档
    std::fs::write(output_path, document_content)
        .map_err(|e| Error::DocumentSave(e.to_string()))?;

    info!("文档保存成功: {}", output_path);
    Ok(())
}

/// 生成文本文档
fn generate_text_document(data: &serde_json::Value, output_path: &str) -> Result<()> {
    // 从数据中提取信息
    let title = data
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("未命名文档");

    let author = data
        .get("author")
        .and_then(|v| v.as_str())
        .unwrap_or("未知作者");

    let date = match data.get("date").and_then(|v| v.as_str()) {
        Some(date_str) => date_str.to_string(),
        None => chrono::Local::now().format("%Y-%m-%d").to_string(),
    };

    let content = data
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("暂无内容");

    // 构建文档内容
    let document_content = format!(
        "{}\n{}\n\n作者: {}\n日期: {}\n\n{}\n\n---\n生成工具: smart-doc v{}",
        "=".repeat(40),
        title,
        author,
        date,
        content,
        env!("CARGO_PKG_VERSION")
    );

    // 保存文档
    std::fs::write(output_path, document_content)
        .map_err(|e| Error::DocumentSave(e.to_string()))?;

    info!("文本文档保存成功: {}", output_path);
    Ok(())
}

/// 显示模板信息
pub fn show_template_info(template_path: &str) -> Result<()> {
    let path = Path::new(template_path);

    if !path.exists() {
        return Err(Error::TemplateNotFound(path.to_path_buf()));
    }

    let metadata = std::fs::metadata(path).map_err(|e| Error::Io(e))?;

    println!("模板信息:");
    println!("==========");
    println!("路径: {}", path.display());
    println!("大小: {} 字节", metadata.len());
    println!("类型: {}", get_file_type(path));
    println!("修改时间: {:?}", metadata.modified());

    // 如果是文本文件，显示前几行
    if is_text_file(path) {
        println!("\n文件预览:");
        println!("---------");
        if let Ok(content) = std::fs::read_to_string(path) {
            let lines: Vec<&str> = content.lines().take(10).collect();
            for (i, line) in lines.iter().enumerate() {
                println!("{:3}: {}", i + 1, line);
            }
            if content.lines().count() > 10 {
                println!("... (更多内容)");
            }
        }
    }

    Ok(())
}

/// 验证模板
pub fn validate_template(template_path: &str) -> Result<()> {
    let path = Path::new(template_path);

    if !path.exists() {
        return Err(Error::TemplateNotFound(path.to_path_buf()));
    }

    println!("验证模板: {}", path.display());

    // 检查文件是否可读
    match std::fs::read_to_string(path) {
        Ok(content) => {
            println!("✅ 文件可读");

            // 简单的模板语法检查
            let placeholder_count = content.matches("{{").count();
            let closing_count = content.matches("}}").count();

            if placeholder_count == closing_count {
                println!("✅ 模板占位符匹配: {} 对", placeholder_count);
            } else {
                warn!(
                    "⚠️  模板占位符不匹配: {{ 出现 {} 次, }} 出现 {} 次",
                    placeholder_count, closing_count
                );
            }

            // 检查常见的模板语法问题
            let lines: Vec<(usize, &str)> = content.lines().enumerate().collect();
            let mut has_errors = false;

            for (line_num, line) in lines {
                // 检查未闭合的占位符
                let open_count = line.matches("{{").count();
                let close_count = line.matches("}}").count();

                if open_count > close_count {
                    error!("第 {} 行: 有未闭合的 {{", line_num + 1);
                    has_errors = true;
                }

                // 检查嵌套的占位符（简单检查）
                if line.contains("{{{{") || line.contains("}}}}") {
                    warn!("第 {} 行: 可能有嵌套的占位符", line_num + 1);
                }
            }

            if !has_errors {
                println!("✅ 模板语法检查通过");
            } else {
                println!("❌ 模板语法检查失败");
            }
        }
        Err(e) => {
            error!("❌ 文件读取失败: {}", e);
            return Err(Error::Io(e));
        }
    }

    Ok(())
}

/// 列出可用模板
pub fn list_templates() -> Result<()> {
    let current_dir = std::env::current_dir().map_err(|e| Error::Io(e))?;

    let template_dir = current_dir.join("templates");

    println!("可用模板:");
    println!("==========");

    if template_dir.exists() {
        match std::fs::read_dir(&template_dir) {
            Ok(entries) => {
                let mut templates = Vec::new();

                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if is_template_file(&path) {
                            templates.push(path);
                        }
                    }
                }

                if templates.is_empty() {
                    println!("没有找到模板文件");
                    println!("请将模板文件放在 {} 目录中", template_dir.display());
                } else {
                    for (i, template) in templates.iter().enumerate() {
                        let filename = template
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("未知文件");

                        let metadata = std::fs::metadata(template)
                            .map(|m| format!("{} 字节", m.len()))
                            .unwrap_or_else(|_| "无法获取信息".to_string());

                        println!("{}. {} ({})", i + 1, filename, metadata);
                    }
                }
            }
            Err(e) => {
                error!("读取模板目录失败: {}", e);
                return Err(Error::Io(e));
            }
        }
    } else {
        println!("模板目录不存在: {}", template_dir.display());
        println!("正在创建模板目录...");

        if let Err(e) = std::fs::create_dir(&template_dir) {
            error!("创建模板目录失败: {}", e);
            return Err(Error::Io(e));
        }

        println!("✅ 已创建模板目录: {}", template_dir.display());
        println!("请将模板文件放在此目录中");
    }

    Ok(())
}

/// 获取文件类型
fn get_file_type(path: &Path) -> String {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            "docx" => "Microsoft Word 文档".to_string(),
            "txt" => "文本文件".to_string(),
            "json" => "JSON 数据文件".to_string(),
            "xml" => "XML 文件".to_string(),
            "html" | "htm" => "HTML 文件".to_string(),
            "md" => "Markdown 文件".to_string(),
            _ => format!("{} 文件", extension.to_string_lossy()),
        }
    } else {
        "未知类型".to_string()
    }
}

/// 检查是否为文本文件
fn is_text_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_str().unwrap_or("").to_lowercase();
        matches!(
            ext.as_str(),
            "txt" | "json" | "xml" | "html" | "htm" | "md" | "rst"
        )
    } else {
        false
    }
}

/// 检查是否为模板文件
fn is_template_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_str().unwrap_or("").to_lowercase();
        matches!(
            ext.as_str(),
            "docx" | "txt" | "json" | "xml" | "html" | "htm" | "md"
        )
    } else {
        false
    }
}
