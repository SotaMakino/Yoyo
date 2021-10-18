use std::collections::HashMap;

use crate::dom;

#[derive(Debug, PartialEq, Eq)]
pub struct Parser {
    pub pos: usize,
    pub input: String,
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
        self.consume_while(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' ))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_parser(source: &str) -> Parser {
        Parser {
            pos: 0,
            input: source.to_string(),
        }
    }

    #[test]
    fn test_next_char() {
        let source = "Test";
        assert_eq!(Parser::next_char(&get_parser(source)), 'T');
    }

    #[test]
    fn test_next_next_char() {
        let source = "Test";
        assert_eq!(Parser::next_next_char(&get_parser(source)), 'e');
    }

    #[test]
    fn test_start_with() {
        let source = "Test";
        assert!(Parser::start_with(&get_parser(source), "T"));
        assert!(!Parser::start_with(&get_parser(source), "e"));
    }

    #[test]
    fn test_eof() {
        let source = "Test";
        let mut parser = get_parser(source);
        assert!(!Parser::eof(&parser));
        parser.pos = 4;
        assert!(Parser::eof(&parser));
    }

    #[test]
    fn test_consume_char() {
        let source = "Test";
        assert_eq!(Parser::consume_char(&mut get_parser(source)), 'T');
    }

    #[test]
    fn test_consume_while() {
        let source = "test";
        assert_eq!(
            Parser::consume_while(&mut get_parser(source), |char| matches!(char, 'a'..='z')),
            "test"
        );
    }

    #[test]
    fn test_consume_whitespace() {
        let source = "   test";
        let mut parser = get_parser(source);
        Parser::consume_whitespace(&mut parser);
        assert_eq!(parser.pos, 3);
    }

    #[test]
    fn test_parse_tag_name() {
        let source = "h1>";
        assert_eq!(Parser::parse_tag_name(&mut get_parser(source)), "h1");
    }

    #[test]
    fn test_parse_node() {
        let comment = "<!-- comment -->";
        let elem = "<title>Title</title>";
        let text = "text";
        assert_eq!(
            Parser::parse_node(&mut get_parser(comment)),
            Parser::parse_comment(&mut get_parser(comment))
        );
        assert_eq!(
            Parser::parse_node(&mut get_parser(elem)),
            Parser::parse_element(&mut get_parser(elem))
        );
        assert_eq!(
            Parser::parse_node(&mut get_parser(text)),
            Parser::parse_text(&mut get_parser(text))
        );
    }

    #[test]
    fn test_parse_comment() {
        let comment = "<!-- comment -->";
        assert_eq!(
            Parser::parse_comment(&mut get_parser(comment)),
            dom::comment()
        );
    }

    #[test]
    fn test_parse_text() {
        let text = "text<p>";
        let node = dom::Node {
            node_type: dom::NodeType::Text("text".to_string()),
            children: Vec::new(),
        };
        assert_eq!(Parser::parse_text(&mut get_parser(text)), node);
    }

    #[test]
    fn test_parse_element() {
        let elem = "<title>Title</title>";
        let expected = dom::element(
            "title".to_string(),
            HashMap::new(),
            vec![dom::text("Title".to_string())],
        );
        assert_eq!(Parser::parse_element(&mut get_parser(elem)), expected);
    }

    #[test]
    fn test_parse_attr() {
        let attr = "id=\"1\"";
        assert_eq!(
            Parser::parse_attr(&mut get_parser(attr)),
            ("id".to_string(), "1".to_string())
        );
    }

    #[test]
    fn test_parse_attributes_value() {
        let value = "\"1\"";
        assert_eq!(
            Parser::parse_attributes_value(&mut get_parser(value)),
            "1".to_string()
        );
    }

    #[test]
    fn test_parse_nodes() {
        let source = "<title id='1'>Test</title>";
        println!("{:?}", Parser::parse_nodes(&mut get_parser(source)));
    }

    #[test]
    fn test_parse_nodes_with_comments() {
        let source = "
        <div>
            <!-- comments -->
            <title id='1'>Test</title>
        </div>
        ";
        println!("{:?}", Parser::parse_nodes(&mut get_parser(source)));
    }
}
