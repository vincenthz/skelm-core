use std::collections::HashMap;
use std::str::FromStr;

use crate::template::jinja::StripStyle;

use super::{
    Expression,
    ast::{Operator, StatementAst},
    block::Block,
};

#[derive(Clone)]
pub enum Value {
    String(String),
    Int(u64),
    Bool(bool),
    Field(Values),
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
}

pub fn render(template: &[StatementAst], values: &Values) -> String {
    let mut render = Render::new(template, values);

    for stmt in template {
        match stmt {
            StatementAst::If {
                conditions,
                else_,
                end,
            } => {
                let mut do_else = true;
                for (cond, cond_body) in conditions {
                    if eval_condition(&render, cond) {
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
            StatementAst::For { binder, body, end } => {}
            StatementAst::Block(statement_asts) => {}
            StatementAst::Expression(block) => {
                //
                let val = eval_expression(&render, block.content);
                if let Some(val) = val {
                    match val {
                        Value::String(s) => {
                            render.push_str(block.left_strip_style, block.right_strip_style, &s)
                        }
                        Value::Int(i) => render.push_str(
                            block.left_strip_style,
                            block.right_strip_style,
                            &i.to_string(),
                        ),
                        Value::Field(_hash_map) => render.push_str(
                            block.left_strip_style,
                            block.right_strip_style,
                            &format!("[hashmap]"),
                        ),
                        Value::Bool(b) => render.push_str(
                            block.left_strip_style,
                            block.right_strip_style,
                            &format!("{}", b),
                        ),
                    }
                }
            }
            StatementAst::Comment(_block) => {}
            StatementAst::Text(t) => render.output.push_str(t),
        }
    }
    //let mut local = HashMap::new();
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
        Expression::ArraySubscript(expression, array_subexpr) => todo!(),
        Expression::MemberAccess(expression, _) => todo!(),
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
    false
}

fn eval_condition(render: &Render, block: &Block<&Expression>) -> bool {
    false
}
