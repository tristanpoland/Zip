// src/dom.rs
use std::collections::HashMap;
use std::sync::Arc;
use druid::Data;

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: Arc<NodeType>,
    pub children: Arc<Vec<Node>>,
    pub attributes: Arc<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Element(String),  // Tag name
    Text(String),
    Document,
}

impl Data for NodeType {
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeType::Element(a), NodeType::Element(b)) => a == b,
            (NodeType::Text(a), NodeType::Text(b)) => a == b,
            (NodeType::Document, NodeType::Document) => true,
            _ => false,
        }
    }
}

impl Data for Node {
    fn same(&self, other: &Self) -> bool {
        // Compare node types
        self.node_type.same(&other.node_type)
            // For children, we'll just compare lengths as a basic check
            // A more thorough implementation would compare each child
            && Arc::ptr_eq(&self.children, &other.children)
            // For attributes, compare Arc pointers
            && Arc::ptr_eq(&self.attributes, &other.attributes)
    }
}

impl Node {
    pub fn new_element(tag_name: &str) -> Self {
        Node {
            node_type: Arc::new(NodeType::Element(tag_name.to_string())),
            children: Arc::new(Vec::new()),
            attributes: Arc::new(HashMap::new()),
        }
    }
    
    pub fn new_text(data: &str) -> Self {
        Node {
            node_type: Arc::new(NodeType::Text(data.to_string())),
            children: Arc::new(Vec::new()),
            attributes: Arc::new(HashMap::new()),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        Arc::make_mut(&mut self.children).push(child);
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        Arc::make_mut(&mut self.attributes).insert(key.to_string(), value.to_string());
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node> {
        Arc::make_mut(&mut self.children)
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut HashMap<String, String> {
        Arc::make_mut(&mut self.attributes)
    }
}