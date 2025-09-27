use crate::transpiler::{Token, TokenType, errors::ParseError};

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub indent_level: usize,
    pub file: String,
    pub source_lines: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, file_path: &str, content: Vec<String>) -> Self {
        Self {
            tokens,
            current: 0,
            indent_level: 0,
            file: file_path.to_string(),
            source_lines: content,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek().token_type, TokenType::Eof)
    }

    pub(crate) fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .unwrap_or(&Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
                line: 0,
                column: 0,
            })
            .clone()
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(self.create_unexpected_token_error(message.to_string()))
        }
    }

    pub fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    // ===== Utilidades =====

    pub fn skip_whitespace_tokens(&mut self) {
        while self.check(&TokenType::Indent) || self.check(&TokenType::Dedent) {
            self.advance();
        }
    }

    pub fn get_identifier(&mut self) -> Result<String, ParseError> {
        match &self.advance().token_type {
            TokenType::Identifier(name) => Ok(name.clone()),
            _ => Err(self.create_error("Expected identifier")),
        }
    }

    pub fn get_string_literal(&self, literal: &str) -> String {
        literal.trim_matches('"').to_string()
    }

    pub fn create_error(&self, message: &str) -> ParseError {
        let line = self.get_current_line();
        let line_content = self.get_line_content(line);
        let char_index = self.get_char_index();

        ParseError::new(
            line,
            self.file.clone(),
            message.to_string(),
            line_content,
            char_index,
        )
    }

    pub(crate) fn create_unexpected_token_error(&self, expected: String) -> ParseError {
        let line = self.get_current_line();
        let line_content = self.get_line_content(line);
        let char_index = self.get_char_index();

        ParseError::unexpected_token(
            line,
            self.file.clone(),
            line_content,
            char_index,
            expected,
            self.peek().clone(),
        )
    }

    fn get_current_line(&self) -> usize {
        if let Some(token) = self.tokens.get(self.current) {
            token.line
        } else {
            1
        }
    }

    fn get_line_content(&self, line: usize) -> String {
        self.source_lines
            .get(line.saturating_sub(1))
            .cloned()
            .unwrap_or_default()
    }

    fn get_char_index(&self) -> usize {
        if let Some(token) = self.tokens.get(self.current) {
            token.column
        } else {
            0
        }
    }
}
