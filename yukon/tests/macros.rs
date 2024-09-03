use yukon_macros::sql;

#[test]
fn test_sql_macro() {
    let query = sql!("SELECT * FROM users");
    assert_eq!(query.sql, "SELECT * FROM users");

    let query = sql!(r#"SELECT * FROM users"#);
    assert_eq!(query.sql, "SELECT * FROM users");
}
