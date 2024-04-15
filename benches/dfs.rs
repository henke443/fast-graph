use criterion::{criterion_group, criterion_main, measurement::Measurement, Bencher, Criterion};
use fast_graph::{Graph, NodeID, GraphInterface, algorithms::IterDepthFirst};

use graphlib;
use petgraph;


macro_rules! fastgraph_dfs {
    ($g: tt, $str: tt ,$x: expr) => {
        $g.bench_function($str, |b| {
            let mut graph: Graph<(), ()> = Graph::new();

            let mut current_node = graph.add_node(());
            let root = current_node;

            for _i in 1..=$x {
                let n = graph.add_node(());
                let n2 = graph.add_node(());
                let n3 = graph.add_node(());
                graph.add_edge(current_node, n, ());
                graph.add_edge(current_node, n2, ());
                graph.add_edge(n2, n3, ());
                current_node = n;
            }
            b.iter(|| {
                let mut visited = vec![];
                
                for n in graph.iter_depth_first(root) {
                    visited.push(n);
                }
    
                assert_eq!( visited.len(), ($x as usize)*3 + 1);
            });
        });
    };
}

macro_rules! graphlib_dfs {
    ($g: expr, $str: tt ,$x: expr) => {
        $g.bench_function($str, |b| {
            let mut graph: graphlib::Graph<()> = graphlib::Graph::new();

            let mut current_node = graph.add_vertex(());

            for _i in 1..=$x {
                let n = graph.add_vertex(());
                let n2 = graph.add_vertex(());
                let n3 = graph.add_vertex(());
                graph.add_edge(&current_node, &n).unwrap();
                graph.add_edge(&current_node, &n2).unwrap();
                graph.add_edge(&n2, &n3).unwrap();
                current_node = n.clone();
            }
            b.iter(|| {
                let mut vertices = vec![];
                for n in graph.dfs() {
                    vertices.push(n);
                }
                assert_eq!( vertices.len(), ($x as usize)*3 + 1);
            });
        });
    };
}

macro_rules! petgraph_dfs {
    ($g: expr, $str: tt ,$x: expr) => {
        $g.bench_function($str, |b| {
            let mut graph = petgraph::stable_graph::StableDiGraph::new();

            let mut current_node = graph.add_node(());
            let root = current_node;

            for _i in 1..=$x {
                let n = graph.add_node(());
                let n2 = graph.add_node(());
                let n3 = graph.add_node(());
                graph.add_edge(current_node, n, ());
                graph.add_edge(current_node, n2, ());
                graph.add_edge(n2, n3, ());
                current_node = n;
            }
            b.iter(|| {
                let mut vertices = vec![];
                let mut dfs = petgraph::visit::Dfs::new(&graph, root);
                while let Some(nx) = dfs.next(&graph) {
                    vertices.push(graph[nx]);
                }
                assert_eq!( vertices.len(), ($x as usize)*3 + 1);
            });
        });
    };

}


pub fn bench_dfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs_1000");
    fastgraph_dfs!(group, "fastgraph_dfs_1000", 1000);
    graphlib_dfs!(group, "graphlib_dfs_1000", 1000);
    petgraph_dfs!(group, "petgraph_dfs_1000", 1000);
    group.finish();

    let mut group = c.benchmark_group("dfs_10000");
    fastgraph_dfs!(group, "fastgraph_dfs_10000", 10000);
    petgraph_dfs!(group, "petgraph_dfs_10000", 10000);
    group.finish();
}


criterion_group!(
    benches,
    bench_dfs
);

criterion_main!(benches);