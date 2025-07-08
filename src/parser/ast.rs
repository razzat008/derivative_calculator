#[derive(Debug, Clone, PartialEq)]
/// Represents an expression in the AST. An expression is just a variation of these enum members.
pub enum Expr {
    Number(i32),
    Variable(char),
    Constant(char),
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a unary operation in the AST.
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a binary operation in the AST.
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
