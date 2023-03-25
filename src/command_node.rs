use std::fmt;

pub enum CommandNode {
    STRING(String),
    NUMBER(usize)
}

impl CommandNode {

    pub fn get_name(&self) -> String {
        match self {
            Self::STRING(_) => String::from("s"),
            Self::NUMBER(_) => String::from("n")
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::STRING(s) => Some(s),
            Self::NUMBER(_) => None
        }
    }

    pub fn as_number(&self) -> Option<usize> {
        match self {
            Self::STRING(_) => None,
            Self::NUMBER(n) => Some(*n)
        }
    }
}

impl fmt::Display for CommandNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::STRING(s) => {
                write!(f, "STR:{}", s)
            },
            Self::NUMBER(n) => {
                write!(f, "NUM:{}", n)
            }
        }
    }
}