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

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

enum Unit {
    Px,
}

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

    // fn parse_rules(&mut self) -> Rule {
    //     Rule {
    //         selectors: self.parse_selectors(),
    //         declarations: self.parse_declarations(),
    //     }
    // }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|char| match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace()
                }
                '{' => break,
                _ => panic!("unexpected char in selector"),
            }
        }
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        SimpleSelector {
            id: None,
            class: None,
            tag_name: Some(self.parse_tag_name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_selectors() {
        let source = "h1, h2{";
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
}
