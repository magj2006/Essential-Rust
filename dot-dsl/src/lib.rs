
pub mod graph {
    pub mod graph_items;

    use graph_items::node::Node;
    use graph_items::edge::Edge;

    use std::collections::HashMap;

    #[derive(Clone, Default)]
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl <'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {                    
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        /*
        pub fn with_nodes(mut self, nodes: &'a [Node<'a>]) -> Self {
             self.nodes.extend_from_slice(nodes);
            //self.nodes = nodes.to_vec();
            self
        }
        */
        pub fn with_nodes(self, nodes: &'a [Node]) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(mut self, edges: &'a [Edge]) -> Self {
            edges.iter().for_each(|e| self.edges.push((*e).clone()));

            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            for (k, v) in attrs.iter() {
                self.attrs.insert(k.to_string(), v.to_string());
            }
    
            self
        }

        pub fn get_node(&'a self, name: &'a str) -> Option<&Node> {
            self.nodes.iter().find(|&x| x.name == name)
        }
    }
}


