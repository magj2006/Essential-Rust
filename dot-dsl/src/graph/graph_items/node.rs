use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node<'a> {
    pub name: &'a str,
    attr: HashMap<&'a str, &'a str>,
}

impl <'a> Node<'a> {
    pub fn new(name: &'a str) -> Self {
        Node {
            name, 
            attr: HashMap::new()
        }
    }

    pub fn with_attrs(mut self, attr: &[(&'a str, &'a str)]) -> Self {
        for (k, v) in attr.iter() {
            self.attr.insert(k, v);
        }

        self
    }

    pub fn get_attr(&self, key: &'a str) -> Option<&'a str> {
        match self.attr.get(key)  {
            Some(&x) => Some(x),
            None => None,
        }
    }
}