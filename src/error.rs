//! 错误处理模块
//!
//! 定义项目中的错误类型和处理逻辑。

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// 项目结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

/// 项目错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// IO错误
    #[error("IO错误: {0}")]
    Io(#[from] io::Error),

    /// 模板错误
    #[error("模板错误: {0}")]
    Template(String),

    /// 模板文件不存在
    #[error("模板文件不存在: {0}")]
    TemplateNotFound(PathBuf),

    /// 模板语法错误
    #[error("模板语法错误: {0}")]
    TemplateSyntax(String),

    /// 模板渲染错误
    #[error("模板渲染错误: {0}")]
    TemplateRender(String),

    /// 数据解析错误
    #[error("数据解析错误: {0}")]
    DataParse(String),

    /// 数据文件不存在
    #[error("数据文件不存在: {0}")]
    DataNotFound(PathBuf),

    /// JSON解析错误
    #[error("JSON解析错误: {0}")]
    Json(#[from] serde_json::Error),

    /// XML解析错误
    #[error("XML解析错误: {0}")]
    Xml(String),

    /// ZIP压缩错误
    #[error("ZIP压缩错误: {0}")]
    Zip(String),

    /// 文档生成错误
    #[error("文档生成错误: {0}")]
    DocumentGeneration(String),

    /// 文档保存错误
    #[error("文档保存错误: {0}")]
    DocumentSave(String),

    /// 不支持的格式
    #[error("不支持的格式: {0}")]
    UnsupportedFormat(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),

    /// 参数错误
    #[error("参数错误: {0}")]
    Argument(String),

    /// 其他错误
    #[error("其他错误: {0}")]
    Other(String),
}

impl Error {
    /// 创建模板错误
    pub fn template(msg: impl Into<String>) -> Self {
        Error::Template(msg.into())
    }

    /// 创建模板语法错误
    pub fn template_syntax(msg: impl Into<String>) -> Self {
        Error::TemplateSyntax(msg.into())
    }

    /// 创建模板渲染错误
    pub fn template_render(msg: impl Into<String>) -> Self {
        Error::TemplateRender(msg.into())
    }

    /// 创建数据解析错误
    pub fn data_parse(msg: impl Into<String>) -> Self {
        Error::DataParse(msg.into())
    }

    /// 创建文档生成错误
    pub fn document_generation(msg: impl Into<String>) -> Self {
        Error::DocumentGeneration(msg.into())
    }

    /// 创建文档保存错误
    pub fn document_save(msg: impl Into<String>) -> Self {
        Error::DocumentSave(msg.into())
    }

    /// 创建不支持的格式错误
    pub fn unsupported_format(format: impl Into<String>) -> Self {
        Error::UnsupportedFormat(format.into())
    }

    /// 创建配置错误
    pub fn config(msg: impl Into<String>) -> Self {
        Error::Config(msg.into())
    }

    /// 创建参数错误
    pub fn argument(msg: impl Into<String>) -> Self {
        Error::Argument(msg.into())
    }

    /// 创建其他错误
    pub fn other(msg: impl Into<String>) -> Self {
        Error::Other(msg.into())
    }
}

/// 从tera错误转换
impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        // tera::Error 是一个结构体，内部包含 ErrorKind 枚举
        // 通过错误消息字符串来判断错误类型
        let err_str = err.to_string();

        // 根据错误消息内容判断错误类型
        if err_str.contains("Template '") && err_str.contains("' not found") {
            // 提取模板名称
            if let Some(start) = err_str.find("Template '") {
                let start_idx = start + "Template '".len();
                if let Some(end) = err_str[start_idx..].find("' not found") {
                    let template_name = &err_str[start_idx..start_idx + end];
                    Error::TemplateNotFound(PathBuf::from(template_name))
                } else {
                    Error::Template(err_str)
                }
            } else {
                Error::Template(err_str)
            }
        } else if err_str.contains("render")
            || err_str.contains("Render")
            || err_str.contains("渲染")
        {
            Error::TemplateRender(err_str)
        } else if err_str.contains("syntax")
            || err_str.contains("Syntax")
            || err_str.contains("语法")
        {
            Error::TemplateSyntax(err_str)
        } else {
            Error::Template(err_str)
        }
    }
}

/// 从quick-xml错误转换
impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        Error::Xml(err.to_string())
    }
}

/// 从zip错误转换
impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::Zip(err.to_string())
    }
}

/// 错误处理工具函数
pub trait ResultExt<T> {
    /// 添加上下文信息
    fn context(self, context: &str) -> Result<T>;

    /// 添加带格式的上下文信息
    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: Into<String>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, context: &str) -> Result<T> {
        self.map_err(|e| Error::Other(format!("{}: {}", context, e)))
    }

    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: Into<String>,
    {
        self.map_err(|e| Error::Other(format!("{}: {}", f().into(), e)))
    }
}

/// 错误处理宏
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !($cond) {
            return Err($err.into());
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            return Err(Error::Other(format!($fmt, $($arg)*)).into());
        }
    };
}

/// 断言宏
#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $err:expr) => {
        if $left != $right {
            return Err($err.into());
        }
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:tt)*) => {
        if $left != $right {
            return Err(Error::Other(format!($fmt, $($arg)*)).into());
        }
    };
}
