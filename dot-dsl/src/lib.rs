pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;

    pub struct Graph {
        pub nodes : Vec<graph_items::node::Node>,
        pub edges : Vec<graph_items::edge::Edge>,
        pub attrs : HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: Default::default(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.clone();
            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            pub struct Edge(String, String);

            impl Edge {
                pub fn new(begin: &str, end: &str) -> Self {
                    Edge(begin.to_owned(), end.to_owned())
                }
            }
        }

        pub mod node {
            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct Node(String);

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node(value.to_owned())
                }
            }
        }
    }
}
