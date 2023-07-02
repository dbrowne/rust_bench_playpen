use  criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_bench_playpen::db_functions::establish_connection;
use rust_bench_playpen::db_models::Item;
use core::time::Duration;

fn loop_test(iter_count:i64){
    let mut conn = establish_connection().unwrap();
    let rslt = match   Item::get_last_id(&mut conn){
        Ok(v) => v,
        Err(_) => 0 as i64
    };

    for ctr in 1 ..= iter_count  {
        let _rslt = match Item::persist_item(&mut conn, &(ctr+rslt), "test_id_val"){
            Ok(v) => v,
            Err(e) => panic!("Error: {}", e),
        };
    }

}

pub  fn  first_benchmark(c: &mut Criterion) {
    let  mut  group = c.benchmark_group("first_benchmark");
    group.measurement_time(Duration::new(15, 0));
    group.significance_level(0.1).sample_size(50);
    group.bench_function("loop_test", |b| b.iter(|| loop_test(black_box(200))));
    group.finish();
}

criterion_group!(benches, first_benchmark);
criterion_main!(benches);