use std::collections::HashMap;

struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
    tag_names: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        node_type: NodeType::Text(data),
        children: Vec::new(),
    }
}

fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_names: name,
            attributes: attrs,
        }),
        children,
    }
}
