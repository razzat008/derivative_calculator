use crate::parser::ast::{BinaryOp, Expr, UnaryOp};

pub fn differentiate(expr: &Expr) -> Expr {
    match expr {
        Expr::Number(_) => Expr::Number(0),
        Expr::Variable(c) if *c == 'x' => Expr::Number(1),
        Expr::Variable(_) => Expr::Number(0),
        Expr::Constant(_) => Expr::Number(0),
        Expr::UnaryOp {
            op: UnaryOp::Neg,
            expr,
        } => Expr::UnaryOp {
            op: UnaryOp::Neg,
            expr: Box::new(differentiate(expr)),
        },
        Expr::BinaryOp {
            op: BinaryOp::Add,
            left,
            right,
        } => Expr::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(differentiate(left)),
            right: Box::new(differentiate(right)),
        },
        Expr::BinaryOp {
            op: BinaryOp::Sub,
            left,
            right,
        } => Expr::BinaryOp {
            op: BinaryOp::Sub,
            left: Box::new(differentiate(left)),
            right: Box::new(differentiate(right)),
        },
        Expr::BinaryOp {
            op: BinaryOp::Mul,
            left,
            right,
        } => Expr::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(Expr::BinaryOp {
                op: BinaryOp::Mul,
                left: Box::new(differentiate(left)),
                right: right.clone(),
            }),
            right: Box::new(Expr::BinaryOp {
                op: BinaryOp::Mul,
                left: left.clone(),
                right: Box::new(differentiate(right)),
            }),
        },
        Expr::BinaryOp {
            op: BinaryOp::Div,
            left,
            right,
        } => Expr::BinaryOp {
            op: BinaryOp::Div,
            left: Box::new(Expr::BinaryOp {
                op: BinaryOp::Sub,
                left: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Mul,
                    left: Box::new(differentiate(left)),
                    right: right.clone(),
                }),
                right: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Mul,
                    left: left.clone(),
                    right: Box::new(differentiate(right)),
                }),
            }),
            right: Box::new(Expr::BinaryOp {
                op: BinaryOp::Pow,
                left: right.clone(),
                right: Box::new(Expr::Number(2)),
            }),
        },
        Expr::BinaryOp {
            op: BinaryOp::Pow,
            left,
            right,
        } => {
            // Only handle x^n where n is a constant for now
            if let Expr::Number(n) = **right {
                Expr::BinaryOp {
                    op: BinaryOp::Mul,
                    left: Box::new(Expr::Number(n)),
                    right: Box::new(Expr::BinaryOp {
                        op: BinaryOp::Pow,
                        left: left.clone(),
                        right: Box::new(Expr::Number(n - 1)),
                    }),
                }
            } else {
                panic!("General power rule not implemented");
            }
        }
    }
}
