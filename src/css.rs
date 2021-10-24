pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Unit {
    Px,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
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

    pub fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule())
        }
        rules
    }

    pub fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-'))
    }

    fn parse_number(&mut self) -> f32 {
        self.consume_while(|char| matches!(char, '0'..='9' | '.'))
            .parse()
            .unwrap()
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                _ => panic!("unexpected char in selector"),
            }
        }
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            id: None,
            class: Vec::new(),
            tag_name: None,
        };
        while !self.eof() {
            self.consume_whitespace();
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_name());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_name());
                }
                ',' | '{' => {
                    break;
                }
                _ => {
                    selector.tag_name = Some(self.parse_name());
                }
            }
        }
        selector
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
        assert!(self.consume_char() == '{');
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                break;
            }
            declarations.push(self.parse_declaration());
            match self.next_char() {
                ';' => {
                    self.consume_char();
                }
                _ => panic!("unexpected char in declaration"),
            }
        }
        assert!(self.consume_char() == '}');
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_name();
        assert!(self.consume_char() == ':');
        self.consume_whitespace();
        let value = self.parse_value();
        Declaration { name, value }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => {
                let length = self.parse_number();
                self.consume_while(|char| char != ';');
                Value::Length(length, Unit::Px)
            }
            '#' => {
                self.consume_char();
                Value::Color(self.parse_color())
            }
            _ => Value::Keyword(self.parse_name()),
        }
    }

    fn parse_color(&mut self) -> Color {
        Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        }
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let pair_str = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(pair_str, 16).unwrap()
    }
}

pub fn parse(source: String) -> StyleSheet {
    let mut parser = Parser {
        pos: 0,
        input: source,
    };

    StyleSheet {
        rules: parser.parse_rules(),
    }
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
    fn test_parse_rules() {
        let source = "
        h1 {
          margin: auto;
        }

        h2 {
          color: #111111;
        }
        ";
        println!("{:?}", Parser::parse_rules(&mut get_parser(source)));
    }

    #[test]
    fn test_parse_rule() {
        let source = "h1,
        h2,
        h3 {
          margin: auto;
          display: inline;
        }
        ";
        println!("{:?}", Parser::parse_rule(&mut get_parser(source)));
    }

    #[test]
    fn test_parse_simple_selectors() {
        let source = "h1, h2 {";
        let expected = vec![
            Selector::Simple(SimpleSelector {
                id: None,
                class: Vec::new(),
                tag_name: Some("h1".to_string()),
            }),
            Selector::Simple(SimpleSelector {
                id: None,
                class: Vec::new(),
                tag_name: Some("h2".to_string()),
            }),
        ];
        assert_eq!(Parser::parse_selectors(&mut get_parser(source)), expected);
    }

    #[test]
    fn test_parse_class_selectors() {
        let source = ".fruit.apple {";
        let expected = vec![Selector::Simple(SimpleSelector {
            id: None,
            class: vec!["fruit".to_string(), "apple".to_string()],
            tag_name: None,
        })];
        assert_eq!(Parser::parse_selectors(&mut get_parser(source)), expected);
    }

    #[test]
    fn test_parse_id_selectors() {
        let source = "#id {";
        let expected = vec![Selector::Simple(SimpleSelector {
            id: Some("id".to_string()),
            class: Vec::new(),
            tag_name: None,
        })];
        assert_eq!(Parser::parse_selectors(&mut get_parser(source)), expected);
    }

    #[test]
    fn test_parse_declarations() {
        let source = "{
            display: none;
            margin-left: 10.2px;
          }";
        println!("{:?}", Parser::parse_declarations(&mut get_parser(source)));
    }

    #[test]
    fn test_parse_declaration() {
        let source = "margin: auto;";
        assert_eq!(
            Parser::parse_declaration(&mut get_parser(source)),
            Declaration {
                name: "margin".to_string(),
                value: Value::Keyword("auto".to_string())
            }
        );
    }

    #[test]
    fn test_parse_keyword_value() {
        let source = "auto;";
        assert_eq!(
            Parser::parse_value(&mut get_parser(source)),
            Value::Keyword("auto".to_string())
        );
    }

    #[test]
    fn test_parse_unit_value() {
        let source = "10px";
        assert_eq!(
            Parser::parse_value(&mut get_parser(source)),
            Value::Length(10.0, Unit::Px)
        );
    }

    #[test]
    fn test_parse_color_value() {
        let source = "#812dd3";
        assert_eq!(
            Parser::parse_value(&mut get_parser(source)),
            Value::Color(Color {
                r: 129,
                g: 45,
                b: 211,
                a: 255
            })
        );
    }
}
