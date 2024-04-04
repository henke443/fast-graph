fast-graph
=========
### A fast, lightweight and extensible implementation of a graph data structure.

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docsrs-badge]][docsrs-url]
![MSRV][msrv-badge]
[![Discord chat][discord-badge]][discord-url]
[![build_status][]](https://github.com/henke443/fast-graph/actions)


 ## Lightweight & fast.
 ⚠️ The version 1.0.0 is a bit misleading and there will be some breaking changes in the coming 1-2 weeks at the very least.

 By default, [SlotMaps](https://docs.rs/slotmap/latest/slotmap/index.html) are used to store the nodes and edges which solves the [ABA problem] while also providing O(1) insertion, deletion and lookup times. Additionally, and optionally,
 [HashBrown](https://docs.rs/hashbrown/latest/hashbrown/index.html) is used instead of [`std::HashMap`] to map category names to ids in the [`CategoryGraph`](https://docs.rs/fast-graph/latest/fast_graph/categories/struct.CategoryGraph.html) struct.

 [ABA problem]: https://en.wikipedia.org/wiki/ABA_problem
 
 ## Extensible & Generic

 The [Graph] is generic over the node and edge data types, which can be any type that implements [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html). There's also traits for making even more customized graph-like data structures if the need arises.

 [`std::HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html

 ## Serde & Specta

 There's optional features to enable [serde] & [specta] support.

 ## Categories

 The [CategoryGraph] struct uses a hash map to map category names ([String](https://doc.rust-lang.org/std/string/index.html)) to a category node ([NodeID]) (where the node's edges are the nodes belonging to the category).
 There's also some useful extra functions to query categories and their nodes, and a [Categorized] trait that can be implemented for a custom struct if needed.

 In other words a simple extension to the graph that allows for efficient and easy grouping of nodes by strings.

 # Structure
 [Node] - Struct representing a node in the graph. Contains a [NodeID] which is a key to the node in the slotmap, which has a generic data field and a list of edges.

 [Edge] - Struct representing an edge in the graph. Contains an [EdgeID] which is a key to the edge in the slotmap, and two [NodeID]s which are the nodes the edge connects (from & to). An edge can also have "data", which could be anything or nothing; for example the weight of the connection or a struct or enum representing something else.

 [GraphWriter] - Trait defining methods to alter a graph, i.e. adding, removing, and editing nodes and edges.

 [SlotMapGraph] - Trait defining methods to access the nodes and edges of a graph where the nodes and edges are stored in slotmaps. 
 Implements [GraphWriter].

 [Graph] - The default graph struct which implements [SlotMapGraph]. It only contains two slotmaps, one for nodes and one for edges.

 [Categorized] - Trait that extends [SlotMapGraph] with category specific methods.

 [CategoryGraph] - A graph with categories. Categories are normal nodes (which can contain edges & data), but the graph also contains a hashmap that maps category names to category nodes for easy access.

 # Examples

 ## Simple [Graph] and the ABA problem.

 ```rs
 use fast_graph::{Graph, Node, Edge};
 /* We need to have these traits in scope: */
 use fast_graph::{SlotMapGraph, GraphWriter};

 #[derive(Clone, Debug)]
 struct EdgeData(String);
 
 #[derive(Clone, Debug)]
 struct NodeData(String);

 let mut graph: Graph<NodeData, EdgeData> = Graph::new();

 let node1 = graph.add_node(NodeData("Node 1".into())).clone();
 let node2 = graph.add_node(NodeData("Node 2".into())).clone();
 let edge1 = graph.add_edge(node1.id, node2.id, EdgeData("Edge 1".into())).clone();

 assert_eq!(graph.node(node1.id).unwrap().id, node1.id);
 assert_eq!(graph.edge(edge1.id).unwrap().id, edge1.id);

 graph.remove_node(node1.id).unwrap();

 // Since we just removed node 1, it should be None now.
 assert_eq!(graph.node(node1.id), None);
 // And node 2 still points to node 2.
 assert_eq!(graph.node(node2.id).unwrap().id, node2.id);

 println!("{:#?}", graph);

 ```

 ## [CategoryGraph] example
 ```rs
 use fast_graph::*;

 #[derive(Clone, Debug, Default, PartialEq)]
 #[cfg_attr(feature = "serde", derive(serde::Serialize))]
 enum NodeData {
     Number(u32),
     CategoryData(String),
     #[default]
     None,
 }

 let mut graph: CategoryGraph<NodeData, ()> = CategoryGraph::new();

 let node1 = graph.add_node(NodeData::Number(1)).id;
 let node2 = graph.add_node(NodeData::Number(2)).id;
 let node3 = graph.add_node(NodeData::Number(3)).id;

 let category1 = graph.create_category("Category 1", vec![node1, node2],
     NodeData::CategoryData("Category 1".into())
 ).unwrap();
 
 assert_eq!(graph.category("Category 1").unwrap().connections.len(), 2);
 
 // The category node should have the same data as the one we passed in.
 let category1_data = graph.category("Category 1").unwrap().data.clone();
 if let NodeData::CategoryData(category1_name) = category1_data {
    assert_eq!(category1_name, "Category 1".to_string());
 }
 
 // Adding to a category that doesn't exist will create it. 
 let category2 = graph.add_to_category("Category 2", vec![node2]);
 assert_eq!(graph.all_categories().len(), 2);
 
 // Adding to the same category twice will return the same category node.
 let category2_1 = graph.add_to_category("Category 2", vec![node3]);
 assert_eq!(graph.all_categories().len(), 2);
 assert_eq!(category2, category2_1);
 
 // The "Category 2" node should have two connections, one to node2 and one to node3.
 let category2_node = graph.category("Category 2").unwrap();
 assert_eq!(
 // this:
     category2_node.connections.iter()
         .map(|edge_id|
             graph.edge(*edge_id).unwrap().to
         )
         .collect::<Vec<NodeID>>(),
 // should equal:
     vec![node2, node3]
 );

 // Creating a category twice will error.
 assert!(
     graph.create_category("Category 1",
         vec![node3], NodeData::CategoryData("Category 1".into())
     ).is_err()
 );
 ```

## [Change log](CHANGELOG.md)


[Graph]: https://docs.rs/fast-graph/latest/fast_graph/struct.Graph.html
[Node]: https://docs.rs/fast-graph/latest/fast_graph/node/struct.Node.html
[NodeID]: https://docs.rs/fast-graph/latest/fast_graph/node/struct.NodeID.html
[Edge]: https://docs.rs/fast-graph/latest/fast_graph/edge/struct.Edge.html
[EdgeID]: https://docs.rs/fast-graph/latest/fast_graph/edge/struct.EdgeID.html
[GraphWriter]: https://docs.rs/fast-graph/latest/fast_graph/writer/trait.GraphWriter.html
[SlotMapGraph]: https://docs.rs/fast-graph/latest/fast_graph/slotmap_graph/trait.SlotMapGraph.html
[Categorized]: https://docs.rs/fast-graph/latest/fast_graph/categories/trait.Categorized.html
[CategoryGraph]: https://docs.rs/fast-graph/latest/fast_graph/categories/struct.CategoryGraph.html


[build_status]: https://github.com/henke443/fast-graph/workflows/Continuous%20integration/badge.svg?branch=master
[API documentation]: https://docs.rs/fast-graph/
[docsrs-badge]: https://img.shields.io/docsrs/fast-graph
[docsrs-url]: https://docs.rs/fast-graph/latest/fast_graph
[crates-badge]: https://img.shields.io/crates/v/fast-graph.svg
[crates-url]: https://crates.io/crates/fast-graph
[discord-badge]:  https://img.shields.io/discord/1225406740070404148?logo=discord&style=flat
[discord-url]: https://discord.gg/n2tc79tJ4e
[msrv-badge]: https://img.shields.io/badge/rustc-1.64+-blue.svg
[RELEASES]: RELEASES.rst
