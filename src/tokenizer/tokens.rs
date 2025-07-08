#![allow(unused_variables, dead_code)]
use std::fmt::{self, Display, Write};

// Possible(valid) tokens that can be found on a mathematical expressions entered by the user.
#[derive(Debug, PartialEq, Clone)]
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
    CONSTANT(char),
    EOF,
    WhiteSpace(WhiteSpace),
}

// Implementing the display trait to write the standard output for the Token
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
            Self::CONSTANT(c) => write!(f, "{}", c),
            Self::WhiteSpace(whitespace) => write!(f, "{whitespace}"),
        }
    }
}

// Possible WhiteSpaces that can be encountered while parsing mathematical expressions
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum WhiteSpace {
    SPACE,
    TAB,
    NEWLINE,
}

// Implementing the display trait to write the standard output for the Token
impl Display for WhiteSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Self::TAB => '\t',
            Self::SPACE => ' ',
            Self::NEWLINE => '\n',
        })
    }
}
