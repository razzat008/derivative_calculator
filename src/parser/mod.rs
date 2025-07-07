pub mod ast;
use crate::tokenizer::tokens::{self, Token};
use ast::{BinaryOp, Expr, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    pub fn parse(&mut self) -> Option<Expr> {
        let expr = self.parse_expr()?;
        // Ensure all tokens are consumed (except whitespace or EOF)
        while let Some(token) = self.peek() {
            match token {
                Token::WhiteSpace(_) => {
                    self.next();
                }
                Token::EOF => break,
                _ => return None, // Unexpected token after valid expr
            }
        }
        Some(expr)
    }

    // expr = term (('+' | '-') term)*
    fn parse_expr(&mut self) -> Option<Expr> {
        let mut node = self.parse_term()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::PLUS => {
                    self.next();
                    node = Expr::BinaryOp {
                        op: BinaryOp::Add,
                        left: Box::new(node),
                        right: Box::new(self.parse_term()?),
                    };
                }
                Token::MINUS => {
                    self.next();
                    node = Expr::BinaryOp {
                        op: BinaryOp::Sub,
                        left: Box::new(node),
                        right: Box::new(self.parse_term()?),
                    };
                }
                _ => break,
            }
        }
        Some(node)
    }

    // term = factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> Option<Expr> {
        let mut node = self.parse_factor()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::MUL => {
                    self.next();
                    node = Expr::BinaryOp {
                        op: BinaryOp::Mul,
                        left: Box::new(node),
                        right: Box::new(self.parse_factor()?),
                    };
                }
                Token::DIV => {
                    self.next();
                    node = Expr::BinaryOp {
                        op: BinaryOp::Div,
                        left: Box::new(node),
                        right: Box::new(self.parse_factor()?),
                    };
                }
                _ => break,
            }
        }
        Some(node)
    }

    // factor = base ('^' factor)?
    fn parse_factor(&mut self) -> Option<Expr> {
        let mut node = self.parse_base()?;
        if let Some(Token::POW) = self.peek() {
            self.next();
            node = Expr::BinaryOp {
                op: BinaryOp::Pow,
                left: Box::new(node),
                right: Box::new(self.parse_factor()?),
            };
        }
        Some(node)
    }

    // base = NUMBER | VARIABLE | '(' expr ')' | '-' base
    fn parse_base(&mut self) -> Option<Expr> {
        match self.peek()? {
            Token::NUMBER(n) => {
                let n = *n;
                self.next();
                Some(Expr::Number(n))
            }
            Token::VARIABLE(c) => {
                let c = *c;
                self.next();
                Some(Expr::Variable(c))
            }
            Token::LEFTPAREN => {
                self.next();
                let expr = self.parse_expr();
                if let Some(Token::RIGHTPAREN) = self.peek() {
                    self.next();
                    expr
                } else {
                    None
                }
            }
            Token::MINUS => {
                self.next();
                Some(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(self.parse_base()?),
                })
            }
            _ => None,
        }
    }
}
