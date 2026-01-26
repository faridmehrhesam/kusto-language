use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperatorKind {
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxKind {
    QueryBlock(Vec<SyntaxKind>, Vec<SyntaxKind>),
    Directive(String),

    LetStatement(Box<SyntaxKind>, Box<SyntaxKind>),

    CountOperator(Box<Option<SyntaxKind>>),
    CountAsIdentifierClause(Box<SyntaxKind>),

    NameDeclaration(Box<SyntaxKind>),
    BracketedName(Box<SyntaxKind>),

    StringLiteral(String),
    CompoundStringLiteral(Vec<String>),

    Identifier(String),
}
