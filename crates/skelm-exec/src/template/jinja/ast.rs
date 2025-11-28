use super::block::Block;
use super::parser::{ParseToken, Statement};

#[derive(Debug)]
pub enum StatementAst {
    If {
        condition: Expression,
        then: (),
        else_: Option<()>,
    },
    For {
        binder: Binder,
        iterator: Expression,
        body: Vec<StatementAst>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Is,
    In,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LesserThan,
    LesserEqual,
}

// TODO support multi binds
pub type Binder = String;

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    String(String),
    Number(String),
    Boolean(bool),
    ArraySubscript(Box<Expression>, ArraySubexpr),
    MemberAccess(Box<Expression>, String),
    Call(Box<Expression>, Vec<(String, Expression)>),
    Filter(Box<Expression>, Box<Expression>),
    LogicalAnd(Box<Expression>, Box<Expression>),
    LogicalOr(Box<Expression>, Box<Expression>),
    LogicalNot(Box<Expression>),
    BinaryOperation {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum ArraySubexpr {
    Value(Box<Expression>),
    Range(Option<String>, Option<String>),
}

pub fn ast<'a>(blocks: &[Block<ParseToken<'a>>]) -> Vec<StatementAst> {
    todo!()
}
