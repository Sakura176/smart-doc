/// Open XML 使用 Dxa (Twentieths of a Point) 作为主要长度单位
/// 1 inch = 1440 dxa, 1 cm ≈ 567 dxa, 1 pt = 20 dxa
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Dxa(i64),     // 二十分之一点 (Word 默认)
    Emu(i64),     // English Metric Units (DrawingML 用)
    Percent(u16), // 百分比 (0-100)
    Auto,         // 自动
}

impl Length {
    pub fn from_cm(cm: f64) -> Self {
        Length::Dxa((cm * 567.0) as i64)
    }
    pub fn from_mm(mm: f64) -> Self {
        Length::Dxa((mm * 56.7) as i64)
    }
    pub fn from_pt(pt: f64) -> Self {
        Length::Dxa((pt * 20.0) as i64)
    }
    pub fn from_px(px: u32, dpi: u32) -> Self {
        Length::Emu((px as i64 * 914400) / dpi as i64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Hex(String),       // "FF0000" (6位十六进制)
    Theme(ThemeColor), // 主题颜色引用
    Auto,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThemeColor {
    Dark1,
    Light1,
    Dark2,
    Light2,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Hyperlink,
    FollowedHyperlink,
}

#[derive(Debug, Clone)]
pub struct FontDef {
    pub ascii: String,             // ASCII 字体
    pub h_ansi: Option<String>,    // 高 ANSI 字体
    pub east_asia: Option<String>, // 东亚字体 (中文/日文)
    pub hint: Option<FontHint>,
}

#[derive(Debug, Clone)]
pub enum FontHint {
    Default,
    Ansi,
    EastAsia,
    ComplexScript,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

impl Alignment {
    pub fn to_xml_attr(&self) -> &'static str {
        match self {
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
            Alignment::Justify => "both",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_from_px() {
        assert_eq!(Length::from_px(10, 96), Length::Emu(10 * 914400 / 96));
        assert_eq!(Length::from_px(10, 192), Length::Emu(10 * 914400 / 192));
    }
}
