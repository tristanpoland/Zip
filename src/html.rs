// src/html.rs
use crate::dom::{Node, NodeType};
use scraper::{Html, Selector};
use std::sync::Arc;

pub struct HTMLParser;

impl HTMLParser {
    pub fn parse(source: &str) -> Result<Node, crate::BrowserError> {
        let document = Html::parse_document(source);
        let html = document.root_element();
        
        Ok(Self::parse_node(&html))
    }
    
    fn parse_node(element: &scraper::ElementRef) -> Node {
        let tag_name = element.value().name();
        let mut node = Node::new_element(tag_name);
        
        // Parse attributes
        for (key, value) in element.value().attrs() {
            node.set_attribute(key, value);
        }
        
        // Parse children
        for child in element.children() {
            if let Some(element) = child.value().as_element() {
                if let Some(element_ref) = scraper::ElementRef::wrap(child) {
                    node.add_child(Self::parse_node(&element_ref));
                }
            } else if let Some(text) = child.value().as_text() {
                if !text.trim().is_empty() {
                    node.add_child(Node::new_text(text));
                }
            }
        }
        
        node
    }
}