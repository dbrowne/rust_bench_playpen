use rust_bench_playpen::db_functions::establish_connection;
use rust_bench_playpen::db_models::Item;

#[test]
fn test_establish_connection() {
    let conn = establish_connection();
    assert_eq!(conn.is_ok(), true);
}


#[test]
fn test_insert_item() {
    let id: i64 = 123456789;
    let event = "test_insert_item";

    let mut conn = establish_connection().unwrap();
    let rslt = Item::persist_item(&mut conn, &id, &event);
    assert_eq!(rslt.is_ok(), true);
}

#[test]
fn test_id_val() {
    let mut conn = establish_connection().unwrap();

    for ctr in 0..10 {
        let rslt = Item::persist_item(&mut conn, &ctr, "test_id_val");
        assert_eq!(rslt.is_ok(), true);
    }

    let val = Item::get_last_id(&mut conn);
      let  _rslt = match val {
          Ok(v) => assert_eq!(v, 9),
          Err(e) => panic!("Error: {}", e),
      };
}








