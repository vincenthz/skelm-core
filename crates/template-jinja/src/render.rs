use std::collections::HashMap;
use std::str::FromStr;

use super::StripStyle;

use super::{
    Expression,
    ast::{ArraySubexpr, Operator, StatementAst, StatementLeaf},
    block::Block,
};

#[derive(Clone)]
pub enum Value {
    String(String),
    Int(u64),
    Bool(bool),
    Array(Vec<Value>),
    Struct(Values),
}

pub type Values = HashMap<String, Value>;

pub struct ScopedValues(Vec<Values>);

impl ScopedValues {
    pub fn new() -> Self {
        Self(vec![HashMap::new()])
    }

    pub fn push_context(&mut self) {
        self.0.push(HashMap::new())
    }

    pub fn pop_context(&mut self) {
        self.0.pop();
    }

    pub fn push_value(&mut self, s: &str, v: Value) {
        let scope = self.0.last_mut().unwrap();
        scope.insert(s.to_string(), v);
    }

    pub fn get_value(&self, s: &str) -> Option<&Value> {
        for scope in self.0.iter().rev() {
            if let Some(value) = scope.get(s) {
                return Some(value);
            }
        }
        None
    }
}

pub struct Render<'a, 'b> {
    values: &'a Values,
    locals: ScopedValues,
    output: String,
    stmts: Vec<&'b [StatementAst<'b>]>,
    functions: HashMap<String, CallFn>,
}

type CallFn = Box<dyn Fn(&[Value]) -> Value>;

impl<'a, 'b> Render<'a, 'b> {
    pub fn new(template: &'b [StatementAst<'b>], values: &'a Values) -> Self {
        Self {
            values,
            locals: ScopedValues::new(),
            output: String::new(),
            stmts: vec![template],
            functions: HashMap::new(),
        }
    }

    pub fn push_str(&mut self, left: StripStyle, right: StripStyle, s: &str) {
        self.output.push_str(s)
    }

    pub fn eval_statements(&mut self, statements: &[StatementAst]) {
        for stmt in statements {
            match stmt {
                StatementAst::If {
                    conditions,
                    else_,
                    end,
                } => {
                    let mut do_else = true;
                    for (cond, cond_body) in conditions {
                        if eval_condition(&self, cond) {
                            do_else = false;
                            break;
                        }
                    }

                    if do_else {
                        if let Some(else_) = else_ {
                            //
                        }
                    }
                }
                StatementAst::For { binder, body, end } => {
                    let (name, expression) = binder.content;
                    let val = eval_expression(self, expression);
                    match val {
                        Some(val) => match val {
                            Value::String(_) => {}
                            Value::Int(_) => {}
                            Value::Bool(_) => {}
                            Value::Array(values) => {
                                for val in values {
                                    self.locals.push_context();
                                    self.locals.push_value(name, val);

                                    self.eval_statements(&body);

                                    self.locals.pop_context();
                                }
                            }
                            Value::Struct(hash_map) => todo!(),
                        },
                        None => {}
                    }
                }
                StatementAst::Block(statement_asts) => {
                    self.eval_statements(&statement_asts);
                }
                StatementAst::Leaf(leaf) => match leaf {
                    StatementLeaf::Set(expr_value_block) => {
                        let (e1, e2) = expr_value_block.content;
                        let lval = eval_lvalue(self, e1).unwrap();
                        let val = eval_expression(self, e2);
                        if let Some(val) = val {
                            match lval {
                                Lvalue::Identifier(identifier) => {
                                    self.locals.push_value(&identifier, val)
                                }
                                Lvalue::Subscript(lvalue, array_subexpr) => todo!(),
                                Lvalue::Member(lvalue, _) => todo!(),
                            }
                        }
                    }
                },
                StatementAst::Expression(block) => {
                    //
                    let val = eval_expression(&self, block.content);
                    if let Some(val) = val {
                        match val {
                            Value::String(s) => {
                                self.push_str(block.left_strip_style, block.right_strip_style, &s)
                            }
                            Value::Int(i) => self.push_str(
                                block.left_strip_style,
                                block.right_strip_style,
                                &i.to_string(),
                            ),
                            Value::Struct(_hash_map) => self.push_str(
                                block.left_strip_style,
                                block.right_strip_style,
                                &format!("[hashmap]"),
                            ),
                            Value::Bool(b) => self.push_str(
                                block.left_strip_style,
                                block.right_strip_style,
                                &format!("{}", b),
                            ),
                            Value::Array(_hash_map) => self.push_str(
                                block.left_strip_style,
                                block.right_strip_style,
                                &format!("[array]"),
                            ),
                        }
                    }
                }
                StatementAst::Comment(_block) => {}
                StatementAst::Text(t) => self.output.push_str(t),
            }
        }
    }
}

pub fn render(template: &[StatementAst], values: &Values) -> String {
    let mut render = Render::new(template, values);
    render.eval_statements(template);
    render.output
}

pub enum Lvalue {
    Identifier(String),
    Subscript(Box<Lvalue>, ArraySubexpr),
    Member(Box<Lvalue>, String),
}

fn eval_lvalue(render: &Render, expression: &Expression) -> Result<Lvalue, ()> {
    match expression {
        Expression::Identifier(identifier) => Ok(Lvalue::Identifier(identifier.clone())),
        Expression::String(_) => Err(()),
        Expression::Number(_) => Err(()),
        Expression::Boolean(_) => Err(()),
        Expression::ArraySubscript(expression, array_subexpr) => {
            let e = eval_lvalue(render, expression)?;
            Ok(Lvalue::Subscript(Box::new(e), array_subexpr.clone()))
        }
        Expression::MemberAccess(expression, member) => {
            let e = eval_lvalue(render, expression)?;
            Ok(Lvalue::Member(Box::new(e), member.clone()))
        }
        Expression::Call(_e, _args) => Err(()),
        Expression::Filter(_e1, _e2) => Err(()),
        Expression::LogicalAnd(_e1, _e2) => Err(()),
        Expression::LogicalOr(_e1, _e2) => Err(()),
        Expression::LogicalNot(_e) => Err(()),
        Expression::BinaryOperation { lhs, op, rhs } => Err(()),
    }
}

fn eval_expression(render: &Render, expression: &Expression) -> Option<Value> {
    match expression {
        Expression::Identifier(id) => {
            let val = render
                .locals
                .get_value(&id)
                .or_else(|| render.values.get(id));
            val.cloned()
        }
        Expression::String(s) => Some(Value::String(s.clone())),
        Expression::Number(n) => {
            let value = u64::from_str(&n).ok();
            value.map(Value::Int)
        }
        Expression::Boolean(b) => Some(Value::Bool(*b)),
        Expression::ArraySubscript(expression, array_subexpr) => {
            let val = eval_expression(render, expression);
            match array_subexpr {
                ArraySubexpr::Value(expression) => {
                    let subscript = eval_expression(render, expression);
                }
                ArraySubexpr::Range(_, _) => todo!(),
            }
            match val {
                Some(val) => {
                    match val {
                        Value::String(_) => None,
                        Value::Int(_) => None,
                        Value::Bool(_) => None,
                        Value::Array(values) => {
                            // TODO
                            None
                        }
                        Value::Struct(hash_map) => {
                            // TODO
                            None
                        }
                    }
                }
                None => None,
            }
        }
        Expression::MemberAccess(expression, field) => {
            let val = eval_expression(render, expression);
            let Some(Value::Struct(fields)) = val else {
                panic!("accessing non structure")
            };
            let val = fields.get(field);
            val.cloned()
        }
        Expression::Call(expression, items) => {
            let name = eval_call_expr(&expression).unwrap();
            match name.as_str() {
                "namespace" => {
                    let mut object = HashMap::new();
                    for (k, v) in items {
                        let e = eval_expression(render, v);
                        if let Some(val) = e {
                            object.insert(k.clone(), val);
                        }
                    }
                    Some(Value::Struct(object))
                }
                _ => {
                    panic!("call to {} not supported ({:?})", name, items)
                }
            }
        }
        Expression::Filter(e1, e2) => {
            let val = eval_expression(render, e1);
            if let Some(val) = val {
                eval_filter(render, val, e2).unwrap()
            } else {
                None
            }
        }
        Expression::LogicalAnd(e1, e2) => {
            let v1 = eval_expression(render, e1);
            let v2 = eval_expression(render, e2);
            let b1 = value_as_bool(&v1);
            if b1 {
                let b2 = value_as_bool(&v2);
                Some(Value::Bool(b2))
            } else {
                Some(Value::Bool(false))
            }
        }
        Expression::LogicalOr(e1, e2) => {
            let v1 = eval_expression(render, e1);
            let v2 = eval_expression(render, e2);
            let b1 = value_as_bool(&v1);
            if b1 {
                Some(Value::Bool(true))
            } else {
                let b2 = value_as_bool(&v2);
                Some(Value::Bool(b2))
            }
        }
        Expression::LogicalNot(exp) => {
            let v = eval_expression(render, exp);
            let b = value_as_bool(&v);
            Some(Value::Bool(b))
        }
        Expression::BinaryOperation { lhs, op, rhs } => match op {
            Operator::Add => todo!(),
            Operator::Sub => todo!(),
            Operator::Is => todo!(),
            Operator::In => todo!(),
            Operator::Equal => todo!(),
            Operator::NotEqual => todo!(),
            Operator::GreaterThan => todo!(),
            Operator::GreaterEqual => todo!(),
            Operator::LesserThan => todo!(),
            Operator::LesserEqual => todo!(),
        },
    }
    //
}

fn value_as_bool(value: &Option<Value>) -> bool {
    match value {
        None => false,
        Some(val) => match val {
            Value::String(s) => !s.is_empty(),
            Value::Int(i) => *i != 0,
            Value::Bool(b) => *b,
            Value::Struct(hash_map) => !hash_map.is_empty(),
            Value::Array(values) => !values.is_empty(),
        },
    }
}

fn eval_call_expr<'a>(expr: &'a Expression) -> Result<&'a String, ()> {
    match expr {
        Expression::Identifier(identifier) => Ok(identifier),
        Expression::String(_) => Err(()),
        Expression::Number(_) => Err(()),
        Expression::Boolean(_) => Err(()),
        Expression::ArraySubscript(_e, array_subexpr) => Err(()),
        Expression::MemberAccess(_e, _) => Err(()),
        Expression::Call(_e, items) => Err(()),
        Expression::Filter(_e1, _e2) => Err(()),
        Expression::LogicalAnd(_e1, _e2) => Err(()),
        Expression::LogicalOr(_e1, _e2) => Err(()),
        Expression::LogicalNot(_e) => Err(()),
        Expression::BinaryOperation { lhs, op, rhs } => Err(()),
    }
}

fn eval_condition(render: &Render, block: &Block<&Expression>) -> bool {
    let ret = eval_expression(render, block.content);
    value_as_bool(&ret)
}

fn eval_filter(render: &Render, val: Value, expr: &Expression) -> Result<Option<Value>, ()> {
    //
    match expr {
        Expression::Identifier(identifier) => {
            //
            todo!()
        }
        Expression::String(_) => Err(()),
        Expression::Number(_) => Err(()),
        Expression::Boolean(_) => Err(()),
        Expression::ArraySubscript(expression, array_subexpr) => todo!(),
        Expression::MemberAccess(expression, _) => todo!(),
        Expression::Call(expression, items) => todo!(),
        Expression::Filter(expression, expression1) => todo!(),
        Expression::LogicalAnd(expression, expression1) => todo!(),
        Expression::LogicalOr(expression, expression1) => todo!(),
        Expression::LogicalNot(expression) => todo!(),
        Expression::BinaryOperation { lhs, op, rhs } => todo!(),
    }
}

// filters
// replace by a compile-time hashtable
const BUILTIN_FILTERS: &[(&str, u32)] = &[
    ("abs", 10),
    ("attr", 10),
    ("batch", 10),
    ("capitalize", 10),
    ("center", 10),
    ("default", 10),
    ("dictsort", 10),
    ("escape", 10),
    ("filesizeformat", 10),
    ("first", 10),
    ("float", 10),
    ("forceescape", 10),
    ("format", 10),
    ("groupby", 10),
    ("indent", 10),
    ("int", 10),
    ("items", 10),
    ("join", 10),
    ("last", 10),
    ("length", 10),
    ("list", 10),
    ("lower", 10),
    ("map", 10),
    ("max", 10),
    ("min", 10),
    ("pprint", 10),
    ("random", 10),
    ("reject", 10),
    ("rejectattr", 10),
    ("replace", 10),
    ("reverse", 10),
    ("round", 10),
    ("safe", 10),
    ("select", 10),
    ("selectattr", 10),
    ("slice", 10),
    ("sort", 10),
    ("string", 10),
    ("striptags", 10),
    ("sum", 10),
    ("title", 10),
    ("tojson", 10),
    ("trim", 10),
    ("truncate", 10),
    ("unique", 10),
    ("upper", 10),
    ("urlencode", 10),
    ("urlize", 10),
    ("wordcount", 10),
    ("wordwrap", 10),
    ("xmlattr", 10),
];

const BUILTIN_TESTS: &[(&str)] = &[
    ("boolean"),
    ("callable"),
    ("defined"),
    ("divisibleby"),
    ("eq"),
    ("escaped"),
    ("even"),
    ("false"),
    ("filter"),
    ("float"),
    ("ge"),
    ("gt"),
    ("in"),
    ("integer"),
    ("iterable"),
    ("le"),
    ("lower"),
    ("lt"),
    ("mapping"),
    ("ne"),
    ("none"),
    ("number"),
    ("odd"),
    ("sameas"),
    ("sequence"),
    ("string"),
    ("test"),
    ("true"),
    ("undefined"),
    ("upper"),
];
