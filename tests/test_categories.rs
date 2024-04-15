
#[cfg(test)]
#[cfg(feature = "categories")]
mod test_categories {
    use fast_graph::categories::*;
    use fast_graph::GraphInterface;


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
        assert_eq!(graph.nodes.len() - 1, 5); // Slotmap has one extra empty slot in the start.

        assert_eq!(graph.category("Category 1").unwrap().connections.len(), 2);
        assert_eq!(graph.category("Category 2").unwrap().connections.len(), 1);

        assert_eq!(graph.edges.len(), 3);
        assert_eq!(graph.nodes.len() - 1, 5); // Slotmap has one extra empty slot in the start.
        assert_eq!(
            graph.category_by_id(category1).unwrap().data,
            NodeData::CategoryName("Category 1".into())
        );

        println!("{:#?}", graph);
    }
}