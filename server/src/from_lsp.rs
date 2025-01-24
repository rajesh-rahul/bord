use anyhow::format_err;
use line_index::{LineIndex, TextRange, TextSize, WideLineCol};
use tower_lsp::lsp_types as lsp;

/// Convert (line, column) for a particular text document into an offset value from the start
/// of the text document.
///
/// Source: rust-analyzer
pub(crate) fn offset(line_index: &LineIndex, line: u32, col: u32) -> anyhow::Result<TextSize> {
    let wide_line_col = WideLineCol { line, col };
    let line_col = line_index
        .to_utf8(line_index::WideEncoding::Utf16, wide_line_col)
        .ok_or_else(|| format_err!("Invalid wide col offset"))?;

    line_index.offset(line_col).ok_or_else(|| {
        format_err!(
            "Invalid offset {line_col:?} (line index length: {:?})",
            line_index.len()
        )
    })
}

/// Like `offset` but there is a start offset and end offset.
///
/// Source: rust-analyzer
pub fn text_range(line_index: &LineIndex, range: lsp::Range) -> anyhow::Result<TextRange> {
    let start = offset(line_index, range.start.line, range.start.character)?;
    let end = offset(line_index, range.end.line, range.end.character)?;

    match end < start {
        true => Err(format_err!("Invalid Range")),
        false => Ok(TextRange::new(start, end)),
    }
}
