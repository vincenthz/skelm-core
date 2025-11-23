use aho_corasick::{AhoCorasickBuilder, PatternID};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    /// {% %}
    Statement,
    /// {{ }}
    Expression,
    /// {# #}
    Comment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripStyle {
    Minus,
    Plus,
    None,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub block_type: BlockType,
    pub left_strip_style: StripStyle,
    pub right_strip_style: StripStyle,
    pub content: &'a str,
}

/// encode the following structure `B (A B)*` where B can be empty content
pub struct Interspersed<A, B> {
    pub first: B,
    pub found: Vec<(A, B)>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn block<'a>(jinja_template: &'a str) -> Interspersed<Block<'a>, &'a str> {
    let open_patterns = &["{{-", "{{+", "{{", "{%-", "{%+", "{%", "{#"];
    let close_patterns = &["-}}", "+}}", "}}", "-%}", "+%}", "%}", "#}"];

    assert_eq!(open_patterns.len(), close_patterns.len());

    fn block_type_pattern(pattern_id: PatternID) -> BlockType {
        match pattern_id.as_u32() {
            0 | 1 | 2 => BlockType::Expression,
            3 | 4 | 5 => BlockType::Statement,
            6 => BlockType::Comment,
            n => panic!("internal error: unexpected pattern id {}", n),
        }
    }

    fn strip_style_pattern(pattern_id: PatternID) -> StripStyle {
        match pattern_id.as_u32() {
            0 | 3 => StripStyle::Minus,
            1 | 4 => StripStyle::Plus,
            2 | 5 | 6 => StripStyle::None,
            n => panic!("internal error: unexpected strip style {}", n),
        }
    }

    let ac_opens = AhoCorasickBuilder::new()
        .match_kind(aho_corasick::MatchKind::LeftmostLongest)
        .build(open_patterns)
        .unwrap();
    let ac_closes = AhoCorasickBuilder::new()
        .match_kind(aho_corasick::MatchKind::LeftmostLongest)
        .build(close_patterns)
        .unwrap();

    let mut current = jinja_template;

    let Some(mut mat_open) = ac_opens.find(current) else {
        return Interspersed {
            first: current,
            found: vec![],
        };
    };
    let first = &current[0..mat_open.start()];
    let mut found = Vec::new();

    while !current.is_empty() {
        let content_block_start = &current[mat_open.end()..];
        let Some(mat_close) = ac_closes.find(content_block_start) else {
            panic!("unterminated block");
        };
        current = &content_block_start[mat_close.end()..];
        let block_type = block_type_pattern(mat_open.pattern());
        let left_strip_style = strip_style_pattern(mat_open.pattern());
        let right_strip_style = strip_style_pattern(mat_close.pattern());
        let content = &content_block_start[..mat_close.start()];

        if block_type_pattern(mat_close.pattern()) != block_type {
            // do something bad
        };

        let block = Block {
            block_type,
            left_strip_style,
            right_strip_style,
            content,
        };

        // if we find a next opening, we re-iterate the loop, otherwise we just
        if let Some(mat_open_next) = ac_opens.find(current) {
            let next_block_content = &current[0..mat_open.start()];
            found.push((block, next_block_content));
            mat_open = mat_open_next;
        } else {
            found.push((block, current));
            current = &"";
        }
    }

    Interspersed { first, found }
}
