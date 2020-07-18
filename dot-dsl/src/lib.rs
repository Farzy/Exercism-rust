pub mod graph {
    use std::collections::HashMap;

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
    }

    pub mod graph_items {
        pub mod edge {
            pub struct Edge;

            impl Edge {
                pub fn new() -> Self {
                    unimplemented!("Construct a new Edge struct.");
                }
            }
        }

        pub mod node {
            pub struct Node;

            impl Node {
                pub fn new() -> Self {
                    unimplemented!("Construct a new Node struct.");
                }
            }
        }
    }
}
