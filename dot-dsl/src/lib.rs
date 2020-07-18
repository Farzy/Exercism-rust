pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;

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

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.clone();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .into_iter()
                .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                .collect();
            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct Edge {
                begin: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(begin: &str, end: &str) -> Self {
                    Edge {
                        begin: begin.to_owned(),
                        end: end.to_owned(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .into_iter()
                        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                        .collect();
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct Node {
                value: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node{
                        value: value.to_owned(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .into_iter()
                        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                        .collect();
                    self
                }
            }
        }
    }
}
