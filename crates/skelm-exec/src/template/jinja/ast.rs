use super::block::Block;
use super::parser::{ParseToken, Statement};

#[derive(Debug)]
pub enum StatementAst<'a> {
    If {
        conditions: Vec<(Expression, StatementBlock<'a>)>,
        else_: Option<()>,
    },
    For {
        binder: Binder,
        iterator: Expression,
        body: StatementBlock<'a>,
    },
    Expression(&'a Expression),
    Statement(Statement),
}

pub type StatementBlock<'a> = Vec<Block<StatementAst<'a>>>;

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

pub enum AstBuilder<'a> {
    If {
        conditions: Vec<(&'a Expression, StatementBlock<'a>)>,
    },
    IfElse {
        conditions: Vec<(&'a Expression, StatementBlock<'a>)>,
        else_: StatementBlock<'a>,
    },
    For {
        binder: &'a Binder,
        body: StatementBlock<'a>,
    },
}

impl<'a> AstBuilder<'a> {}

pub fn ast<'a>(blocks: &[Block<ParseToken<'a>>]) -> Vec<Block<StatementAst<'a>>> {
    let mut stack: Vec<AstBuilder<'_>> = Vec::new();

    let mut blocks = blocks.iter();
    while let Some(block) = blocks.next() {
        match &block.content {
            ParseToken::Statement(statement) => {
                match statement {
                    Statement::If { condition } => {
                        // push a new builder
                        stack.push(AstBuilder::If {
                            conditions: vec![(condition, vec![])],
                        });
                    }
                    Statement::Elif { condition } => {
                        //
                        match stack.last_mut() {
                            Some(AstBuilder::If { conditions }) => {
                                conditions.push((condition, vec![]))
                            }
                            Some(_) => {
                                // elif is a non if context, TODO create an error
                                todo!()
                            }
                            None => {
                                // should not happen
                                todo!()
                            }
                        };
                    }
                    Statement::For { bind, iter } => stack.push(AstBuilder::For {
                        binder: bind,
                        body: vec![],
                    }),
                    Statement::Set { expr, value } => todo!(),
                    Statement::EndIf => todo!(),
                    Statement::Else => todo!(),
                    Statement::EndFor => todo!(),
                }
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
