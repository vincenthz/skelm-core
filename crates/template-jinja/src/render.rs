use std::collections::HashMap;
use std::str::FromStr;

use super::StripStyle;

use super::{
    Expression,
    ast::{ArraySubexpr, Operator, StatementAst},
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
}

impl<'a, 'b> Render<'a, 'b> {
    pub fn new(template: &'b [StatementAst<'b>], values: &'a Values) -> Self {
        Self {
            values,
            locals: ScopedValues::new(),
            output: String::new(),
            stmts: vec![template],
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
        Expression::Call(expression, items) => todo!(),
        Expression::Filter(expression, expression1) => todo!(),
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

fn eval_condition(render: &Render, block: &Block<&Expression>) -> bool {
    let ret = eval_expression(render, block.content);
    value_as_bool(&ret)
}
