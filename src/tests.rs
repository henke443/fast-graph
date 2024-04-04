#[cfg(feature = "serde")]
extern crate serde;

use slotmap::KeyData;
use std::collections::HashMap;

use crate::GraphInterface;
use crate::NodeID;

use crate::categories::*;
use crate::Graph;

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

    impl Graph<NodeData, EdgeData> {
        pub fn new_category_node(&mut self, category: &'static str, nodes: Vec<NodeID>) -> NodeID {
            let category_node = CategoryNode { category, nodes };
            let node = NodeData::CategoryNode(category_node);
            let node = self.add_node(node);
            node
        }

        pub fn all_categories(&self) -> Vec<&CategoryNode> {
            self.nodes
                .iter()
                .filter_map(|node| match &node.1.data {
                    NodeData::CategoryNode(category_node) => Some(category_node),
                    _ => None,
                })
                .collect()
        }
    }

    let edge_data = EdgeData("Henrik");

    let mut graph: Graph<NodeData, EdgeData> = Graph::new();
    let node1 = graph.add_node(NodeData::Int64(123));
    assert_eq!(graph.node(node1).unwrap().id, node1);

    let node2 = graph.add_node(NodeData::String("Hello".into()));
    assert_eq!(graph.node(node2).unwrap().id, node2);
    
    let edge1 ={ graph.add_edge(node1, node2, edge_data) };
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

#[test]
pub fn test_graph_categories() {
    let mut graph: CategorizedGraph<NodeData, ()> = CategorizedGraph::new();

    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    enum NodeData {
        String(String),
        CategoryName(String),
        #[default]
        None,
    }

    let node1 = graph.add_node(NodeData::String("Node 1".into()));
    let node2 = graph.add_node(NodeData::String("Node 2".into()));
    let node3 = graph.add_node(NodeData::String("Node 3".into()));

    let category1 = graph
        .create_category(
            "Category 1",
            vec![node1, node2],
            NodeData::CategoryName("Category 1".into()),
        )
        .unwrap();
    let category2 = graph.add_to_category("Category 2", vec![node3]);

    println!("Categories: {:#?}", graph.categories);

    assert_eq!(graph.categories.len(), 2);
    assert_eq!(graph.nodes.len()-1, 5); // Slotmap has one extra empty slot in the start.

    assert_eq!(graph.category("Category 1").unwrap().connections.len(), 2);
    assert_eq!(graph.category("Category 2").unwrap().connections.len(), 1);

    assert_eq!(graph.edges.len(), 3);
    assert_eq!(graph.nodes.len()-1, 5); // Slotmap has one extra empty slot in the start.
    assert_eq!(graph.category_by_id(category1).unwrap().data, NodeData::CategoryName("Category 1".into()));

    println!("{:#?}", graph);
}
