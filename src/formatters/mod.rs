// formatters/mod.rs
pub trait DocumentFormatter {
    fn format(&self, doc: &Document) -> Result<Vec<u8>>;
}

pub enum Formatter {
    Docx(DocxFormatter),
    Xlsx(XlsxFormatter),
    Pptx(PptxFormatter),
    Pdf(PdfFormatter),
    MD(MarkdownFormatter),
}
