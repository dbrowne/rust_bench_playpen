use  rust_bench_playpen::db_functions::{establish_connection,persist_item};

#[test]
fn test_establish_connection() {
    let conn = establish_connection();
    assert_eq!(conn.is_ok(), true);
}


#[test]
fn test_insert_item(){
    let  id:i64 = 123456789;
    let event = "test_insert_item";

    let mut conn = establish_connection().unwrap();
    let rslt = persist_item(&mut conn, &id, &event);
    assert_eq!(rslt.is_ok(), true);

}