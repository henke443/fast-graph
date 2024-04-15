use fast_graph::{algorithms::IterDepthFirst, Graph, GraphInterface, NodeID};
use hashbrown::HashSet;
use rand::Rng;
use rand::seq::SliceRandom;

fn random_nodes(count: usize, breadth: usize) -> Vec<usize> {
    return (0..count).collect();
}

fn random_edges(count: usize, breadth: usize) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();
    let mut random_edges: HashSet<(usize, usize)> = HashSet::new();

    for _ in 0..breadth {
        let mut from = (0..count).collect::<Vec<usize>>();
        from.shuffle(&mut rng);
        let mut to = (0..count).collect::<Vec<usize>>();
        to.shuffle(&mut rng);
        
        to.iter().zip(from.iter()).for_each(|(t, f)| {
            if *t != *f {
                random_edges.insert((*f, *t));
            }
        });
    }
    
    random_edges.into_iter().collect()
}

fn random_petgraph(nodes: (Vec<usize>, Vec<(usize, usize)>)) -> (petgraph::stable_graph::StableDiGraph<usize, ()>, petgraph::stable_graph::NodeIndex) {
    let mut graph = petgraph::stable_graph::StableDiGraph::new();

    let mut node_ids = vec![];
    for node in nodes.0 {
        let n = graph.add_node(node);
        node_ids.push(n);
    }

    for (n1, n2) in nodes.1 {
        graph.add_edge(node_ids[n1], node_ids[n2], ());
    }

    (graph, node_ids[0])
}

fn random_fastgraph(nodes: (Vec<usize>, Vec<(usize, usize)>)) -> (Graph<usize, ()>, NodeID) {
    let mut graph = Graph::new();

    let mut node_ids = vec![];
    for node in nodes.0 {
        let n = graph.add_node(node);
        node_ids.push(n);
    }

    for (n1, n2) in nodes.1 {
        graph.add_edge(node_ids[n1], node_ids[n2], ());
    }

    (graph, node_ids[0])
}



#[test]
pub fn test_fastgraph_dfs_equals_petgraph_random() {
    let random_nodes = random_nodes(100, 10);
    let random_edges = random_edges(100, 10);
    println!("random_nodes {:?}", random_nodes);
    println!("random_edges {:?}", random_edges);
    let petgraph = random_petgraph((random_nodes.clone(), random_edges.clone()));
    let fastgraph = random_fastgraph((random_nodes, random_edges));

    let mut visited_fastgraph = vec![];
    let mut visited_petgraph = vec![];

    let mut dfs = fastgraph.0.iter_depth_first(fastgraph.1);
    while let Some(v) = dfs.next() {
        visited_fastgraph.push(fastgraph.0.node(v).unwrap().data);
    }

    let mut dfs = petgraph::visit::Dfs::new(&petgraph.0, petgraph.1);
    while let Some(nx) = dfs.next(&petgraph.0) {
        visited_petgraph.push(petgraph.0[nx]);
    }

    println!("fastgraph: {:?}", visited_fastgraph);
    println!("petgraph: {:?}", visited_petgraph);

    assert_eq!(visited_fastgraph.len(), visited_petgraph.len());
    assert_eq!(visited_fastgraph, visited_petgraph);

}

#[test]
pub fn test_fastgraph_dfs_equals_petgraph() {
    let mut graph: Graph<i32, ()> = Graph::new();

    let mut current_node = graph.add_node(1);
    let root = current_node;

    for i in 1..=10 {
        let n = graph.add_node(i+100);
        let n2 = graph.add_node(i+200);
        let n3 = graph.add_node(i+300);
        graph.add_edge(current_node, n, ());
        graph.add_edge(current_node, n2, ());
        graph.add_edge(n2, n3, ());
        current_node = n;
    }

    let mut visited_fastgraph = vec![];
    
    let mut dfs = graph.iter_depth_first(root);
    while let Some(v) = dfs.next() {
        visited_fastgraph.push(v);
    }

    let mut pgraph = petgraph::stable_graph::StableGraph::new();

    let mut current_node = pgraph.add_node(1);

    let root = current_node;
    for i in 1..=10 {
        let n = pgraph.add_node(i+100);
        let n2 = pgraph.add_node(i+200);
        let n3 = pgraph.add_node(i+300);
        pgraph.add_edge(current_node, n, ());
        pgraph.add_edge(current_node, n2, ());
        pgraph.add_edge(n2, n3, ());
        current_node = n;
    }

    let mut visited_petgraph = vec![];
    let mut dfs = petgraph::visit::Dfs::new(&pgraph, root);
    while let Some(nx) = dfs.next(&pgraph) {
        visited_petgraph.push(pgraph[nx]);
    }

    assert_eq!(visited_fastgraph.len(), visited_petgraph.len());

    for i in 0..visited_fastgraph.len() {
        //println!("fastgraph: {}, petgraph: {}", graph.node(visited_fastgraph[i]).unwrap().data, visited_petgraph[i]);
        assert_eq!(graph.node(visited_fastgraph[i]).unwrap().data, visited_petgraph[i])
    }
    
}
