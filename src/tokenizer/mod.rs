use std::fmt;
use std::fmt::{Display, Write};

pub(crate) enum Token {
    NUMBER(i32),
    PLUS,
    MINUS,
    MUL,
    DIV,
    POW,
    LEFTPAREN,
    RIGHTPAREN,
    VARIABLE(char),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PLUS => f.write_str("+"),
            Self::MINUS => f.write_str("-"),
            Self::MUL => f.write_str("*"),
            Self::DIV => f.write_str("/"),
            Self::POW => f.write_str("^"),
            Self::LEFTPAREN => f.write_str("("),
            Self::RIGHTPAREN => f.write_str(")"),
            Self::EOF => f.write_str("<EOF>"),
            Self::NUMBER(val) => write!(f, "{}", val),
            Self::VARIABLE(name) => write!(f, "{}", name),
        }
    }
}

pub(crate) enum WhiteSpace {
    SPACE,
    TAB,
    NEWLINE,
}
impl Display for WhiteSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Self::TAB => '\t',
            Self::SPACE => ' ',
            Self::NEWLINE => '\n',
        })
    }
}

