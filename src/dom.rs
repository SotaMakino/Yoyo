use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(class_list) => class_list.split(' ').collect(),
            None => HashSet::new(),
        }
    }
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
        node_type: NodeType::Comment,
        children: Vec::new(),
    }
}

pub fn element(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_name,
            attributes: attrs,
        }),
        children,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_text() {
        let data = "data".to_string();
        let data2 = "data".to_string();
        assert_eq!(
            text(data),
            Node {
                node_type: NodeType::Text(data2),
                children: vec![]
            }
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            comment(),
            Node {
                node_type: NodeType::Comment,
                children: vec![]
            }
        );
    }

    #[test]
    fn test_element() {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "1".to_string());
        let mut attrs2 = HashMap::new();
        attrs2.insert("id".to_string(), "1".to_string());
        assert_eq!(
            element("h1".to_string(), attrs, vec![]),
            Node {
                node_type: NodeType::Element(ElementData {
                    tag_name: "h1".to_string(),
                    attributes: attrs2
                }),
                children: vec![]
            }
        );
    }
}
