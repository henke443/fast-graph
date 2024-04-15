#[cfg(feature = "serde")]
extern crate serde;

use slotmap::KeyData;
use std::collections::HashMap;

use fast_graph::GraphInterface;
use fast_graph::NodeID;

use fast_graph::Graph;

#[test]
fn test_graph_basics() {
    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    struct CategoryNode {
        category: &'static str,
        nodes: Vec<NodeID>,
    }

    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    enum NodeData {
        CategoryNode(CategoryNode),
        String(String),
        HashMap(HashMap<String, String>),
        Int(i32),
        Int64(i64),
    }

    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    struct EdgeData(&'static str);

    let edge_data = EdgeData("Henrik");

    let mut graph: Graph<NodeData, EdgeData> = Graph::new();
    let node1 = graph.add_node(NodeData::Int64(123));
    assert_eq!(graph.node(node1).unwrap().id, node1);

    let node2 = graph.add_node(NodeData::String("Hello".into()));
    assert_eq!(graph.node(node2).unwrap().id, node2);

    let edge1 = { graph.add_edge(node1, node2, edge_data) };
    assert_eq!(graph.edge(edge1).unwrap().id, edge1);

    println!("{:#?}", graph);
}

#[test]
fn test_graph_syntax_sugar() {
    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    enum NodeData {
        String(String),
    }
    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    enum EdgeData {
        None,
    }
    impl Default for EdgeData {
        fn default() -> Self {
            EdgeData::None
        }
    }
    let mut graph: Graph<NodeData, EdgeData> = Graph::new();
    let nodes = &[
        NodeData::String("Hello".into()),
        NodeData::String("World".into()),
        NodeData::String("Universe".into()),
        NodeData::String("Galaxy".into()),
    ];

    let node_ids = graph.add_nodes(nodes);
    graph.add_edges(&[
        (node_ids[0], node_ids[1]),
        (node_ids[1], node_ids[2]),
        (node_ids[2], node_ids[3]),
        (node_ids[3], node_ids[0]),
    ]);
    println!("{:#?}", graph);
}
