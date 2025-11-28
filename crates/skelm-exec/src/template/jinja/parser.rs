use logos::Logos;

use super::ast::Expression;
use super::block::{Block, BlockType};
use super::lexer::*;
use super::position::Position;

#[derive(Debug)]
pub enum Statement {
    If { condition: Expression },
    Elif { condition: Expression },
    For { bind: String, iter: Expression },
    Set { expr: Expression, value: Expression },
    EndIf,
    Else,
    EndFor,
}

pub fn lexing(
    position: Position,
    statement: &str,
) -> Result<Vec<(Position, StatementToken, Position)>, (Position, LexerError, Position)> {
    let mut lexer = StatementToken::lexer(statement);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        let span = lexer.span();
        let mut start_pos = position;
        let mut end_pos = position;
        start_pos.update_by(&statement[0..span.start]);
        end_pos.update_by(&statement[0..span.end]);

        match token {
            Err(e) => return Err((start_pos, e, end_pos)),
            Ok(token) => tokens.push((start_pos, token, end_pos)),
        }
    }

    Ok(tokens)
}

#[derive(Debug)]
pub enum ParseToken<'a> {
    Statement(Statement),
    Expression(Expression),
    Comment(&'a str),
    Text(&'a str),
}

pub fn parse<'a>(blocks: &[Block<&'a str>]) -> Result<Vec<Block<ParseToken<'a>>>, String> {
    blocks
        .into_iter()
        .map(|block| match block.block_type {
            BlockType::Statement => {
                let st = parse_statement(block.start_pos, block.content)?;
                Ok(block.map(ParseToken::Statement(st)))
            }
            BlockType::Expression => {
                let expr = parse_expression(block.start_pos, block.content)?;
                Ok(block.map(ParseToken::Expression(expr)))
            }
            BlockType::Comment => Ok(block.map(ParseToken::Comment(block.content))),
            BlockType::Text => Ok(block.map(ParseToken::Text(block.content))),
        })
        .collect::<Result<Vec<_>, _>>()
    /*
    for block in blocks {
        //println!("block: {:?}", block);
        if block.block_type == BlockType::Statement {
            println!("  {:?}", st)
        } else if block.block_type == BlockType::Expression {
            let st = parse_expression(block.start_pos, block.content).unwrap();
            println!("  {:?}", st)
        }
    }
    */
}

pub fn parse_statement(position: Position, statement: &str) -> Result<Statement, String> {
    let tokens = lexing(position, statement).map_err(|(start, lex_error, end)| {
        format!("lexing error {:?} at {:?} {:?}", lex_error, start, end)
    })?;
    let statement = super::grammar::StatementParser::new().parse(tokens);

    statement.map_err(|e| format!("parsing error {:?}", e))
}

pub fn parse_expression(position: Position, expression: &str) -> Result<Expression, String> {
    let tokens = lexing(position, expression).map_err(|(start, lex_error, end)| {
        format!("lexing error {:?} at {:?} {:?}", lex_error, start, end)
    })?;
    let expression = super::grammar::ExpressionParser::new().parse(tokens);
    expression.map_err(|e| format!("parsing error {:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let r =
        //    parse_statement("set ns = namespace(multi_step_tool=true, last_query_index=messages)");
        parse_statement(Position::default(), "if ns.multi_step_tool and message.role == \"user\" and not(message.content.startswith('<tool_response>') and message.content.endswith('</tool_response>'))");
        match r {
            Err(r) => panic!("failed: {}", r),
            Ok(e) => println!("{:?}", e),
        }
    }
}
