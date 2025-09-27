// src/transpiler/errors.rs - SIMPLIFICADO
use crate::util::color;
use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    ReadFile(ReadFileError),
    Lexer(LexerError),
    Parser(ParseError),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::ReadFile(e) => e.fmt(f),
            CompilerError::Lexer(e) => e.fmt(f),
            CompilerError::Parser(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for CompilerError {} // Solo implementar Error estándar

#[derive(Debug)]
pub struct ReadFileError {
    pub file: String,
    pub error: String,
}

impl fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} Read file error at {}: {}",
            color::error("[ERROR]"),
            self.file,
            self.error,
        )
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub line: usize,
    pub file: String,
    pub error: String,
    pub line_content: String,
    pub char_index: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} Lexer error at {}, line {}: {}",
            color::error("[ERROR]"),
            self.file,
            self.line,
            self.error,
        )?;

        write!(f, "\n   └─> {}", self.line_content)?;
        let pointer = " ".repeat(self.char_index) + "^";
        write!(f, "\n       {}", color::bold(&pointer))
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub file: String,
    pub error: String,
    pub line_content: String,
    pub char_index: usize,
    pub expected: Option<String>,
    pub found: Option<crate::transpiler::Token>,
}

impl ParseError {
    pub fn new(
        line: usize,
        file: String,
        error: String,
        line_content: String,
        char_index: usize,
    ) -> Self {
        Self {
            line,
            file,
            error,
            line_content,
            char_index,
            expected: None,
            found: None,
        }
    }

    pub fn unexpected_token(
        line: usize,
        file: String,
        line_content: String,
        char_index: usize,
        expected: String,
        found: crate::transpiler::Token,
    ) -> Self {
        Self {
            line,
            file,
            error: format!("Expected '{}', found '{}'", expected, found.literal),
            line_content,
            char_index,
            expected: Some(expected),
            found: Some(found),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} Parser error at {}, line {}: {}",
            color::error("[ERROR]"),
            self.file,
            self.line,
            self.error
        )?;

        write!(f, "\n   └─> {}", self.line_content)?;
        let pointer = " ".repeat(self.char_index) + "^";
        write!(f, "\n       {}", color::bold(&pointer))
    }
}

// Conversiones automáticas
impl From<ReadFileError> for CompilerError {
    fn from(error: ReadFileError) -> Self {
        CompilerError::ReadFile(error)
    }
}

impl From<LexerError> for CompilerError {
    fn from(error: LexerError) -> Self {
        CompilerError::Lexer(error)
    }
}

impl From<ParseError> for CompilerError {
    fn from(error: ParseError) -> Self {
        CompilerError::Parser(error)
    }
}
