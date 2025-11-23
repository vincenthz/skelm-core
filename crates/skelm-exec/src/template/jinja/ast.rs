#[derive(Debug)]
pub enum StatementAst {
    If {
        condition: Expression,
        then: (),
        else_: Option<()>,
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
