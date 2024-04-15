
// use criterion::{measurement::Measurement, Bencher, Criterion};
// use graphlib::*;

// // use `cargo bench --features sbench` for benching with GraphCapacity of 10_000_000

// // includes benches for :
// // 1. new() -> Graph<T>
// // 2. with_capacity(capacity: usize) -> Graph<T>
// fn graphlib_bench_create(c: &mut Criterion) {
//     c.bench_function("new", |b| b.iter(Graph::<usize>::new));

//     macro_rules! with_capacity {
//         ($str: tt ,$x: expr) => {
//             c.bench_function($str, |b| b.iter(|| Graph::<usize>::with_capacity($x)));
//         };
//     }
//     with_capacity!("graphlib_with_capacity_10", 10);
//     with_capacity!("graphlib_with_capacity_100", 100);
//     with_capacity!("graphlib_with_capacity_500", 500);
//     with_capacity!("graphlib_with_capacity_1000", 1000);
//     #[cfg(feature = "sbench")]
//     with_capacity!("graphlib_with_capacity_m", 10_000_000);
// }

