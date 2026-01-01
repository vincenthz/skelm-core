use super::*;

const STRING1: &str = r#"{% if variable %}
{{- "hello" + variable }}
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
