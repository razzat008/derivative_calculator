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

impl Expr {
    /// Recursively simplify the AST.
    /// - Combines all constant multiplications (e.g., 3*4*x^2 -> 12*x^2)
    /// - Handles basic arithmetic simplifications (0, 1, etc.)
    pub fn simplify(&self) -> Expr {
        match self {
            // Multiplication: recursively combine all constant factors
            Expr::BinaryOp {
                op: BinaryOp::Mul,
                left,
                right,
            } => {
                // Helper to flatten multiplication tree and collect constants
                fn flatten_mul(expr: &Expr, constants: &mut i32, others: &mut Vec<Expr>) {
                    match expr {
                        Expr::BinaryOp {
                            op: BinaryOp::Mul,
                            left,
                            right,
                        } => {
                            flatten_mul(left, constants, others);
                            flatten_mul(right, constants, others);
                        }
                        Expr::Number(n) => *constants *= *n,
                        other => others.push(other.clone()),
                    }
                }
                let mut const_product = 1;
                let mut others = Vec::new();
                flatten_mul(&left.simplify(), &mut const_product, &mut others);
                flatten_mul(&right.simplify(), &mut const_product, &mut others);

                // If any factor is zero, the whole product is zero
                if const_product == 0 || others.iter().any(|e| matches!(e, Expr::Number(0))) {
                    return Expr::Number(0);
                }

                // Remove all 1s from others (except if all are removed)
                others.retain(|e| !matches!(e, Expr::Number(1)));

                // If only constant remains
                if others.is_empty() {
                    return Expr::Number(const_product);
                }
                // If constant is 1, just multiply the rest
                if const_product == 1 && others.len() == 1 {
                    return others.into_iter().next().unwrap();
                }
                // If constant is not 1, prepend it
                let mut expr = if const_product != 1 {
                    Expr::BinaryOp {
                        op: BinaryOp::Mul,
                        left: Box::new(Expr::Number(const_product)),
                        right: Box::new(others[0].clone()),
                    }
                } else {
                    others[0].clone()
                };
                for other in &others[1..] {
                    expr = Expr::BinaryOp {
                        op: BinaryOp::Mul,
                        left: Box::new(expr),
                        right: Box::new(other.clone()),
                    };
                }
                expr
            }
            Expr::BinaryOp {
                op: BinaryOp::Add,
                left,
                right,
            } => {
                let left = left.simplify();
                let right = right.simplify();
                match (&left, &right) {
                    (Expr::Number(0), r) => r.clone(),
                    (l, Expr::Number(0)) => l.clone(),
                    (Expr::Number(a), Expr::Number(b)) => Expr::Number(a + b),
                    _ => Expr::BinaryOp {
                        op: BinaryOp::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }
            Expr::BinaryOp {
                op: BinaryOp::Sub,
                left,
                right,
            } => {
                let left = left.simplify();
                let right = right.simplify();
                match (&left, &right) {
                    (l, Expr::Number(0)) => l.clone(),
                    (Expr::Number(a), Expr::Number(b)) => Expr::Number(a - b),
                    _ => Expr::BinaryOp {
                        op: BinaryOp::Sub,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }
            Expr::BinaryOp {
                op: BinaryOp::Div,
                left,
                right,
            } => {
                let left = left.simplify();
                let right = right.simplify();
                match (&left, &right) {
                    (Expr::Number(0), _) => Expr::Number(0),
                    (l, Expr::Number(1)) => l.clone(),
                    (Expr::Number(a), Expr::Number(b)) if *b != 0 => Expr::Number(a / b),
                    _ => Expr::BinaryOp {
                        op: BinaryOp::Div,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }
            Expr::BinaryOp {
                op: BinaryOp::Pow,
                left,
                right,
            } => {
                let left = left.simplify();
                let right = right.simplify();
                match (&left, &right) {
                    (_, Expr::Number(0)) => Expr::Number(1),
                    (l, Expr::Number(1)) => l.clone(),
                    (Expr::Number(a), Expr::Number(b)) => Expr::Number(a.pow(*b as u32)),
                    _ => Expr::BinaryOp {
                        op: BinaryOp::Pow,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }
            Expr::UnaryOp {
                op: UnaryOp::Neg,
                expr,
            } => {
                let expr = expr.simplify();
                if let Expr::Number(n) = expr {
                    Expr::Number(-n)
                } else {
                    Expr::UnaryOp {
                        op: UnaryOp::Neg,
                        expr: Box::new(expr),
                    }
                }
            }
            _ => self.clone(),
        }
    }

    pub fn pretty(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Variable(c) => c.to_string(),
            Expr::Constant(c) => c.to_string(),
            Expr::UnaryOp {
                op: UnaryOp::Neg,
                expr,
            } => format!("-{}", expr.pretty()),
            Expr::BinaryOp {
                op: BinaryOp::Mul,
                left,
                right,
            } => {
                // Omit * between number and variable/power
                match (&**left, &**right) {
                    (Expr::Number(_), Expr::Variable(_))
                    | (
                        Expr::Number(_),
                        Expr::BinaryOp {
                            op: BinaryOp::Pow, ..
                        },
                    )
                    | (Expr::Variable(_), Expr::Number(_)) => {
                        format!("{}{}", left.pretty(), right.pretty())
                    }
                    _ => format!("{}*{}", left.pretty(), right.pretty()),
                }
            }
            Expr::BinaryOp {
                op: BinaryOp::Pow,
                left,
                right,
            } => {
                format!("{}^{}", left.pretty(), right.pretty())
            }
            Expr::BinaryOp {
                op: BinaryOp::Add,
                left,
                right,
            } => {
                format!("{}+{}", left.pretty(), right.pretty())
            }
            Expr::BinaryOp {
                op: BinaryOp::Sub,
                left,
                right,
            } => {
                format!("{}-{}", left.pretty(), right.pretty())
            }
            Expr::BinaryOp {
                op: BinaryOp::Div,
                left,
                right,
            } => {
                format!("{}/{}", left.pretty(), right.pretty())
            }
        }
    }
}
