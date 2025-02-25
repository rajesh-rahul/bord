use bord_sqlite3_parser::{batch, incr, parse, parse_events_and_tokens, slot, CstTrait};
// use pretty_assertions::assert_eq;
#[test]
fn test_parse() {
    let cst: incr::IncrSqlCst = parse("CREATE TABLE f");

    println!("{}", cst);
}

#[test]
fn ensure_both_cst_kinds_match() {
    let input_sql = include_str!("../../test_schema.sql");

    let events_and_tokens = parse_events_and_tokens(input_sql);

    let incr_cst: incr::IncrSqlCst = parse(input_sql);
    assert!(incr_cst.errors().next().is_none());
    assert_eq!(events_and_tokens, incr_cst.to_events_and_tokens());

    let batch_cst: batch::SqlCst = parse(input_sql);
    assert!(batch_cst.errors().next().is_none());
    assert_eq!(events_and_tokens, batch_cst.to_events_and_tokens());

    let slot_cst: slot::SlotIncrSqlCst = parse(input_sql);
    assert!(slot_cst.errors().next().is_none());
    assert_eq!(events_and_tokens, slot_cst.to_events_and_tokens());

    let incr_cst_display = incr_cst.to_string();
    let batch_cst_display = batch_cst.to_string();
    let slot_cst_display = slot_cst.to_string();

    assert_eq!(incr_cst_display, batch_cst_display);
    assert_eq!(incr_cst_display, slot_cst_display);
}
