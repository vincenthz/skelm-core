use std::path::PathBuf;
use std::{collections::HashMap, fs};

use clap::Parser;
use template_jinja::{BlockType, Value, ast, block, parse, render};

#[derive(Parser)]
pub struct Args {
    template: PathBuf,
    json: PathBuf,
    #[arg(short, long)]
    block: bool,
    #[arg(short, long)]
    parse: bool,
    #[arg(short, long)]
    ast: bool,
    #[arg(short, long)]
    render: bool,
}

fn value_from_json(val: &serde_json::Value) -> Value {
    match val {
        serde_json::Value::Null => panic!("null not supported"),
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(number) => Value::Int(number.as_u64().unwrap()),
        serde_json::Value::String(s) => Value::String(s.clone()),
        serde_json::Value::Array(values) => {
            Value::Array(values.iter().map(value_from_json).collect::<Vec<_>>())
        }
        serde_json::Value::Object(object) => {
            let values = object
                .iter()
                .map(|(k, v)| (k.clone(), value_from_json(v)))
                .collect::<HashMap<_, _>>();
            Value::Struct(values)
        }
    }
}

fn main() {
    let args = Args::parse();

    let template = fs::read_to_string(args.template).unwrap();
    let json = fs::read_to_string(args.json).unwrap();

    let blocks = block(&template).unwrap();

    if args.block {
        for block in &blocks {
            println!("{:?}", block)
        }
    }

    let parse = parse(&blocks).unwrap();

    if args.parse {
        for p in &parse {
            println!("{:?}", p)
        }
    }

    let ast = ast(&parse);

    if args.ast {
        for a in &ast {
            println!("{:?}", a)
        }
    }

    if args.render {
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let serde_json::Value::Object(object) = value else {
            panic!("value need to be an object")
        };

        let values = object
            .iter()
            .map(|(k, v)| (k.clone(), value_from_json(v)))
            .collect::<HashMap<_, _>>();

        let render = render(&ast, &values);
        println!("{}", render)
    }
}
