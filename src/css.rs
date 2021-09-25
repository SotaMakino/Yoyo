struct StyleSheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Eq)]
enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq, Eq)]
struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Option<String>,
}

#[derive(Debug, PartialEq)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug, PartialEq)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

#[derive(Debug, PartialEq, Eq)]
enum Unit {
    Px,
}

#[derive(Debug, PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.as_ref().unwrap().len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

struct Parser {
    pos: usize,
    input: String,
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

    fn parse_rules(&mut self) -> Rule {
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
                '#' | '{' => break,
                _ => panic!("unexpected char in selector"),
            }
        }
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        match self.next_char() {
            '#' => {
                self.consume_char();
                SimpleSelector {
                    id: Some(self.parse_name()),
                    class: None,
                    tag_name: None,
                }
            }
            _ => SimpleSelector {
                id: None,
                class: None,
                tag_name: Some(self.parse_name()),
            },
        }
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
            _ => Value::Keyword(self.parse_name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_selectors() {
        let source = "h1, h2 {";
        let mut parser = Parser {
            pos: 0,
            input: source.to_string(),
        };
        let expected = vec![
            Selector::Simple(SimpleSelector {
                id: None,
                class: None,
                tag_name: Some("h1".to_string()),
            }),
            Selector::Simple(SimpleSelector {
                id: None,
                class: None,
                tag_name: Some("h2".to_string()),
            }),
        ];

        assert_eq!(Parser::parse_selectors(&mut parser), expected);
    }

    #[test]
    fn parse_id_selectors() {
        let source = "#id {";
        let mut parser = Parser {
            pos: 0,
            input: source.to_string(),
        };
        let expected = vec![Selector::Simple(SimpleSelector {
            id: Some("id".to_string()),
            class: None,
            tag_name: None,
        })];

        assert_eq!(Parser::parse_selectors(&mut parser), expected);
    }

    #[test]
    fn parse_declarations() {
        let source = "{
            display: none;
            margin-left: 10.2px;
          }";
        let mut parser = Parser {
            pos: 0,
            input: source.to_string(),
        };
        println!("{:?}", Parser::parse_declarations(&mut parser));
    }

    #[test]
    fn parse_declaration() {
        let source = "margin: auto;";
        let mut parser = Parser {
            pos: 0,
            input: source.to_string(),
        };
        assert_eq!(
            Parser::parse_declaration(&mut parser),
            Declaration {
                name: "margin".to_string(),
                value: Value::Keyword("auto".to_string())
            }
        );
    }

    #[test]
    fn parse_keyword_value() {
        let source = "auto;";
        let mut parser = Parser {
            pos: 0,
            input: source.to_string(),
        };
        assert_eq!(
            Parser::parse_value(&mut parser),
            Value::Keyword("auto".to_string())
        );
    }

    #[test]
    fn parse_unit_value() {
        let source = "10px";
        let mut parser = Parser {
            pos: 0,
            input: source.to_string(),
        };
        assert_eq!(
            Parser::parse_value(&mut parser),
            Value::Length(10.0, Unit::Px)
        );
    }
}
