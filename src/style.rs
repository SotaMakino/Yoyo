use std::collections::HashMap;

use crate::{css, dom};

type PropertyMap = HashMap<String, css::Value>;

pub struct StyledNode<'a> {
    node: &'a dom::Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
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
            dom::NodeType::Comment() => todo!(),
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

    #[test]
    fn test_match_rules() {
        let mut hash = HashMap::new();
        hash.insert("id".to_string(), "1".to_string());
        let elem = dom::ElementData {
            tag_name: "h1".to_string(),
            attributes: hash,
        };
        let style_sheet = css::StyleSheet {
            rules: vec![css::Rule {
                selectors: vec![css::Selector::Simple(css::SimpleSelector {
                    tag_name: Some("h1".to_string()),
                    id: None,
                    class: Vec::new(),
                })],
                declarations: vec![css::Declaration {
                    name: "margin".to_string(),
                    value: css::Value::Keyword("auto".to_string()),
                }],
            }],
        };

        println!("{:?}", match_rules(&elem, &style_sheet));
    }

    #[test]
    fn test_matches_simple_selectors() {
        let mut hash = HashMap::new();
        hash.insert("id".to_string(), "1".to_string());
        let elem = dom::ElementData {
            tag_name: "h1".to_string(),
            attributes: hash,
        };
        let selector_with_heading = css::SimpleSelector {
            tag_name: Some("h1".to_string()),
            id: None,
            class: Vec::new(),
        };
        let selector_with_para = css::SimpleSelector {
            tag_name: Some("p".to_string()),
            id: None,
            class: Vec::new(),
        };
        assert!(matches_simple_selectors(&elem, &selector_with_heading));
        assert!(!matches_simple_selectors(&elem, &selector_with_para));
    }
}
