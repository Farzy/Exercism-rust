pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;
    use maplit::hashmap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap! {},
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .into_iter()
                .map(|(key, value)|
                    ((*key).to_owned(), (*value).to_owned()))
                .collect();
            self
        }

        pub fn get_node(&self, nodename: &str) -> Option<Node> {
            self.nodes
                .iter()
                .find(|&node| node.value == nodename)
                .cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use maplit::hashmap;

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
                        attrs: hashmap! {}
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
            use maplit::hashmap;

            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct Node {
                pub value: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node {
                        value: value.to_owned(),
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .into_iter()
                        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                        .collect();
                    self
                }

                pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(attr_name).map(|s| s.as_str())
                }
            }
        }
    }
}
