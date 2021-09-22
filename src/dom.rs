use std::collections::HashMap;

pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

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

pub fn element(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_names: tag_name,
            attributes: attrs,
        }),
        children,
    }
}
