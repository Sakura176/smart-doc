//! 核心模块
//!
//! 包含文档生成的核心数据结构和逻辑。

pub mod document;
pub mod generator;
// pub mod template;
pub mod style;

// 重新导出常用类型
pub use document::{DocElement, Document, DocumentBuilder, DocumentMetadata};
pub use generator::DocumentGenerator;
// pub use template::TemplateEngine;
pub use style::{Style, StyleManager};

/// 输出格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    /// DOCX 格式 (Microsoft Word)
    Docx,
    /// XLSX 格式 (Microsoft Excel)
    Xlsx,
    /// PDF 格式
    Pdf,
    /// HTML 格式
    Html,
    /// 纯文本格式
    Text,
}

impl OutputFormat {
    /// 从字符串解析输出格式
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "docx" => Some(OutputFormat::Docx),
            "xlsx" => Some(OutputFormat::Xlsx),
            "pdf" => Some(OutputFormat::Pdf),
            "html" => Some(OutputFormat::Html),
            "txt" | "text" => Some(OutputFormat::Text),
            _ => None,
        }
    }

    /// 获取格式的默认文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Docx => "docx",
            OutputFormat::Xlsx => "xlsx",
            OutputFormat::Pdf => "pdf",
            OutputFormat::Html => "html",
            OutputFormat::Text => "txt",
        }
    }

    /// 获取格式的描述信息
    pub fn description(&self) -> &'static str {
        match self {
            OutputFormat::Docx => "Microsoft Word Document",
            OutputFormat::Xlsx => "Microsoft Excel Workbook",
            OutputFormat::Pdf => "Portable Document Format",
            OutputFormat::Html => "HyperText Markup Language",
            OutputFormat::Text => "Plain Text",
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.extension())
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OutputFormat::from_str(s).ok_or_else(|| {
            format!(
                "不支持的格式: '{}'。支持的格式有: docx, xlsx, pdf, html, txt",
                s
            )
        })
    }
}

/// 文档生成选项
#[derive(Debug, Clone, Default)]
pub struct GenerateOptions {
    /// 是否压缩输出
    pub compress: bool,
    /// 输出质量 (0-100)
    pub quality: u8,
    /// 是否包含元数据
    pub include_metadata: bool,
    /// 是否包含样式
    pub include_styles: bool,
    /// 页面设置
    pub page_settings: Option<PageSettings>,
}

/// 页面设置
#[derive(Debug, Clone, PartialEq)]
pub struct PageSettings {
    /// 页面大小
    pub size: PageSize,
    /// 页面方向
    pub orientation: PageOrientation,
    /// 页边距 (单位: 毫米)
    pub margins: Margins,
}

/// 页面大小
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageSize {
    /// A4 纸张 (210mm × 297mm)
    A4,
    /// Letter 纸张 (216mm × 279mm)
    Letter,
    /// Legal 纸张 (216mm × 356mm)
    Legal,
    /// 自定义大小
    Custom { width: f32, height: f32 },
}

impl PageSize {
    /// 获取页面尺寸 (宽, 高)，单位: 毫米
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            PageSize::A4 => (210.0, 297.0),
            PageSize::Letter => (216.0, 279.0),
            PageSize::Legal => (216.0, 356.0),
            PageSize::Custom { width, height } => (*width, *height),
        }
    }
}

impl Default for PageSize {
    fn default() -> Self {
        PageSize::A4
    }
}

/// 页面方向
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PageOrientation {
    /// 纵向
    #[default]
    Portrait,
    /// 横向
    Landscape,
}

/// 页边距
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Margins {
    /// 上边距 (毫米)
    pub top: f32,
    /// 下边距 (毫米)
    pub bottom: f32,
    /// 左边距 (毫米)
    pub left: f32,
    /// 右边距 (毫米)
    pub right: f32,
}

impl Margins {
    /// 创建对称的页边距
    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }

    /// 创建所有边距相同的页边距
    pub fn all(value: f32) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }
}

/// 文档生成器接口
pub trait DocumentGeneratorTrait {
    /// 生成文档
    fn generate(
        &self,
        document: &Document,
        format: OutputFormat,
        options: &GenerateOptions,
    ) -> crate::error::Result<Vec<u8>>;

    /// 生成并保存文档到文件
    fn generate_to_file(
        &self,
        document: &Document,
        format: OutputFormat,
        output_path: &std::path::Path,
        options: &GenerateOptions,
    ) -> crate::error::Result<()> {
        let data = self.generate(document, format, options)?;
        std::fs::write(output_path, data)?;
        Ok(())
    }
}

/// 模板渲染器接口
pub trait TemplateRenderer {
    /// 渲染模板
    fn render(&self, template: &str, data: &serde_json::Value) -> crate::error::Result<String>;

    /// 渲染模板并创建文档
    fn render_to_document(
        &self,
        template: &str,
        data: &serde_json::Value,
    ) -> crate::error::Result<Document>;
}

/// 核心配置
#[derive(Debug, Clone)]
pub struct CoreConfig {
    /// 模板目录
    pub template_dir: std::path::PathBuf,
    /// 输出目录
    pub output_dir: std::path::PathBuf,
    /// 默认输出格式
    pub default_format: OutputFormat,
    /// 默认生成选项
    pub default_options: GenerateOptions,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            template_dir: std::path::PathBuf::from("templates"),
            output_dir: std::path::PathBuf::from("output"),
            default_format: OutputFormat::Docx,
            default_options: GenerateOptions::default(),
        }
    }
}

/// 核心引擎
pub struct CoreEngine {
    /// 配置
    config: CoreConfig,
    /// 文档生成器
    generator: Box<dyn DocumentGeneratorTrait>,
    /// 模板渲染器
    renderer: Box<dyn TemplateRenderer>,
    /// 样式管理器
    style_manager: StyleManager,
}

impl CoreEngine {
    /// 创建新的核心引擎
    pub fn new(
        config: CoreConfig,
        generator: Box<dyn DocumentGeneratorTrait>,
        renderer: Box<dyn TemplateRenderer>,
    ) -> Self {
        Self {
            config,
            generator,
            renderer,
            style_manager: StyleManager::new(),
        }
    }

    /// 获取配置
    pub fn config(&self) -> &CoreConfig {
        &self.config
    }

    /// 获取样式管理器
    pub fn style_manager(&self) -> &StyleManager {
        &self.style_manager
    }

    /// 获取样式管理器（可变引用）
    pub fn style_manager_mut(&mut self) -> &mut StyleManager {
        &mut self.style_manager
    }

    /// 从模板生成文档
    pub fn generate_from_template(
        &self,
        template_name: &str,
        data: &serde_json::Value,
        output_format: Option<OutputFormat>,
        options: Option<&GenerateOptions>,
    ) -> crate::error::Result<Vec<u8>> {
        // 构建模板路径
        let template_path = self.config.template_dir.join(template_name);

        // 读取模板内容
        let template_content = std::fs::read_to_string(&template_path)
            .map_err(|_| crate::error::Error::TemplateNotFound(template_path.clone()))?;

        // 渲染模板
        let _rendered = self.renderer.render(&template_content, data)?;

        // 创建文档
        let document = self.renderer.render_to_document(&template_content, data)?;

        // 确定输出格式
        let format = output_format.unwrap_or(self.config.default_format);

        // 确定生成选项
        let options = options.unwrap_or(&self.config.default_options);

        // 生成文档
        self.generator.generate(&document, format, options)
    }

    /// 从模板生成文档并保存到文件
    pub fn generate_from_template_to_file(
        &self,
        template_name: &str,
        data: &serde_json::Value,
        output_filename: &str,
        output_format: Option<OutputFormat>,
        options: Option<&GenerateOptions>,
    ) -> crate::error::Result<()> {
        // 生成文档数据
        let data = self.generate_from_template(template_name, data, output_format, options)?;

        // 构建输出路径
        let output_path = self.config.output_dir.join(output_filename);

        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 保存文档
        std::fs::write(&output_path, data)?;

        Ok(())
    }

    /// 直接生成文档
    pub fn generate_document(
        &self,
        document: &Document,
        output_format: Option<OutputFormat>,
        options: Option<&GenerateOptions>,
    ) -> crate::error::Result<Vec<u8>> {
        let format = output_format.unwrap_or(self.config.default_format);
        let options = options.unwrap_or(&self.config.default_options);
        self.generator.generate(document, format, options)
    }

    /// 直接生成文档并保存到文件
    pub fn generate_document_to_file(
        &self,
        document: &Document,
        output_filename: &str,
        output_format: Option<OutputFormat>,
        options: Option<&GenerateOptions>,
    ) -> crate::error::Result<()> {
        // 生成文档数据
        let data = self.generate_document(document, output_format, options)?;

        // 构建输出路径
        let output_path = self.config.output_dir.join(output_filename);

        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 保存文档
        std::fs::write(&output_path, data)?;

        Ok(())
    }
}
