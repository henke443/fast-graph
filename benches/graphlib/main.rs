// #[macro_use]
// extern crate criterion;

// use criterion::measurement::WallTime;
// use criterion::{BenchmarkGroup, Criterion};
// use fast_graph::*;
// use fast_graph::algorithms::IterDepthFirst;

// mod bench_graphlib;

// use bench_graphlib::graphlib_bench_iterators;

// // use `cargo bench --features sbench` for benching with GraphCapacity of 10_000_000

// // includes benches for :
// // 1. new() -> Graph<T>
// // 2. with_capacity(capacity: usize) -> Graph<T>
// // fn bench_create(c: &mut Criterion) {
// //     c.bench_function("new", |b| b.iter(Graph::<usize>::new));

// //     macro_rules! with_capacity {
// //         ($str: tt ,$x: expr) => {
// //             c.bench_function($str, |b| b.iter(|| Graph::<usize>::with_capacity($x)));
// //         };
// //     }
// //     with_capacity!("with_capacity_10", 10);
// //     with_capacity!("with_capacity_100", 100);
// //     with_capacity!("with_capacity_500", 500);
// //     with_capacity!("with_capacity_1000", 1000);
// //     #[cfg(feature = "sbench")]
// //     with_capacity!("with_capacity_m", 10_000_000);
// // }

// // includes benches for :
// // 1. dfs(&self) -> Dfs<T>
// // 2. bfs(&self) -> Bfs<T>
// // 3. topo(&self) -> Topo<T>
// // 4. vertices(&self) -> VertexIter
// // 5. roots(&self) -> VertexIter

// // fn bench_create(c: &mut Criterion) {
// //     c.bench_function("new", |b| b.iter(Graph::<usize>::new));

// //     macro_rules! with_capacity {
// //         ($str: tt ,$x: expr) => {
// //             c.bench_function($str, |b| b.iter(|| Graph::<usize>::with_capacity($x)));
// //         };
// //     }
// //     with_capacity!("with_capacity_10", 10);
// //     with_capacity!("with_capacity_100", 100);
// //     with_capacity!("with_capacity_500", 500);
// //     with_capacity!("with_capacity_1000", 1000);
// //     #[cfg(feature = "sbench")]
// //     with_capacity!("with_capacity_m", 10_000_000);
// // }


// fn bench_fastgraph_dfs
// fn fastgraph_bench_iterators(c: &mut BenchmarkGroup<'static, WallTime>) {
//     struct NodeData;
//     struct EdgeData;

//     macro_rules! dfs {
//         ($str: tt ,$x: expr) => {
//             c.bench_function($str, |b| {
//                 let mut graph: Graph<NodeData, EdgeData> = Graph::new();
//                 let mut vertices = vec![];

//                 let mut root: Option<NodeID>;
//                 let mut v1 = graph.add_node(NodeData);
//                 root = Some(v1);

//                 for i in 1..=$x {
//                     let v2 = graph.add_node(NodeData);
//                     graph.add_edge(v1, v2, EdgeData);
//                     v1 = v2;
//                 }
//                 b.iter(|| {
//                     for v in graph.iter_depth_first(v1) {
//                         vertices.push(v);
//                     }
//                 })
//             });
//         };
//     }
//     dfs!("fastgraph_dfs_10", 10);
//     // dfs!("dfs_100", 100);
//     // dfs!("dfs_500", 500);
//     dfs!("fastgraph_dfs_1000", 1000);
// }

// fn compare_with_graphlib(c: &mut Criterion) {
//     let mut group = c.benchmark_group("DFS Comparison with Graphlib");
//     fastgraph_bench_iterators(&mut group);
//     graphlib_bench_iterators(&mut group);
// }

// criterion_group!(
//     benches,
//     compare_with_graphlib
// );

// criterion_main!(benches);

//     // #[cfg(feature = "sbench")]
//     // dfs!("dfs_m", 10_000_000);

//     // macro_rules! bfs {
//     //     ($str: tt ,$x: expr) => {
//     //         c.bench_function($str, |b| {
//     //             let mut graph: Graph<usize> = Graph::new();
//     //             let mut vertices = vec![];

//     //             let mut v1 = graph.add_vertex(0);

//     //             for i in 1..=$x {
//     //                 let v2 = graph.add_vertex(i);
//     //                 graph.add_edge(&v1, &v2);
//     //                 v1 = v2.clone();
//     //             }

//     //             b.iter(|| {
//     //                 for v in graph.bfs() {
//     //                     vertices.push(v);
//     //                 }
//     //             })
//     //         });
//     //     };
//     // }
//     // bfs!("bfs_10", 10);
//     // bfs!("bfs_100", 100);
//     // bfs!("bfs_500", 500);
//     // bfs!("bfs_1000", 1000);
//     // #[cfg(feature = "sbench")]
//     // bfs!("bfs_m", 10_000_000);

//     // macro_rules! topo {
//     //     ($str: tt ,$x: expr) => {
//     //         c.bench_function($str, |b| {
//     //             let mut graph: Graph<usize> = Graph::new();
//     //             let mut vertices = vec![];

//     //             let mut v1 = graph.add_vertex(0);

//     //             for i in 1..=$x {
//     //                 let v2 = graph.add_vertex(i);
//     //                 graph.add_edge(&v1, &v2);
//     //                 v1 = v2.clone();
//     //             }
//     //             b.iter(|| {
//     //                 for v in graph.topo() {
//     //                     vertices.push(v);
//     //                 }
//     //             })
//     //         });
//     //     };
//     // }
//     // topo!("topo_10", 10);
//     // topo!("topo_100", 100);
//     // topo!("topo_500", 500);
//     // topo!("topo_1000", 1000);
//     // #[cfg(feature = "sbench")]
//     // topo!("topo_m", 10_000_000);

//     // macro_rules! vertices {
//     //     ($str: tt ,$x: expr) => {
//     //         c.bench_function($str, |b| {
//     //             let mut graph: Graph<usize> = Graph::new();
//     //             let mut vertices = vec![];

//     //             for i in 1..=$x {
//     //                 graph.add_vertex(i);
//     //             }

//     //             b.iter(|| {
//     //                 for v in graph.vertices() {
//     //                     vertices.push(v);
//     //                 }
//     //             })
//     //         });
//     //     };
//     // }
//     // vertices!("vertices_10", 10);
//     // vertices!("vertices_100", 100);
//     // vertices!("vertices_500", 500);
//     // vertices!("vertices_1000", 1000);
//     // #[cfg(feature = "sbench")]
//     // vertices!("vertices_m", 10_000_000);

//     // macro_rules! roots {
//     //     ($str: tt ,$x: expr) => {
//     //         c.bench_function($str, |b| {
//     //             let mut graph: Graph<usize> = Graph::new();
//     //             let mut roots = vec![];

//     //             let mut v1 = graph.add_vertex(0);

//     //             for i in 1..=$x {
//     //                 let v2 = graph.add_vertex(i);
//     //                 graph.add_edge(&v1, &v2);
//     //                 v1 = v2.clone();
//     //             }

//     //             b.iter(|| {
//     //                 for v in graph.roots() {
//     //                     roots.push(v);
//     //                 }
//     //             })
//     //         });
//     //     };
//     // }

//     // roots!("roots_10", 10);
//     // roots!("roots_100", 100);
//     // roots!("roots_500", 500);
//     // roots!("roots_1000", 1000);
//     // #[cfg(feature = "sbench")]
//     // roots!("roots_m", 10_000_000);
