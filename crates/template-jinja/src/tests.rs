use std::collections::HashMap;

use super::render::Value;

use super::*;

const STRING1: &str = r#"{% if variable %}
{{- "hello" + variable }}
{% endif %}"#;
const STRING2: &str = r#"{% if variable > variable2 %}
{{- "hello" + variable }}
{% else %}
{{- "else" + variable -}}
{% endif %}"#;

#[test]
fn block_works() {
    let r = block(STRING1).unwrap();
    macro_rules! assert_content {
        ($id:expr, $bty:expr, $content:literal) => {
            assert_eq!($id.block_type, $bty);
            assert_eq!($id.content, $content);
        };
    }
    assert_eq!(r.len(), 5);
    assert_content!(r[0], BlockType::Statement, " if variable ");
    assert_content!(r[1], BlockType::Text, "\n");
    assert_content!(r[2], BlockType::Expression, " \"hello\" + variable ");
    assert_content!(r[3], BlockType::Text, "\n");
    assert_content!(r[4], BlockType::Statement, " endif ");
}

#[test]
fn ast_works1() {
    let r = block(STRING1).unwrap();
    let syn = parse(&r).unwrap();
    let ast = ast(&syn);

    println!("{:?}", ast);
    assert_eq!(ast.len(), 1);

    match &ast[0] {
        ast::StatementAst::If {
            conditions,
            else_,
            end,
        } => {
            assert_eq!(conditions.len(), 1);
            assert!(else_.is_none());
            assert!(end.is_some());
        }
        ast => {
            panic!("expect if, got something else : {:?}", ast)
        }
    }
}

#[test]
fn ast_works2() {
    let r = block(STRING2).unwrap();
    let syn = parse(&r).unwrap();
    let ast = ast(&syn);

    println!("{:?}", ast);
    assert_eq!(ast.len(), 1);

    match &ast[0] {
        ast::StatementAst::If {
            conditions,
            else_,
            end,
        } => {
            assert_eq!(conditions.len(), 1);
            assert!(else_.is_some());
            assert!(end.is_some());
        }
        ast => {
            panic!("expect if, got something else : {:?}", ast)
        }
    }
}

#[test]
fn render_works() {
    const TEMPLATE: &str = r#"Hello {{ name }}!
Welcome to {{ company }}."#;

    let mut values = Values::new();
    values.insert(String::from("name"), Value::String(String::from("Alice")));
    values.insert(
        String::from("company"),
        Value::String(String::from("Acme Corp")),
    );

    let r = block(TEMPLATE).unwrap();
    let syn = parse(&r).unwrap();
    let ast = ast(&syn);

    let rendered = render(&ast, &values);

    assert_eq!(
        &rendered,
        r#"Hello Alice!
Welcome to Acme Corp."#
    )
}

#[test]
fn golden() {
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

    fn golden_render(template: &str, values: serde_json::Value, expected: &str) {
        let r = block(template).unwrap();
        let syn = parse(&r).unwrap();
        let ast = ast(&syn);

        let serde_json::Value::Object(object) = values else {
            panic!("expecting a golden test values to be an object")
        };

        let values = object
            .iter()
            .map(|(k, v)| (k.clone(), value_from_json(v)))
            .collect::<HashMap<_, _>>();

        let rendered = render(&ast, &values);

        assert_eq!(expected, rendered.trim());
    }

    macro_rules! test {
        ($name:literal, $number:literal) => {
            golden_render(
                include_str!(concat!("../tests-files/", $name, ".j2")),
                serde_json::from_str(include_str!(concat!(
                    "../tests-files/",
                    $name,
                    "_",
                    $number,
                    ".json"
                )))
                .unwrap(),
                include_str!(concat!("../tests-files/", $name, "_", $number, ".output")),
            );
        };
    }

    test!("hello", 1);
    test!("users", 1);
}
