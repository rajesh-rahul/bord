#[derive(Clone)]
pub struct SqliteVersion(pub [u16; 3]);

impl SqliteVersion {
    /// SQLite added support for underscores in numeric literals in version 3.46.0.
    /// https://www.sqlite.org/lang_expr.html
    pub fn underscore_in_numerics(&self) -> bool {
        self.0 >= [3, 46, 0]
    }
}
