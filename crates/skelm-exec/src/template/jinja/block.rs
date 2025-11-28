use super::position::Position;
use aho_corasick::{AhoCorasickBuilder, PatternID};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    /// {% %}
    Statement,
    /// {{ }}
    Expression,
    /// {# #}
    Comment,
    /// normal text
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripStyle {
    Minus,
    Plus,
    None,
}

#[derive(Debug)]
pub struct Block<T> {
    pub block_type: BlockType,
    pub start_pos: Position,
    pub end_pos: Position,
    pub left_strip_style: StripStyle,
    pub right_strip_style: StripStyle,
    pub content: T,
}

impl<'a> Block<&'a str> {
    pub fn text(start_pos: Position, content: &'a str) -> Self {
        let mut end_pos = start_pos;
        end_pos.update_by(content);
        Self {
            block_type: BlockType::Text,
            start_pos,
            end_pos,
            left_strip_style: StripStyle::None,
            right_strip_style: StripStyle::None,
            content,
        }
    }
}

impl<T> Block<T> {
    pub fn map<U>(&self, content: U) -> Block<U> {
        Block {
            block_type: self.block_type,
            start_pos: self.start_pos,
            end_pos: self.end_pos,
            left_strip_style: self.left_strip_style,
            right_strip_style: self.right_strip_style,
            content,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Mismatch types
    #[error("Mismatch block types, open type {start:?} but closing {end:?}")]
    MismatchBlock { start: BlockType, end: BlockType },
    /// Unterminated block but reach end of content
    #[error("Unterminated block type {block_type:?} {position:?}")]
    UnterminatedBlock {
        block_type: BlockType,
        position: Position,
    },
}

pub fn block<'a>(jinja_template: &'a str) -> Result<Vec<Block<&'a str>>, Error> {
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

    let mut position = Position::new();
    let mut current = jinja_template;

    let Some(mut mat_open) = ac_opens.find(current) else {
        if current.is_empty() {
            return Ok(vec![]);
        }
        return Ok(vec![Block::text(position, current)]);
    };

    let first = &current[0..mat_open.start()];
    let mut blocks = Vec::new();
    if !first.is_empty() {
        blocks.push(Block::text(position, first));
        position.update_by(first);
    }

    while !current.is_empty() {
        let block_type = block_type_pattern(mat_open.pattern());
        let content_block_start = &current[mat_open.end()..];

        let Some(mat_close) = ac_closes.find(content_block_start) else {
            return Err(Error::UnterminatedBlock {
                block_type,
                position,
            });
        };
        position.add_col(mat_open.end() as u32);
        current = &content_block_start[mat_close.end()..];

        let left_strip_style = strip_style_pattern(mat_open.pattern());
        let right_strip_style = strip_style_pattern(mat_close.pattern());

        let content = &content_block_start[..mat_close.start()];

        if block_type_pattern(mat_close.pattern()) != block_type {
            return Err(Error::MismatchBlock {
                start: block_type,
                end: block_type_pattern(mat_close.pattern()),
            });
        };

        let start_pos = position;
        position.update_by(content);

        let block = Block {
            block_type,
            start_pos,
            end_pos: position,
            left_strip_style,
            right_strip_style,
            content,
        };
        blocks.push(block);

        position.add_col(2); // account for the closing block

        // if we find a next opening, we re-iterate the loop, otherwise we just finish the block'ification
        if let Some(mat_open_next) = ac_opens.find(current) {
            let next_block_content = &current[0..mat_open.start()];
            if !next_block_content.is_empty() {
                blocks.push(Block::text(position, next_block_content));
                position.update_by(next_block_content);
            }
            mat_open = mat_open_next;
        } else {
            if !current.is_empty() {
                blocks.push(Block::text(position, current));
            }
            current = &"";
        }
    }

    Ok(blocks)
}
