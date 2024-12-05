pub mod layout {
    use crate::dom::{Node, NodeType};
    use std::collections::HashMap;
    use druid::Data;

    #[derive(Debug, Clone, Data)]
    pub struct LayoutBox {
        pub dimensions: Dimensions,
        pub box_type: BoxType,
        #[data(ignore)]
        pub children: Vec<LayoutBox>,
    }

    impl LayoutBox {
        pub fn new(box_type: BoxType) -> Self {
            LayoutBox {
                dimensions: Dimensions::default(),
                box_type,
                children: Vec::new(),
            }
        }

        fn calculate_dimensions(&mut self, node: &Node, styles: &HashMap<String, String>) {
            // TODO: Implement dimension calculation based on styles
        }

        fn layout_children(&mut self, node: &Node, styles: &HashMap<String, String>) {
            for child in node.children().iter() {
                let child_box = LayoutEngine::layout(child, styles);
                self.children.push(child_box);
            }
        }
    }

    #[derive(Debug, Clone, Data, Default)]
    pub struct Dimensions {
        pub content: Rect,
        pub padding: EdgeSizes,
        pub border: EdgeSizes,
        pub margin: EdgeSizes,
    }

    #[derive(Debug, Clone, Data, Default)]
    pub struct Rect {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
    }

    #[derive(Debug, Clone, Data, Default)]
    pub struct EdgeSizes {
        pub top: f32,
        pub right: f32,
        pub bottom: f32,
        pub left: f32,
    }

    #[derive(Debug, Clone, Data, PartialEq)]
    pub enum BoxType {
        Block,
        Inline,
        Anonymous,
    }

    pub struct LayoutEngine;

    impl LayoutEngine {
        pub fn layout(node: &Node, styles: &HashMap<String, String>) -> LayoutBox {
            let box_type = match &*node.node_type {
                NodeType::Element(tag) => {
                    match tag.as_str() {
                        "div" | "p" | "h1" | "h2" | "h3" => BoxType::Block,
                        "span" | "a" | "strong" | "em" => BoxType::Inline,
                        _ => BoxType::Block,
                    }
                }
                NodeType::Text(_) => BoxType::Inline,
                NodeType::Document => BoxType::Block,
            };

            let mut layout_box = LayoutBox::new(box_type);
            layout_box.calculate_dimensions(node, styles);
            layout_box.layout_children(node, styles);
            layout_box
        }
    }
}
