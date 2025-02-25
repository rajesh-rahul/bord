use anyhow::format_err;
use async_lsp::lsp_types as lsp;
use bord_sqlite3_parser::{incr, CstNodeTrait};
use line_index::{LineIndex, TextRange, TextSize, WideLineCol};

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

pub fn lsp_range(line_index: &LineIndex, range: TextRange) -> anyhow::Result<lsp::Range> {
    let start_pos = line_index
        .try_line_col(range.start())
        .ok_or_else(|| format_err!("range start cannot be converted to LineCol"))?;

    let end_pos = line_index
        .try_line_col(range.end())
        .ok_or_else(|| format_err!("range end cannot be converted to LineCol"))?;

    let start = lsp::Position {
        line: start_pos.line,
        character: start_pos.col,
    };

    let end = lsp::Position {
        line: end_pos.line,
        character: end_pos.col + 1,
    };

    Ok(lsp::Range { start, end })
}

pub fn node_lsp_range<'a>(
    line_index: &LineIndex,
    node: &incr::IncrCstNode<'a>,
) -> anyhow::Result<lsp::Range> {
    let start: u32 = node.start_pos_skip_trivia().try_into()?;
    let end: u32 = node.end_pos_skip_trivia().try_into()?;

    lsp_range(line_index, TextRange::new(start.into(), end.into()))
}
