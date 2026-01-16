use crate::template::jinja::render::Value;

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
