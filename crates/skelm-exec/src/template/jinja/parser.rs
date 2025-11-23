use logos::Logos;

use super::ast::Expression;
use super::lexer::*;

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

pub fn parse_statement(statement: &str) -> Result<Statement, String> {
    let statement = statement.trim();
    let lexer = StatementToken::lexer(statement);
    let tokens = lexer
        .collect::<Result<Vec<StatementToken>, String>>()
        .map_err(|e| format!("Lexing error {:?}", e))?;

    let statement = super::grammar::StatementParser::new().parse(tokens.clone());

    statement.map_err(|e| format!("parsing error {:?}\n{:?}", e, tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let r =
        //    parse_statement("set ns = namespace(multi_step_tool=true, last_query_index=messages)");
        parse_statement("if ns.multi_step_tool and message.role == \"user\" and not(message.content.startswith('<tool_response>') and message.content.endswith('</tool_response>'))");
        match r {
            Err(r) => panic!("failed: {}", r),
            Ok(e) => println!("{:?}", e),
        }
    }
}
