use super::block::Block;
use super::parser::{ParseToken, Statement};

#[derive(Debug)]
pub enum StatementAst<'a> {
    If {
        conditions: Vec<(Block<&'a Expression>, Vec<StatementAst<'a>>)>,
        else_: Option<(Block<()>, Vec<StatementAst<'a>>)>,
        end: Option<Block<()>>,
    },
    For {
        binder: Block<(&'a Binder, &'a Expression)>,
        body: Vec<StatementAst<'a>>,
        end: Option<Block<()>>,
    },
    Leaf(StatementLeaf<'a>),
    Block(Vec<StatementAst<'a>>),
    Expression(Block<&'a Expression>),
    Comment(Block<&'a str>),
    Text(&'a str),
}

#[derive(Debug)]
pub enum StatementLeaf<'a> {
    Set(Block<(&'a Expression, &'a Expression)>),
}

impl<'a> StatementAst<'a> {
    pub fn content_push(&mut self, element: StatementAst<'a>) {
        match self {
            StatementAst::If {
                conditions,
                else_,
                end,
            } => {
                if end.is_some() {
                    panic!("trying to push into a finished if")
                }
                match else_ {
                    Some((_, statements)) => statements.push(element),
                    None => {
                        let (_, last_block) = conditions.last_mut().unwrap();
                        last_block.push(element)
                    }
                }
            }
            StatementAst::For {
                binder: _,
                body,
                end,
            } => {
                if end.is_some() {
                    panic!("trying to push into a finished for");
                }
                body.push(element);
                //todo!()
            }
            StatementAst::Block(statement_asts) => statement_asts.push(element),
            StatementAst::Leaf(leaf) => {
                todo!()
            }
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ArraySubexpr {
    Value(Box<Expression>),
    Range(Option<String>, Option<String>),
}

pub fn ast<'a>(blocks: &'a [Block<ParseToken<'a>>]) -> Vec<StatementAst<'a>> {
    let mut stack: Vec<StatementAst<'a>> = Vec::new();

    stack.push(StatementAst::Block(vec![]));

    let mut blocks = blocks.iter();
    while let Some(block) = blocks.next() {
        match &block.content {
            ParseToken::Statement(statement) => {
                match statement {
                    Statement::If { condition } => {
                        let cond = block.replace_content(condition);
                        let new_leaf = StatementAst::If {
                            conditions: vec![(cond, vec![])],
                            else_: None,
                            end: None,
                        };
                        stack.push(new_leaf);
                    }
                    Statement::Elif { condition } => {
                        let cond = block.replace_content(condition);
                        match stack.last_mut().unwrap() {
                            StatementAst::If {
                                conditions,
                                else_,
                                end,
                            } => {
                                if end.is_some() {
                                    panic!("double bad");
                                }
                                if else_.is_some() {
                                    panic!("bad");
                                }
                                conditions.push((cond, vec![]))
                            }
                            _ => {
                                // invalid
                                todo!()
                            }
                        }
                    }
                    Statement::Else => {
                        match stack.last_mut().unwrap() {
                            StatementAst::If {
                                conditions: _,
                                else_,
                                end,
                            } => {
                                let else_block = block.replace_content(());
                                if end.is_some() {
                                    panic!("double bad");
                                }
                                if else_.is_some() {
                                    panic!("bad");
                                }

                                *else_ = Some((else_block, vec![]));
                            }
                            _ => {
                                // invalid
                                todo!()
                            }
                        }
                    }
                    Statement::EndIf => {
                        let mut ast_if = stack.pop().unwrap();
                        match &mut ast_if {
                            StatementAst::If {
                                conditions: _,
                                else_: _,
                                end,
                            } => {
                                *end = Some(block.replace_content(()));
                            }
                            _ => {
                                // invalid
                                todo!()
                            }
                        };
                        stack.last_mut().unwrap().content_push(ast_if);
                    }
                    Statement::Set { expr, value } => {
                        stack.last_mut().unwrap().content_push(StatementAst::Leaf(
                            StatementLeaf::Set(block.replace_content((expr, value))),
                        ));
                        //
                    }
                    Statement::For { bind, iter } => {
                        let binder = block.replace_content((bind, iter));
                        let new_leaf = StatementAst::For {
                            binder,
                            body: vec![],
                            end: None,
                        };
                        stack.push(new_leaf);
                    }
                    Statement::EndFor => {
                        let mut stmt = stack.pop().unwrap();
                        let StatementAst::For {
                            binder: _,
                            body: _,
                            end,
                        } = &mut stmt
                        else {
                            panic!("unexpected")
                        };
                        *end = Some(block.replace_content(()));
                        stack.last_mut().unwrap().content_push(stmt);
                    }
                }
            }
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
