use async_lsp::lsp_types as lsp;

use crate::{flycheck, text_document::TextDocument};

pub fn perform_diagnostics(
    conn: &rusqlite::Connection,
    doc: &TextDocument,
) -> Vec<lsp::Diagnostic> {
    let mut diagnostics = Vec::new();

    // Add parse errors
    for err in doc.cst.errors() {
        let start_pos = doc.line_index.try_line_col(err.range.0.into()).unwrap();
        let end_pos = doc.line_index.try_line_col((err.range.1).into()).unwrap();

        let start = lsp::Position {
            line: start_pos.line,
            character: start_pos.col,
        };
        let end = lsp::Position {
            line: end_pos.line,
            character: end_pos.col + 1,
        };

        diagnostics.push(lsp::Diagnostic {
            range: lsp::Range { start, end },
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            message: err.to_string(),
            source: Some("bordsql".into()),
            ..Default::default()
        });
    }

    // If stmt have no parse errors let's do a flycheck
    for stmt in doc.cst.statements().filter(|it| !it.has_errors()) {
        if let Err(err) = flycheck::check_statement(conn, &stmt.to_string()) {
            let start_pos = stmt.start_pos_skip_trivia().unwrap_or(0);
            let start_pos = doc.line_index.try_line_col(start_pos.into()).unwrap();

            let end_pos = stmt.end_pos_skip_trivia().unwrap_or(0);
            let end_pos = doc.line_index.try_line_col(end_pos.into()).unwrap();

            let start = lsp::Position {
                line: start_pos.line,
                character: start_pos.col,
            };
            let end = lsp::Position {
                line: end_pos.line,
                character: end_pos.col + 1,
            };

            diagnostics.push(lsp::Diagnostic {
                range: lsp::Range { start, end },
                severity: Some(lsp::DiagnosticSeverity::ERROR),
                message: err,
                source: Some("bordsql".into()),
                ..Default::default()
            });
        }
    }

    diagnostics
}
