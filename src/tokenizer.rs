use crate::command_node::CommandNode;
use std::str::Chars;

pub struct Tokenizer<'a> {
    pub iterator: Chars<'a>
}

impl Tokenizer<'_> {

    fn eat_string(&mut self) -> String {
        let mut s = String::new();
        loop {
            match self.iterator.next() {
                Some(c) => {
                    if c == '"' {
                        break
                    } else {
                        s.push(c);
                    }
                },
                None => { break; }
            }
        }
        return s;
    }

    fn eat_number(&mut self, start: char) -> usize {
        let mut num = String::from(start);
        loop {
            match self.iterator.next() {
                Some(c) => {
                    if c.is_digit(10) {
                        num.push(c);
                    } else {
                        break;
                    }
                },
                None => { break; }
            }
        }
        return num.parse::<usize>().unwrap();
    }

    pub fn tokenize(&mut self) -> Vec<CommandNode> {
        
        let mut result: Vec<CommandNode> = Vec::new();

        loop {
            match self.iterator.next() {
                Some(c) => {
                    if c.is_digit(10) {
                        result.push(CommandNode::NUMBER(self.eat_number(c)));
                    } else if c == '"' {
                        result.push(CommandNode::STRING(self.eat_string()));
                    }
                },
                None => { break; }
            }
        }

        return result;
    }
}