use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \t\n]+")]
#[logos(error = String)]
pub enum StatementToken {
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("[")]
    BrackOpen,
    #[token("]")]
    BrackClose,
    #[token("if")]
    If,
    #[token("endif")]
    EndIf,
    #[token("elif")]
    Elif,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("or")]
    Or,
    #[token("and")]
    And,
    #[token("not")]
    Not,
    #[token("set")]
    Set,
    #[token("is")]
    Is,
    #[token("in")]
    In,
    #[token("endfor")]
    EndFor,
    #[token("|")]
    Pipe,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("=")]
    Assign,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("::")]
    DDColon,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r"-?(?:0|[1-9]\d*)?", |lex| lex.slice().to_owned())]
    Number(String),
    #[regex(r#"[_a-zA-Z][-_a-zA-Z0-9]*"#, |lex| lex.slice().to_owned())]
    Ident(String),
    #[token("\"", string_double_quoted)]
    String(String),
    #[token("\'", string_single_quoted)]
    SString(String),
}

fn string_double_quoted(lex: &mut logos::Lexer<StatementToken>) -> Option<String> {
    lex_string(lex, b'"')
}

fn string_single_quoted(lex: &mut logos::Lexer<StatementToken>) -> Option<String> {
    lex_string(lex, b'\'')
}

fn lex_string(lex: &mut logos::Lexer<StatementToken>, sep: u8) -> Option<String> {
    let slice = lex.remainder().as_bytes();
    let mut i = 0;
    let mut escape = false;
    loop {
        if slice[i] == sep {
            if !escape {
                break;
            }
        }
        if !escape && slice[i] == b'\\' {
            escape = true;
        } else {
            escape = false;
        }

        i += 1
    }
    lex.bump(i + 1);
    Some(std::str::from_utf8(&slice[0..i]).unwrap().to_string())
}
