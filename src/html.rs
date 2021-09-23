use std::collections::HashMap;

use crate::dom;

#[derive(Debug, PartialEq, Eq)]
struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn next_next_char(&self) -> char {
        let mut iter = self.input[self.pos..].chars();
        iter.next();
        iter.next().unwrap()
    }

    fn start_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, '_'));
        self.pos += next_pos;
        current_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char())
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|char| match char {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => match self.next_next_char() {
                '!' => self.parse_comment(),
                _ => self.parse_element(),
            },
            _ => self.parse_text(),
        }
    }

    fn parse_comment(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        self.consume_while(|char| char != '<');
        dom::comment()
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|char| char != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::element(tag_name, attrs, children)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attributes_value();
        (name, value)
    }

    fn parse_attributes_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|char| char != open_quote);
        assert!(self.consume_char() == open_quote);
        value
    }

    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.start_with("</") {
                break;
            }
            let node = self.parse_node();
            nodes.push(node);
        }
        nodes
    }
}

pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    nodes.pop().unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn parse_nodes() {
        let source = "<title id='1'>Test</title>";
        let mut parser = Parser {
            pos: 0,
            input: source.to_string(),
        };
        let mut hash = HashMap::new();
        hash.insert(String::from("id"), String::from("1"));
        let expected = vec![dom::Node {
            node_type: dom::NodeType::Element(dom::ElementData {
                tag_names: "title".to_string(),
                attributes: hash,
            }),
            children: vec![dom::Node {
                node_type: dom::NodeType::Text("Test".to_string()),
                children: vec![],
            }],
        }];

        assert_eq!(Parser::parse_nodes(&mut parser), expected);
    }
}
