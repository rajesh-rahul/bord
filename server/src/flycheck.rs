use rusqlite::Connection;

pub fn check_statement(conn: &Connection, stmt: &str) -> Result<(), String> {
    match conn.prepare(stmt) {
        Err(err) => Err(err.to_string()),
        _ => Ok(()),
    }
}
