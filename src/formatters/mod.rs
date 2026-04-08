// formatters/mod.rs

use crate::{Document, Error};

pub mod docx;

pub trait DocumentFormatter {
    fn format(&self, doc: &Document) -> Result<Vec<u8>, Error>;
}

pub enum Formatter {
    // Docx(DocxFormatter),
    // Xlsx(XlsxFormatter),
    // Pptx(PptxFormatter),
    // Pdf(PdfFormatter),
    // MD(MarkdownFormatter),
}
