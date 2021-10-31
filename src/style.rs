use std::collections::HashMap;

use crate::{
    css::{self, Value},
    dom,
};

type PropertyMap = HashMap<String, css::Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a dom::Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Block,
    Inline,
    None,
}

impl StyledNode<'_> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "inline" => Display::Inline,
                _ => Display::None,
            },
            _ => Display::Inline,
        }
    }
}

pub fn matches(elem: &dom::ElementData, selector: &css::Selector) -> bool {
    match *selector {
        css::Selector::Simple(ref simple_selector) => {
            matches_simple_selectors(elem, simple_selector)
        }
    }
}

pub fn matches_simple_selectors(elem: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    };
    true
}

type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

fn match_rule<'a>(elem: &dom::ElementData, rule: &'a css::Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn match_rules<'a>(
    elem: &dom::ElementData,
    style_sheet: &'a css::StyleSheet,
) -> Vec<MatchedRule<'a>> {
    style_sheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn specified_values(elem: &dom::ElementData, style_sheet: &css::StyleSheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = match_rules(elem, style_sheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

pub fn style_tree<'a>(root: &'a dom::Node, style_sheet: &'a css::StyleSheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            dom::NodeType::Element(ref elem) => specified_values(elem, style_sheet),
            dom::NodeType::Text(_) => HashMap::new(),
            dom::NodeType::Comment => todo!(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, style_sheet))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn rule_1() -> css::Rule {
        css::Rule {
            selectors: vec![css::Selector::Simple(css::SimpleSelector {
                tag_name: None,
                id: Some("1".to_string()),
                class: Vec::new(),
            })],
            declarations: vec![css::Declaration {
                name: "margin".to_string(),
                value: css::Value::Keyword("auto".to_string()),
            }],
        }
    }

    fn rule_2() -> css::Rule {
        css::Rule {
            selectors: vec![css::Selector::Simple(css::SimpleSelector {
                tag_name: Some("h1".to_string()),
                id: None,
                class: Vec::new(),
            })],
            declarations: vec![css::Declaration {
                name: "margin".to_string(),
                value: css::Value::Keyword("0".to_string()),
            }],
        }
    }

    fn style_sheet() -> css::StyleSheet {
        css::StyleSheet {
            rules: vec![rule_1(), rule_2()],
        }
    }

    #[test]
    fn test_matches_simple_selectors() {
        let mut hash = HashMap::new();
        hash.insert("id".to_string(), "1".to_string());
        hash.insert("class".to_string(), "square".to_string());
        let elem = dom::ElementData {
            tag_name: "h1".to_string(),
            attributes: hash,
        };
        let heading_selector = css::SimpleSelector {
            tag_name: Some("h1".to_string()),
            id: None,
            class: Vec::new(),
        };
        let id_selector = css::SimpleSelector {
            tag_name: None,
            id: Some("1".to_string()),
            class: Vec::new(),
        };
        let class_selector = css::SimpleSelector {
            tag_name: None,
            id: None,
            class: vec!["square".to_string()],
        };
        assert!(matches_simple_selectors(&elem, &heading_selector));
        assert!(matches_simple_selectors(&elem, &id_selector));
        assert!(matches_simple_selectors(&elem, &class_selector))
    }

    #[test]
    fn test_match_rules() {
        let mut hash = HashMap::new();
        hash.insert("id".to_string(), "1".to_string());
        let elem = dom::ElementData {
            tag_name: "h1".to_string(),
            attributes: hash,
        };
        println!("{:?}", match_rules(&elem, &style_sheet()));
    }

    #[test]
    fn test_specified_values() {
        let mut hash = HashMap::new();
        hash.insert("id".to_string(), "1".to_string());
        let elem = dom::ElementData {
            tag_name: "h1".to_string(),
            attributes: hash,
        };
        println!("{:?}", specified_values(&elem, &style_sheet()));
    }
}
