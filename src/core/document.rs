//! 文档模块
//!
//! 定义文档的核心数据结构和操作方法。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 文档元素类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocElement {
    /// 段落
    Paragraph {
        /// 元素ID
        id: String,
        /// 文本内容
        text: String,
        /// 样式名称
        style: Option<String>,
        /// 对齐方式
        alignment: Option<Alignment>,
        /// 元数据
        metadata: HashMap<String, String>,
    },
    /// 表格
    Table {
        /// 元素ID
        id: String,
        /// 表格行数据
        rows: Vec<Vec<TableCell>>,
        /// 表头
        headers: Option<Vec<String>>,
        /// 表格样式
        style: Option<TableStyle>,
        /// 元数据
        metadata: HashMap<String, String>,
    },
    /// 图片
    Image {
        /// 元素ID
        id: String,
        /// 图片路径
        path: PathBuf,
        /// 替代文本
        alt_text: String,
        /// 宽度（像素）
        width: u32,
        /// 高度（像素）
        height: u32,
        /// 图片样式
        style: Option<ImageStyle>,
        /// 元数据
        metadata: HashMap<String, String>,
    },
    /// 列表
    List {
        /// 元素ID
        id: String,
        /// 列表项
        items: Vec<ListItem>,
        /// 是否有序列表
        ordered: bool,
        /// 列表样式
        style: Option<ListStyle>,
        /// 元数据
        metadata: HashMap<String, String>,
    },
    /// 页眉
    Header {
        /// 元素ID
        id: String,
        /// 内容元素
        content: Vec<DocElement>,
        /// 样式
        style: Option<String>,
        /// 元数据
        metadata: HashMap<String, String>,
    },
    /// 页脚
    Footer {
        /// 元素ID
        id: String,
        /// 内容元素
        content: Vec<DocElement>,
        /// 样式
        style: Option<String>,
        /// 元数据
        metadata: HashMap<String, String>,
    },
}

/// 对齐方式
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Alignment {
    /// 左对齐
    Left,
    /// 居中对齐
    Center,
    /// 右对齐
    Right,
    /// 两端对齐
    Justify,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

/// 表格单元格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// 单元格内容
    pub content: String,
    /// 跨行数
    pub rowspan: u32,
    /// 跨列数
    pub colspan: u32,
    /// 单元格样式
    pub style: Option<String>,
    /// 元数据
    pub metadata: HashMap<String, String>,
}

impl Default for TableCell {
    fn default() -> Self {
        Self {
            content: String::new(),
            rowspan: 1,
            colspan: 1,
            style: None,
            metadata: HashMap::new(),
        }
    }
}

impl TableCell {
    /// 创建新的表格单元格
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }
}

/// 表格样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStyle {
    /// 边框样式
    pub border: Option<BorderStyle>,
    /// 表头样式
    pub header_style: Option<String>,
    /// 单元格样式
    pub cell_style: Option<String>,
    /// 交替行样式
    pub alternate_row_style: Option<String>,
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderStyle {
    /// 边框宽度（像素）
    pub width: u32,
    /// 边框颜色（十六进制）
    pub color: String,
    /// 边框样式
    pub style: BorderType,
}

/// 边框类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BorderType {
    /// 实线
    Solid,
    /// 虚线
    Dashed,
    /// 点线
    Dotted,
    /// 双线
    Double,
}

/// 图片样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageStyle {
    /// 边框样式
    pub border: Option<BorderStyle>,
    /// 阴影效果
    pub shadow: Option<ShadowStyle>,
    /// 圆角半径（像素）
    pub borderRadius: u32,
}

/// 阴影样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowStyle {
    /// 水平偏移（像素）
    pub offset_x: i32,
    /// 垂直偏移（像素）
    pub offset_y: i32,
    /// 模糊半径（像素）
    pub blur_radius: u32,
    /// 阴影颜色（十六进制）
    pub color: String,
}

/// 列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    /// 缩进级别
    pub level: u32,
    /// 内容
    pub content: String,
    /// 样式
    pub style: Option<String>,
    /// 元数据
    pub metadata: HashMap<String, String>,
}

/// 列表样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListStyle {
    /// 项目符号样式
    pub bullet_style: Option<BulletStyle>,
    /// 编号样式
    pub number_style: Option<NumberStyle>,
    /// 缩进大小（像素）
    pub indent: u32,
}

/// 项目符号样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BulletStyle {
    /// 圆点
    Disc,
    /// 圆圈
    Circle,
    /// 方块
    Square,
    /// 自定义字符
    Custom(String),
}

/// 编号样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NumberStyle {
    /// 十进制数字
    Decimal,
    /// 小写罗马数字
    LowerRoman,
    /// 大写罗马数字
    UpperRoman,
    /// 小写字母
    LowerAlpha,
    /// 大写字母
    UpperAlpha,
}

/// 文档元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// 文档标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 主题
    pub subject: Option<String>,
    /// 关键词
    pub keywords: Vec<String>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 修改时间
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// 自定义元数据
    pub custom: HashMap<String, String>,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            title: String::new(),
            author: String::new(),
            subject: None,
            keywords: Vec::new(),
            created_at: now,
            modified_at: now,
            custom: HashMap::new(),
        }
    }
}

/// 文档结构
#[derive(Debug, Clone)]
pub struct Document {
    /// 文档元素集合
    pub elements: Vec<DocElement>,
    /// 文档元数据
    pub metadata: DocumentMetadata,
    /// 样式映射
    pub styles: HashMap<String, crate::core::style::Style>,
    /// 页面设置
    pub page_settings: Option<crate::core::PageSettings>,
}

impl Document {
    /// 创建新文档
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            metadata: DocumentMetadata::default(),
            styles: HashMap::new(),
            page_settings: None,
        }
    }

    /// 创建带标题的文档
    pub fn with_title(title: impl Into<String>, author: impl Into<String>) -> Self {
        let mut doc = Self::new();
        doc.metadata.title = title.into();
        doc.metadata.author = author.into();
        doc
    }

    /// 添加段落
    pub fn add_paragraph(&mut self, text: impl Into<String>) -> &mut Self {
        let id = uuid::Uuid::new_v4().to_string();
        self.elements.push(DocElement::Paragraph {
            id,
            text: text.into(),
            style: None,
            alignment: None,
            metadata: HashMap::new(),
        });
        self
    }

    /// 添加带样式的段落
    pub fn add_paragraph_with_style(
        &mut self,
        text: impl Into<String>,
        style: impl Into<String>,
    ) -> &mut Self {
        let id = uuid::Uuid::new_v4().to_string();
        self.elements.push(DocElement::Paragraph {
            id,
            text: text.into(),
            style: Some(style.into()),
            alignment: None,
            metadata: HashMap::new(),
        });
        self
    }

    /// 添加表格
    pub fn add_table(&mut self, rows: Vec<Vec<String>>) -> &mut Self {
        let id = uuid::Uuid::new_v4().to_string();

        // 转换字符串为表格单元格
        let table_rows: Vec<Vec<TableCell>> = rows
            .into_iter()
            .map(|row| row.into_iter().map(|cell| TableCell::new(cell)).collect())
            .collect();

        self.elements.push(DocElement::Table {
            id,
            rows: table_rows,
            headers: None,
            style: None,
            metadata: HashMap::new(),
        });
        self
    }

    /// 添加带表头的表格
    pub fn add_table_with_headers(
        &mut self,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> &mut Self {
        let id = uuid::Uuid::new_v4().to_string();

        // 转换字符串为表格单元格
        let table_rows: Vec<Vec<TableCell>> = rows
            .into_iter()
            .map(|row| row.into_iter().map(|cell| TableCell::new(cell)).collect())
            .collect();

        self.elements.push(DocElement::Table {
            id,
            rows: table_rows,
            headers: Some(headers),
            style: None,
            metadata: HashMap::new(),
        });
        self
    }

    /// 添加图片
    pub fn add_image(
        &mut self,
        path: impl Into<PathBuf>,
        alt_text: impl Into<String>,
        width: u32,
        height: u32,
    ) -> &mut Self {
        let id = uuid::Uuid::new_v4().to_string();
        self.elements.push(DocElement::Image {
            id,
            path: path.into(),
            alt_text: alt_text.into(),
            width,
            height,
            style: None,
            metadata: HashMap::new(),
        });
        self
    }

    /// 添加无序列表
    pub fn add_unordered_list(&mut self, items: Vec<String>) -> &mut Self {
        let id = uuid::Uuid::new_v4().to_string();

        let list_items: Vec<ListItem> = items
            .into_iter()
            .map(|content| ListItem {
                level: 0,
                content,
                style: None,
                metadata: HashMap::new(),
            })
            .collect();

        self.elements.push(DocElement::List {
            id,
            items: list_items,
            ordered: false,
            style: None,
            metadata: HashMap::new(),
        });
        self
    }

    /// 添加有序列表
    pub fn add_ordered_list(&mut self, items: Vec<String>) -> &mut Self {
        let id = uuid::Uuid::new_v4().to_string();

        let list_items: Vec<ListItem> = items
            .into_iter()
            .map(|content| ListItem {
                level: 0,
                content,
                style: None,
                metadata: HashMap::new(),
            })
            .collect();

        self.elements.push(DocElement::List {
            id,
            items: list_items,
            ordered: true,
            style: None,
            metadata: HashMap::new(),
        });
        self
    }

    /// 添加样式
    pub fn add_style(
        &mut self,
        name: impl Into<String>,
        style: crate::core::style::Style,
    ) -> &mut Self {
        self.styles.insert(name.into(), style);
        self
    }

    /// 设置页面设置
    pub fn set_page_settings(&mut self, settings: crate::core::PageSettings) -> &mut Self {
        self.page_settings = Some(settings);
        self
    }

    /// 获取文档元素数量
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// 检查文档是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// 获取文档标题
    pub fn title(&self) -> &str {
        &self.metadata.title
    }

    /// 获取作者
    pub fn author(&self) -> &str {
        &self.metadata.author
    }

    /// 更新修改时间
    pub fn update_modified_time(&mut self) {
        self.metadata.modified_at = chrono::Utc::now();
    }

    /// 添加关键词
    pub fn add_keyword(&mut self, keyword: impl Into<String>) -> &mut Self {
        self.metadata.keywords.push(keyword.into());
        self
    }

    /// 设置主题
    pub fn set_subject(&mut self, subject: impl Into<String>) -> &mut Self {
        self.metadata.subject = Some(subject.into());
        self
    }

    /// 添加自定义元数据
    pub fn add_custom_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.metadata.custom.insert(key.into(), value.into());
        self
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// 文档构建器（流式API）
pub struct DocumentBuilder {
    document: Document,
}

impl DocumentBuilder {
    /// 创建新的文档构建器
    pub fn new() -> Self {
        Self {
            document: Document::new(),
        }
    }

    /// 设置文档标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.document.metadata.title = title.into();
        self
    }

    /// 设置作者
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.document.metadata.author = author.into();
        self
    }

    /// 添加段落
    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.document.add_paragraph(text);
        self
    }

    /// 添加带样式的段落
    pub fn paragraph_with_style(
        mut self,
        text: impl Into<String>,
        style: impl Into<String>,
    ) -> Self {
        self.document.add_paragraph_with_style(text, style);
        self
    }

    /// 添加表格
    pub fn table(mut self, rows: Vec<Vec<String>>) -> Self {
        self.document.add_table(rows);
        self
    }

    /// 添加带表头的表格
    pub fn table_with_headers(mut self, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        self.document.add_table_with_headers(headers, rows);
        self
    }

    /// 构建文档
    pub fn build(self) -> Document {
        self.document
    }
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
