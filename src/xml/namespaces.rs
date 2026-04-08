/// WordprocessingML (Word)
pub const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

/// DrawingML - WordProcessing (图片/图形)
pub const WP: &str = "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing";

/// DrawingML 主命名空间 (共享于 Word/Excel/PPT)
pub const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";

/// Relationships
pub const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// Markup Compatibility
pub const MC: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";

/// SpreadsheetML (Excel)
pub const X: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";

/// PresentationML (PowerPoint)
pub const P: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";

/// 关系类型常量
pub mod relationship_types {
    pub const OFFICE_DOCUMENT: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
    pub const STYLES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
    pub const IMAGE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
    pub const THEME: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
}
