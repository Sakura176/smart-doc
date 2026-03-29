//! 智能文档生成工具核心库
//!
//! 这是一个基于 Rust 的个人学习项目，提供文档生成的核心功能。

pub mod cli;
pub mod core;
pub mod error;

/// 重新导出常用类型
pub use crate::core::document::Document;
pub use crate::core::generator::DocumentGenerator;
pub use crate::error::{Error, Result};

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 初始化库
///
/// # 示例
/// ```
/// use smart_doc_core::init;
///
/// init();
/// ```
pub fn init() {
    // 可以在这里初始化全局状态
    // 目前只是一个占位函数
}

/// 获取库信息
///
/// # 返回
/// 包含版本和特性的字符串
pub fn info() -> String {
    format!(
        "smart-doc-core v{}\n\
         Authors: {}\n\
         Repository: {}\n\
         Description: {}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_REPOSITORY"),
        env!("CARGO_PKG_DESCRIPTION")
    )
}
