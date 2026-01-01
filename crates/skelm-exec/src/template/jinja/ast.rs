use super::block::Block;
use super::parser::{ParseToken, Statement};

#[derive(Debug)]
pub enum StatementAst {
    If {
        conditions: Vec<(Expression, StatementBlock)>,
        else_: Option<()>,
    },
    For {
        binder: Binder,
        iterator: Expression,
        body: StatementBlock,
    },
    Expression(Expression),
    Statement(Statement),
}

pub type StatementBlock = Vec<StatementAst>;

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

pub enum AstBuilder {
    If {
        conditions: Vec<(Expression, StatementBlock)>,
    },
    IfElse {
        conditions: Vec<(Expression, StatementBlock)>,
        else_: StatementBlock,
    },
    For {
        binder: Binder,
        body: Box<AstBuilder>,
    },
}

pub fn ast<'a>(blocks: &[Block<ParseToken<'a>>]) -> Vec<StatementAst> {
    let mut stack: Vec<StatementBlock> = Vec::new();

    let mut blocks = blocks.iter();
    while let Some(block) = blocks.next() {
        match &block.content {
            ParseToken::Statement(statement) => {
                match statement {
                    Statement::If { condition } => todo!(),
                    Statement::Elif { condition } => todo!(),
                    Statement::For { bind, iter } => todo!(),
                    Statement::Set { expr, value } => todo!(),
                    Statement::EndIf => todo!(),
                    Statement::Else => todo!(),
                    Statement::EndFor => todo!(),
                }
                //
                todo!()
            }
            ParseToken::Expression(expression) => {
                //
                todo!()
            }
            ParseToken::Comment(_) => todo!(),
            ParseToken::Text(_) => todo!(),
        }
        //
    }
    todo!()
}
