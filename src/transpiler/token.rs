use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    KeywordGet,
    KeywordPost,
    KeywordPatch,
    KeywordPut,
    KeywordDelete,
    KeywordOptions,
    KeywordHead,
    KeywordModel,

    // Types
    KeywordString,
    KeywordNumber,
    KeywordBoolean,
    KeywordList,

    // Validation
    KeywordMin,
    KeywordMax,
    KeywordEmail,
    KeywordPhone,
    KeywordRegex,

    // Modifiers
    KeywordPrivate,
    KeywordPublic,
    KeywordCache,
    KeywordEvict,
    KeywordThrottle,
    KeywordConfigure,

    // Operators and delimiters
    ParenOpen,
    ParenClose,
    SquareBracketOpen,
    SquareBracketClose,
    Equal,
    Comma,
    Colon,
    Plus,

    // Data
    Identifier(String),
    Number(f64),
    String(String),

    // Indentation
    Indent,
    Dedent,

    // Errors and EOF
    Eof,
    Illegal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}
