use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

#[derive(Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(),
}

#[derive(Debug)]
struct ElementData {
    tag_names: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {
        node_type: NodeType::Text(data),
        children: Vec::new(),
    }
}

pub fn comment() -> Node {
    Node {
        node_type: NodeType::Comment(),
        children: Vec::new(),
    }
}

pub fn element(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_names: tag_name,
            attributes: attrs,
        }),
        children,
    }
}
