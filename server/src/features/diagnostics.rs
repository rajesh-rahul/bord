pub struct ParseError {
    pub range: (usize, usize),
    pub err: String,
}

// pub fn perform_diagnostics(
//     conn: &rusqlite::Connection,
//     doc: &TextDocument,
// ) -> Vec<lsp::Diagnostic> {
//     let mut diagnostics = Vec::new();

//     // Add parse errors
//     for err in doc.cst.errors() {
//         let start_pos = doc.line_index.try_line_col(err.range.0.into()).unwrap();
//         let end_pos = doc.line_index.try_line_col((err.range.1).into()).unwrap();

//         let start = lsp::Position {
//             line: start_pos.line,
//             character: start_pos.col,
//         };
//         let end = lsp::Position {
//             line: end_pos.line,
//             character: end_pos.col + 1,
//         };

//         diagnostics.push(lsp::Diagnostic {
//             range: lsp::Range { start, end },
//             severity: Some(lsp::DiagnosticSeverity::ERROR),
//             message: err.to_string(),
//             source: Some("bordsql".into()),
//             ..Default::default()
//         });
//     }

//     // If stmt have no parse errors let's do a flycheck
//     for FlycheckError {
//         range: (start, end),
//         err,
//     } in doc.flycheck_errors.iter().flatten()
//     {
//         let start_pos = doc
//             .line_index
//             .try_line_col((*start).try_into().unwrap())
//             .unwrap();
//         let end_pos = doc
//             .line_index
//             .try_line_col((*end).try_into().unwrap())
//             .unwrap();

//         let start = lsp::Position {
//             line: start_pos.line,
//             character: start_pos.col,
//         };
//         let end = lsp::Position {
//             line: end_pos.line,
//             character: end_pos.col + 1,
//         };

//         diagnostics.push(lsp::Diagnostic {
//             range: lsp::Range { start, end },
//             severity: Some(lsp::DiagnosticSeverity::ERROR),
//             message: err.clone(),
//             source: Some("bordsql".into()),
//             ..Default::default()
//         });
//     }

//     diagnostics
// }
