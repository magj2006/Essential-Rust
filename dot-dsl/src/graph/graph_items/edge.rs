use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge<'a> {
    start: &'a str,
    end: &'a str,
    attr: HashMap<&'a str, &'a str>,
}

impl <'a> Edge<'a> {
    pub fn new(start: &'a str, end: &'a str) -> Self {
        Edge {
            start,
            end,
            attr: HashMap::new()
        }
    }

    pub fn with_attrs(mut self, attr: &[(&'a str, &'a str)]) -> Self {
        for (k, v) in attr.iter() {
            self.attr.insert(k, v);
        }

        self
    }
}