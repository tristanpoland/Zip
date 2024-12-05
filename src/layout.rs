pub mod layout {
    use crate::dom::Node;
    use std::collections::HashMap;
    use druid::Data;

    #[derive(Debug, Clone, Data)]
    pub struct LayoutBox {
        pub dimensions: Dimensions,
        pub box_type: BoxType,
        #[data(ignore)]
        pub children: Vec<LayoutBox>,
    }

    #[derive(Debug, Clone, Data)]
    pub struct Dimensions {
        pub content: Rect,
        pub padding: EdgeSizes,
        pub border: EdgeSizes,
        pub margin: EdgeSizes,
    }

    #[derive(Debug, Clone, Data)]
    pub struct Rect {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
    }

    #[derive(Debug, Clone, Data)]
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
            let mut layout_box = LayoutBox {
                dimensions: Dimensions {
                    content: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 800.0, // Default width
                        height: 0.0,
                    },
                    padding: EdgeSizes {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    },
                    border: EdgeSizes {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    },
                    margin: EdgeSizes {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    },
                },
                box_type: BoxType::Block,
                children: Vec::new(),
            };

            let mut current_y = 0.0;
            for child in node.children.iter() {
                let mut child_box = Self::layout(child, styles);
                child_box.dimensions.content.y = current_y;
                current_y += child_box.dimensions.content.height;
                layout_box.children.push(child_box);
            }

            layout_box.dimensions.content.height = current_y;
            layout_box
        }
    }
}
