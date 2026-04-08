use quick_xml::{
    Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
use std::{collections::HashMap, io::Write};

use crate::error::Result;

/// 流式 XML 写入器
///
/// 封装 quick-xml 的底层 API，提供面向 Open XML 的高层接口：
/// - 自动缩进（由 quick-xml 的 new_with_indent 处理）
/// - 属性延迟写入（先收集到 BytesStart，遇到 text/child/end 时统一写出）
/// - 文本自动转义
/// - 标签栈管理，确保正确闭合
pub struct XmlWriter<W: Write> {
    inner: Writer<W>,
    namespace_map: HashMap<String, String>,
    tag_stack: Vec<String>,
    pending: Option<BytesStart<'static>>,
}

impl<W: Write> XmlWriter<W> {
    /// 创建新的 XmlWriter，底层使用 quick-xml 的自动缩进功能
    pub fn new(inner: W) -> Self {
        Self {
            inner: Writer::new_with_indent(inner, b' ', 2),
            namespace_map: HashMap::new(),
            tag_stack: Vec::new(),
            pending: None,
        }
    }

    /// 注册命名空间前缀（供后续使用）
    pub fn namespace(&mut self, prefix: &str, uri: &str) -> Result<()> {
        self.namespace_map
            .insert(prefix.to_string(), uri.to_string());
        Ok(())
    }

    /// 将 pending 的起始标签写入输出
    ///
    /// 调用时机：在写入文本、子元素、自闭合标签或结束标签之前
    fn flush_pending(&mut self) -> Result<()> {
        if let Some(start) = self.pending.take() {
            self.inner.write_event(Event::Start(start))?;
        }
        Ok(())
    }

    /// 开始一个新元素
    ///
    /// 注意：此时不会立即写入 XML，而是将标签放入 pending 状态。
    /// 属性通过 `attr()` 添加，实际写入延迟到 `text()`/`element()`/`end_element()` 调用时。
    ///
    /// # 示例
    /// ```ignore
    /// writer.start_element("w:p")?;
    /// writer.attr("w:rsidR", "0001")?;
    /// writer.text("Hello")?; // 此时才写入 <w:p w:rsidR="0001">Hello
    /// writer.end_element()?; // 写入 </w:p>
    /// ```
    pub fn start_element(&mut self, name: &str) -> Result<()> {
        self.flush_pending()?;
        self.tag_stack.push(name.to_string());
        self.pending = Some(BytesStart::new(name).into_owned());
        Ok(())
    }

    /// 为当前 pending 的元素添加属性
    ///
    /// 必须在 `start_element()` 之后、`end_element()` 之前调用。
    /// 属性会暂存到 pending 的 BytesStart 中，延迟写出。
    pub fn attr(&mut self, name: &str, value: &str) -> Result<()> {
        let pending = self.pending.as_mut().ok_or_else(|| {
            crate::error::Error::Xml("Cannot add attribute: no open element".into())
        })?;
        pending.push_attribute((name, value));
        Ok(())
    }

    /// 写入文本内容（自动转义 &, <, >, ", '）
    ///
    /// 如果当前有 pending 的起始标签，会先将其写出，再写入文本。
    pub fn text(&mut self, content: &str) -> Result<()> {
        self.flush_pending()?;
        let text = BytesText::new(content);
        self.inner.write_event(Event::Text(text))?;
        Ok(())
    }

    /// 写入自闭合元素（如 `<w:br/>`）
    ///
    /// 如果当前有 pending 的起始标签，会先将其写出。
    pub fn empty_element(&mut self, name: &str) -> Result<()> {
        self.flush_pending()?;
        let start = BytesStart::new(name);
        self.inner.write_event(Event::Empty(start))?;
        Ok(())
    }

    /// 闭合当前元素
    ///
    /// 两种情况：
    /// 1. 如果当前元素仍在 pending（没有子内容），写出为自闭合 `<tag attr="..."/>`
    /// 2. 如果 pending 已 flush（有子内容），正常写出 `</tag>`
    pub fn end_element(&mut self) -> Result<()> {
        if let Some(start) = self.pending.take() {
            let name_bytes = start.name().as_ref().to_vec();
            self.inner.write_event(Event::Empty(start))?;
            let name_str = String::from_utf8_lossy(&name_bytes);
            self.inner
                .write_event(Event::End(BytesEnd::new(name_str.as_ref())))?;
        } else {
            let name = self.tag_stack.pop().ok_or_else(|| {
                crate::error::Error::Xml("Cannot close element: no open tag".into())
            })?;
            self.inner.write_event(Event::End(BytesEnd::new(&name)))?;
        }
        Ok(())
    }

    /// 闭包式安全嵌套
    ///
    /// 自动调用 start_element 和 end_element，确保标签必定闭合。
    /// 即使闭包内 panic 或提前 return，也能保证 XML 结构完整。
    pub fn element<F>(&mut self, name: &str, f: F) -> Result<()>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        self.start_element(name)?;
        f(self)?;
        self.end_element()
    }

    /// 完成写入，返回底层 Writer
    ///
    /// 会检查是否所有标签都已正确闭合。
    pub fn finish(mut self) -> Result<W> {
        self.flush_pending()?;
        if !self.tag_stack.is_empty() {
            return Err(crate::error::Error::Xml(format!(
                "Unclosed tags: {:?}",
                self.tag_stack
            )));
        }
        self.inner.get_mut().flush()?;
        Ok(self.inner.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_element_with_attr() {
        let mut writer = XmlWriter::new(Vec::new());
        writer.start_element("w:p").unwrap();
        writer.attr("w:rsidR", "0001").unwrap();
        writer.text("Hello").unwrap();
        writer.end_element().unwrap();
        let output = String::from_utf8(writer.finish().unwrap()).unwrap();
        assert!(output.contains("w:p"));
        assert!(output.contains("w:rsidR"));
        assert!(output.contains("Hello"));
    }

    #[test]
    fn test_text_escaping() {
        let mut writer = XmlWriter::new(Vec::new());
        writer.element("w:t", |w| w.text("A & B < C")).unwrap();
        let output = String::from_utf8(writer.finish().unwrap()).unwrap();
        assert!(output.contains("A &amp; B &lt; C"));
    }

    #[test]
    fn test_nested_elements() {
        let mut writer = XmlWriter::new(Vec::new());
        writer
            .element("w:body", |w| {
                w.element("w:p", |w| w.text("Hello")).unwrap();
                w.element("w:p", |w| w.text("World")).unwrap();
                Ok(())
            })
            .unwrap();
        let output = String::from_utf8(writer.finish().unwrap()).unwrap();
        assert!(output.contains("w:body"));
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
    }

    #[test]
    fn test_empty_element() {
        let mut writer = XmlWriter::new(Vec::new());
        writer.empty_element("w:br").unwrap();
        let output = String::from_utf8(writer.finish().unwrap()).unwrap();
        assert!(output.contains("w:br"));
    }

    #[test]
    #[should_panic(expected = "no open element")]
    fn test_attr_without_start_element() {
        let mut writer = XmlWriter::new(Vec::new());
        writer.attr("w:val", "center").unwrap();
    }
}
