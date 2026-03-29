# 智能文档生成工具技术实现细节

## 目录
1. [核心数据结构设计](#核心数据结构设计)
2. [模板引擎实现](#模板引擎实现)
3. [文档生成器实现](#文档生成器实现)
4. [性能优化策略](#性能优化策略)
5. [AI集成架构](#ai集成架构)
6. [服务层设计](#服务层设计)
7. [错误处理策略](#错误处理策略)
8. [测试策略](#测试策略)

## 核心数据结构设计

### 1.1 文档内部表示（IR）

```rust
// core/src/element.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 文档元素类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocElement {
    /// 段落
    Paragraph {
        id: String,
        text: String,
        style: Option<String>,
        metadata: HashMap<String, String>,
    },
    /// 表格
    Table {
        id: String,
        rows: Vec<Vec<TableCell>>,
        headers: Option<Vec<String>>,
        style: Option<TableStyle>,
    },
    /// 图片
    Image {
        id: String,
        path: PathBuf,
        alt_text: String,
        width: u32,
        height: u32,
        style: Option<ImageStyle>,
    },
    /// 列表
    List {
        id: String,
        items: Vec<ListItem>,
        ordered: bool,
        style: Option<ListStyle>,
    },
    /// 页眉页脚
    HeaderFooter {
        id: String,
        content: Vec<DocElement>,
        page_type: HeaderFooterType,
    },
}

/// 表格单元格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    pub content: String,
    pub rowspan: u32,
    pub colspan: u32,
    pub style: Option<String>,
}

/// 列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub level: u32,
    pub content: String,
    pub style: Option<String>,
}

// core/src/document.rs

/// 文档结构
pub struct Document {
    /// 文档元素集合
    pub elements: Vec<DocElement>,
    /// 样式定义
    pub styles: HashMap<String, Style>,
    /// 文档元数据
    pub metadata: DocumentMetadata,
    /// 页面设置
    pub page_settings: PageSettings,
}

/// 文档元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: String,
    pub author: String,
    pub subject: Option<String>,
    pub keywords: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// 页面设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSettings {
    pub size: PageSize,
    pub orientation: PageOrientation,
    pub margins: Margins,
    pub header_margin: f32,
    pub footer_margin: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    Custom { width: f32, height: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}
```

### 1.2 样式系统

```rust
// core/src/style.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 样式定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub name: String,
    pub font: FontSettings,
    pub paragraph: ParagraphSettings,
    pub color: ColorSettings,
    pub spacing: SpacingSettings,
    pub border: Option<BorderSettings>,
    pub background: Option<BackgroundSettings>,
}

/// 字体设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSettings {
    pub family: String,
    pub size: f32,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontWeight {
    Normal,
    Bold,
    Light,
    Custom(u16),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// 段落设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphSettings {
    pub alignment: Alignment,
    pub line_spacing: f32,
    pub indent: IndentSettings,
    pub spacing_before: f32,
    pub spacing_after: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndentSettings {
    pub left: f32,
    pub right: f32,
    pub first_line: f32,
    pub hanging: f32,
}

/// 颜色设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSettings {
    pub text: String,
    pub background: Option<String>,
    pub accent: Option<String>,
}

/// 间距设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSettings {
    pub padding: Padding,
    pub margin: Margin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Padding {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margin {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// 样式管理器
pub struct StyleManager {
    styles: HashMap<String, Style>,
    default_styles: HashMap<StyleType, String>,
}

impl StyleManager {
    pub fn new() -> Self {
        let mut styles = HashMap::new();
        let mut default_styles = HashMap::new();
        
        // 添加默认样式
        let normal_style = Style {
            name: "Normal".to_string(),
            font: FontSettings {
                family: "Calibri".to_string(),
                size: 11.0,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
                color: "#000000".to_string(),
            },
            paragraph: ParagraphSettings {
                alignment: Alignment::Left,
                line_spacing: 1.15,
                indent: IndentSettings {
                    left: 0.0,
                    right: 0.0,
                    first_line: 0.0,
                    hanging: 0.0,
                },
                spacing_before: 0.0,
                spacing_after: 8.0,
            },
            color: ColorSettings {
                text: "#000000".to_string(),
                background: None,
                accent: None,
            },
            spacing: SpacingSettings {
                padding: Padding {
                    top: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                    right: 0.0,
                },
                margin: Margin {
                    top: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                    right: 0.0,
                },
            },
            border: None,
            background: None,
        };
        
        styles.insert("Normal".to_string(), normal_style);
        default_styles.insert(StyleType::Paragraph, "Normal".to_string());
        
        Self { styles, default_styles }
    }
    
    pub fn get_style(&self, name: &str) -> Option<&Style> {
        self.styles.get(name)
    }
    
    pub fn add_style(&mut self, style: Style) {
        self.styles.insert(style.name.clone(), style);
    }
    
    pub fn get_default_style(&self, style_type: StyleType) -> Option<&Style> {
        self.default_styles
            .get(&style_type)
            .and_then(|name| self.styles.get(name))
    }
}
```

## 模板引擎实现

### 2.1 模板系统架构

```rust
// core/src/template/mod.rs

use tera::{Tera, Context};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct TemplateEngine {
    tera: Tera,
    template_cache: HashMap<String, String>,
    template_dir: PathBuf,
}

impl TemplateEngine {
    pub fn new(template_dir: PathBuf) -> Result<Self, TemplateError> {
        let mut tera = Tera::default();
        
        // 加载模板目录
        let pattern = template_dir.join("**/*.html");
        tera.add_template_files(&[(&pattern.to_string_lossy(), None)])?;
        
        // 注册自定义过滤器
        Self::register_filters(&mut tera)?;
        
        // 注册自定义函数
        Self::register_functions(&mut tera)?;
        
        Ok(Self {
            tera,
            template_cache: HashMap::new(),
            template_dir,
        })
    }
    
    fn register_filters(tera: &mut Tera) -> Result<(), TemplateError> {
        // 日期格式化过滤器
        tera.register_filter("date_format", |value: &Value, args: &HashMap<String, Value>| {
            let date_str = value.as_str().ok_or_else(|| {
                tera::Error::msg("Value is not a string")
            })?;
            
            let format = args.get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("%Y-%m-%d");
            
            let date = chrono::DateTime::parse_from_rfc3339(date_str)
                .map_err(|e| tera::Error::msg(e.to_string()))?;
            
            Ok(Value::String(date.format(format).to_string()))
        });
        
        // 数字格式化过滤器
        tera.register_filter("number_format", |value: &Value, args: &HashMap<String, Value>| {
            let number = value.as_f64().ok_or_else(|| {
                tera::Error::msg("Value is not a number")
            })?;
            
            let decimals = args.get("decimals")
                .and_then(|v| v.as_u64())
                .unwrap_or(2) as usize;
            
            let formatted = format!("{:.*}", decimals, number);
            Ok(Value::String(formatted))
        });
        
        // 货币格式化过滤器
        tera.register_filter("currency", |value: &Value, args: &HashMap<String, Value>| {
            let number = value.as_f64().ok_or_else(|| {
                tera::Error::msg("Value is not a number")
            })?;
            
            let currency = args.get("currency")
                .and_then(|v| v.as_str())
                .unwrap_or("USD");
            
            let symbol = match currency {
                "USD" => "$",
                "EUR" => "€",
                "GBP" => "£",
                "CNY" => "¥",
                _ => "",
            };
            
            let formatted = format!("{}{:,.2}", symbol, number);
            Ok(Value::String(formatted))
        });
        
        Ok(())
    }
    
    fn register_functions(tera: &mut Tera) -> Result<(), TemplateError> {
        // 当前日期函数
        tera.register_function("now", |args: &HashMap<String, Value>| {
            let format = args.get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("%Y-%m-%d %H:%M:%S");
            
            let now = chrono::Utc::now();
            Ok(Value::String(now.format(format).to_string()))
        });
        
        // 生成UUID函数
        tera.register_function("uuid", |_args: &HashMap<String, Value>| {
            let uuid = uuid::Uuid::new_v4();
            Ok(Value::String(uuid.to_string()))
        });
        
        // 数学计算函数
        tera.register_function("math", |args: &HashMap<String, Value>| {
            let operation = args.get("op")
                .and_then(|v| v.as_str())
                .ok_or_else(|| tera::Error::msg("Missing 'op' argument"))?;
            
            let a = args.get("a")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| tera::Error::msg("Missing or invalid 'a' argument"))?;
            
            let b = args.get("b")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| tera::Error::msg("Missing or invalid 'b' argument"))?;
            
            let result = match operation {
                "add" => a + b,
                "sub" => a - b,
                "mul" => a * b,
                "div" => {
                    if b == 0.0 {
                        return Err(tera::Error::msg("Division by zero"));
                    }
                    a / b
                }
                "mod" => a % b,
                _ => return Err(tera::Error::msg(format!("Unknown operation: {}", operation))),
            };
            
            Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()))
        });
        
        Ok(())
    }
    
    pub fn render(&mut self, template_name: &str, data: &Value) -> Result<String, TemplateError> {
        // 检查缓存
        if let Some(cached) = self.template_cache.get(template_name) {
            let context = Context::from_serialize(data)?;
            return Ok(self.tera.render_str(cached, &context)?);
        }
        
        // 渲染模板
        let context = Context::from_serialize(data)?;
        let result = self.tera.render(template_name, &context)?;
        
        // 缓存模板内容
        if let Some(template) = self.tera.get_template(template_name) {
            self.template_cache.insert(template_name.to_string(), template.source().to_string());
        }
        
        Ok(result)
    }
    
    pub fn render_to_document(&mut self, template_name: &str, data: &Value) -> Result<Document, TemplateError> {
        let rendered = self.render(template_name, data)?;
        
        // 将渲染结果转换为文档
        let document = self.parse_rendered_content(&rendered)?;
        
        Ok(document)
    }
    
    fn parse_rendered_content(&self, content: &str) -> Result<Document, TemplateError> {
        // 解析渲染后的内容，转换为文档元素
        // 这里可以根据需要实现特定的解析逻辑
        // 例如：解析Markdown、HTML或自定义格式
        
        let mut document = Document::new();
        
        // 简单示例：按行分割为段落
        for line in content.lines() {
            if !line.trim().is_empty() {
                document.add_paragraph(line.trim());
            }
        }
        
        Ok(document)
    }
}
```

### 2.2 模板语法扩展

```rust
// core/src/template/extensions.rs

/// 自定义模板标签处理器
pub trait TemplateTagHandler {
    fn tag_name(&self) -> &str;
    fn process(&self, args: &[&str], context: &tera::Context) -> Result<String, tera::Error>;
}

/// 图表标签处理器
pub struct ChartTagHandler;

impl TemplateTagHandler for ChartTagHandler {
    fn tag_name(&self) -> &str {
        "chart"
    }
    
    fn process(&self, args: &[&str], context: &tera::Context) -> Result<String, tera::Error> {
        if args.len() < 2 {
            return Err(tera::Error::msg("Chart tag requires type and data parameters"));
        }
        
        let chart_type = args[0];
        let data_key = args[1];
        
        // 从上下文中获取数据
        let data = context.get(data_key)
            .ok_or_else(|| tera::Error::msg(format!("Data key '{}' not found in context", data_key)))?;
        
        // 生成图表HTML
        let chart_html = match chart_type {
            "bar" => generate_bar_chart(data),
            "line" => generate_line_chart(data),
            "pie" => generate_pie_chart(data),
            _ => return Err(tera::Error::msg(format!("Unknown chart type: {}", chart_type))),
        };
        
        Ok(chart_html)
    }
}

/// 表格标签处理器
pub struct TableTagHandler;

impl TemplateTagHandler for TableTagHandler {
    fn tag_name(&self) -> &str {
        "table"
    }
    
    fn process(&self, args: &[&str], context: &tera::Context) -> Result<String, tera::Error> {
        if args.is_empty() {
            return Err(tera::Error::msg("Table tag requires data parameter"));
        }
        
        let data_key = args[0];
        let data = context.get(data_key)
            .ok_or_else(|| tera::Error::msg(format!("Data key '{}' not found in context", data_key)))?;
        
        // 生成表格HTML
        let table_html = generate_table_html(data, args.get(1).unwrap_or(&"default"));
        
        Ok(table_html)
    }
}

/// 模板扩展管理器
pub struct TemplateExtensionManager {
    handlers: HashMap<String, Box<dyn TemplateTagHandler>>,
}

impl TemplateExtensionManager {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();
        
        // 注册默认处理器
        handlers.insert("chart".to_string(), Box::new(ChartTagHandler));
        handlers.insert("table".to_string(), Box::new(TableTagHandler));
        
        Self { handlers }
    }
    
    pub fn register_handler(&mut self, handler: Box<dyn TemplateTagHandler>) {
        self.handlers.insert(handler.tag_name().to_string(), handler);
    }
    
    pub fn process_tag(&self, tag_name: &str, args: &[&str], context: &