use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum BinOpKind {
    Multiply,
    Divide,
    Modulo,
    Add,
    Subtract,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxKind {
    Expr(ExprKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Literal(LitExprKind),
    BinOp {
        left: Box<ExprKind>,
        op: BinOpKind,
        right: Box<ExprKind>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum LitExprKind {
    Boolean(bool),
    Long(i64),
    Real(f64),
}
