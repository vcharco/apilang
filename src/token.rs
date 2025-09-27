use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    KeywordGet,
    KeywordPost,
    KeywordPatch,
    KeywordPut,
    KeywordDelete,
    KeywordOptions,
    KeywordHead,
    ParenOpen,
    ParenClose,
    Equal,
    Comma,
    Colon,
    Plus,
    Number(f64),
    String,
    Identifier(String),
    Indent,
    Dedent,
    Eof,
    Illegal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}
