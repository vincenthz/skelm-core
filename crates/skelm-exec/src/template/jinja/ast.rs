use super::block::Block;
use super::parser::{ParseToken, Statement};

#[derive(Debug)]
pub enum StatementAst<'a> {
    If {
        conditions: Vec<(Block<&'a Expression>, Vec<StatementAst<'a>>)>,
        else_: Option<()>,
    },
    For {
        binder: Block<(Binder, &'a Expression)>,
        body: Vec<StatementAst<'a>>,
    },
    Block(Vec<StatementAst<'a>>),
    Expression(Block<&'a Expression>),
    Comment(Block<&'a str>),
    Text(&'a str),
}

impl<'a> StatementAst<'a> {
    pub fn content_push(&mut self, element: StatementAst<'a>) {
        match self {
            StatementAst::If { conditions, else_ } => match else_ {
                Some(_) => todo!(),
                None => {
                    let (_, last_block) = conditions.last_mut().unwrap();
                    last_block.push(element)
                }
            },
            StatementAst::For { binder: _, body } => {
                todo!()
            }
            StatementAst::Block(statement_asts) => statement_asts.push(element),
            StatementAst::Expression(_) | StatementAst::Comment(_) | StatementAst::Text(_) => {
                // invalid pushing to a leaf
                todo!()
            }
        }
    }
}

//pub type StatementBlock<'a> = Vec<Block<StatementAst<'a>>>;

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

//pub struct Stack1<T>(T, Vec<T>);

/*
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
    Leaf(),
}

impl<'a> AstBuilder<'a> {
    pub fn append_expr(&mut self, expression: &'a Expression) {
        match self {
            AstBuilder::IfElse { conditions, else_ } => todo!(),
            AstBuilder::For { binder, body } => todo!(),
            AstBuilder::If { conditions } => todo!(),
            AstBuilder::Leaf() => todo!(),
        }
    }
}
*/

pub fn ast<'a>(blocks: &'a [Block<ParseToken<'a>>]) -> Vec<StatementAst<'a>> {
    let mut stack: Vec<StatementAst<'a>> = Vec::new();

    stack.push(StatementAst::Block(vec![]));

    let mut blocks = blocks.iter();
    while let Some(block) = blocks.next() {
        match &block.content {
            ParseToken::Statement(statement) => match statement {
                Statement::If { condition } => {
                    let cond = block.replace_content(condition);
                    let new_leaf = StatementAst::If {
                        conditions: vec![(cond, vec![])],
                        else_: None,
                    };
                    stack.push(new_leaf);
                }
                Statement::Elif { condition } => todo!(),
                Statement::For { bind, iter } => todo!(),
                Statement::Set { expr, value } => todo!(),
                Statement::EndIf => {
                    let ast_if = stack.pop().unwrap();
                    match ast_if {
                        StatementAst::If { conditions, else_ } => (),
                        _ => {
                            // invalid
                            todo!()
                        }
                    };
                    stack.last_mut().unwrap().content_push(ast_if);
                }
                Statement::Else => todo!(),
                Statement::EndFor => todo!(),
            },
            ParseToken::Expression(expression) => {
                stack
                    .last_mut()
                    .unwrap()
                    .content_push(StatementAst::Expression(block.replace_content(expression)));
            }
            ParseToken::Comment(comment) => {
                stack
                    .last_mut()
                    .unwrap()
                    .content_push(StatementAst::Comment(block.replace_content(comment)));
            }
            ParseToken::Text(text) => {
                stack
                    .last_mut()
                    .unwrap()
                    .content_push(StatementAst::Text(text));
            }
        }
    }

    let root = stack.pop().unwrap();
    if !stack.is_empty() {
        panic!("invalid")
    }

    let StatementAst::Block(b) = root else {
        panic!("invalid")
    };
    b
}
