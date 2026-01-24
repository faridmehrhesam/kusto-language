use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum UnaryOperatorKind {
    Minus,
}

#[derive(Debug, Clone)]
pub enum SyntaxNode {
    BoolLiteral(bool),
    LongLiteral(i64),
    RealLiteral(f64),
    StringLiteral(String),
    PrefixUnaryExpression {
        operator: Option<UnaryOperatorKind>,
        expression: Box<SyntaxNode>,
    },
    JsonPair(Box<SyntaxNode>, Box<SyntaxNode>),
    JsonObjectExpression(Vec<SyntaxNode>),
    JsonArrayExpression(Vec<SyntaxNode>),
}
