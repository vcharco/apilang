use crate::transpiler::token::{Token, TokenType};
use crate::util::color;
use std::fmt;
use std::path::PathBuf;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    current_char: Option<char>,
    current_position: usize,
    line_number: usize,
    indent_stack: Vec<usize>,
    token_queue: Vec<Token>,
    file_path: Option<PathBuf>,
    errors: Vec<LexerError>,
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
            "{} Lexer error at {}, line {}: ",
            color::error("[ERROR]"),
            self.file,
            self.line,
        )?;
        write!(f, "{}\n", self.error)?;

        write!(f, "   └─> {}\n", self.line_content)?;
        let pointer = " ".repeat(self.char_index) + "^";
        write!(f, "       {}", color::bold(&pointer))?;

        Ok(())
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, file_path: Option<PathBuf>) -> Self {
        let mut lexer = Lexer {
            input,
            chars: input.chars(),
            current_char: None,
            current_position: 0,
            line_number: 1,
            indent_stack: vec![0],
            token_queue: Vec::new(),
            file_path,
            errors: Vec::new(),
        };
        lexer.advance_char();
        lexer
    }

    fn advance_char(&mut self) {
        if let Some(ch) = self.chars.next() {
            self.current_position += ch.len_utf8();
            self.current_char = Some(ch);
        } else {
            self.current_char = None;
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, Vec<LexerError>> {
        if !self.token_queue.is_empty() {
            return Ok(Some(self.token_queue.remove(0)));
        }

        self.skip_whitespace_and_comments();

        if !self.errors.is_empty() {
            let errors = std::mem::take(&mut self.errors);
            return Err(errors);
        }

        if self.current_char.is_none() {
            while self.indent_stack.len() > 1 {
                self.indent_stack.pop();
                self.token_queue
                    .push(self.new_token_literal(TokenType::Dedent, ""));
            }
            if !self.token_queue.is_empty() {
                return Ok(Some(self.token_queue.remove(0)));
            }
            return Ok(Some(self.new_token_literal(TokenType::Eof, "")));
        }

        if self.current_char == Some('\n') {
            self.line_number += 1;
            self.advance_char();
            self.process_indentation();
            if !self.token_queue.is_empty() {
                return Ok(Some(self.token_queue.remove(0)));
            }
            return self.next_token();
        }

        let token = match self.current_char {
            Some(':') => self.new_token(TokenType::Colon),
            Some('(') => self.new_token(TokenType::ParenOpen),
            Some(')') => self.new_token(TokenType::ParenClose),
            Some('[') => self.new_token(TokenType::SquareBracketOpen),
            Some(']') => self.new_token(TokenType::SquareBracketClose),
            Some(',') => self.new_token(TokenType::Comma),
            Some('=') => self.new_token(TokenType::Equal),
            Some('+') => self.new_token(TokenType::Plus),
            Some('"') => self.read_string_double_quote(),
            Some('\'') => self.read_string_single_quote(),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let literal = self.read_identifier();
                let token_type = self.lookup_identifier(&literal);
                return Ok(Some(self.new_token_literal(token_type, &literal)));
            }
            Some(ch) if ch.is_numeric() => {
                let literal = self.read_number();
                let number_value = literal.parse::<f64>().unwrap_or(0.0);
                let token_type = TokenType::Number(number_value);
                return Ok(Some(self.new_token_literal(token_type, &literal)));
            }
            Some(ch) => {
                self.report_error(&format!("Unexpected character '{}'.", ch.escape_default()));
                self.new_token(TokenType::Illegal)
            }
            None => {
                return Ok(Some(self.new_token_literal(TokenType::Eof, "")));
            }
        };

        Ok(Some(token))
    }

    fn new_token(&mut self, token_type: TokenType) -> Token {
        let literal = self.current_char.map(|c| c.to_string()).unwrap_or_default();
        self.advance_char();
        Token {
            token_type,
            literal,
        }
    }

    fn new_token_literal(&self, token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.current_char {
            match ch {
                '#' => {
                    while self.current_char.is_some() && self.current_char != Some('\n') {
                        self.advance_char();
                    }
                }
                c if c.is_whitespace() && c != '\n' => {
                    self.advance_char();
                }
                _ => break,
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.current_position - self.current_char.unwrap().len_utf8();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.current_position - 1].to_string()
    }

    fn read_string_double_quote(&mut self) -> Token {
        let start_pos = self.current_position;
        self.advance_char();
        while let Some(ch) = self.current_char {
            if ch == '"' || ch == '\n' {
                break;
            }
            self.advance_char();
        }
        let literal = self.input[start_pos..self.current_position - 1].to_string();

        if self.current_char != Some('"') {
            self.report_error("Unterminated string literal.");
        } else {
            self.advance_char();
        }

        Token {
            token_type: TokenType::String,
            literal,
        }
    }

    fn read_string_single_quote(&mut self) -> Token {
        let start_pos = self.current_position;
        self.advance_char();
        while let Some(ch) = self.current_char {
            if ch == '\'' || ch == '\n' {
                break;
            }
            self.advance_char();
        }
        let literal = self.input[start_pos..self.current_position - 1].to_string();

        if self.current_char != Some('\'') {
            self.report_error("Unterminated string literal.");
        } else {
            self.advance_char();
        }

        Token {
            token_type: TokenType::String,
            literal,
        }
    }

    fn lookup_identifier(&self, identifier: &str) -> TokenType {
        match identifier {
            "get" => TokenType::KeywordGet,
            "post" => TokenType::KeywordPost,
            "put" => TokenType::KeywordPut,
            "patch" => TokenType::KeywordPatch,
            "delete" => TokenType::KeywordDelete,
            "options" => TokenType::KeywordOptions,
            "head" => TokenType::KeywordHead,
            "model" => TokenType::KeywordModel,
            "string" => TokenType::KeywordString,
            "number" => TokenType::KeywordNumber,
            "list" => TokenType::KeywordList,
            "boolean" => TokenType::KeywordBoolean,
            "min" => TokenType::KeywordMin,
            "max" => TokenType::KeywordMax,
            "email" => TokenType::KeywordEmail,
            "phone" => TokenType::KeywordPhone,
            "regex" => TokenType::KeywordRegex,
            "public" => TokenType::KeywordPublic,
            "private" => TokenType::KeywordPrivate,
            "cache" => TokenType::KeywordCache,
            "evict" => TokenType::KeywordEvict,
            "throttle" => TokenType::KeywordThrottle,
            "configure" => TokenType::KeywordConfigure,
            _ => TokenType::Identifier(identifier.to_string()),
        }
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.current_position - self.current_char.unwrap().len_utf8();
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                self.advance_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.current_position - 1].to_string()
    }

    fn process_indentation(&mut self) {
        let mut indent_count = 0;
        loop {
            match self.current_char {
                Some(' ') => {
                    indent_count += 1;
                    self.advance_char();
                }
                Some('\t') => {
                    indent_count += 4;
                    self.advance_char();
                }
                _ => break,
            }
        }

        if self.current_char.is_none()
            || self.current_char == Some('\n')
            || self.current_char == Some('#')
        {
            return;
        }

        let last_indent = *self.indent_stack.last().unwrap_or(&0);

        if indent_count > last_indent {
            if indent_count % 4 != 0 {
                self.report_error("Indentation error (must be a multiple of 4 spaces).");
                return;
            }
            self.indent_stack.push(indent_count);
            self.token_queue
                .push(self.new_token_literal(TokenType::Indent, ""));
        } else {
            while indent_count < *self.indent_stack.last().unwrap() {
                self.indent_stack.pop();
                self.token_queue
                    .push(self.new_token_literal(TokenType::Dedent, ""));
            }

            if indent_count != *self.indent_stack.last().unwrap() {
                self.report_error("Indentation error (mismatched indentation level).");
            }
        }
    }

    fn report_error(&mut self, message: &str) {
        let file_info = self
            .file_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("unknown file")
            .to_string();

        let line_start = self.input[..self.current_position]
            .rfind('\n')
            .map_or(0, |i| i + 1);
        let line_end = self.input[self.current_position..]
            .find('\n')
            .map_or(self.input.len(), |i| self.current_position + i);
        let line_content = self.input[line_start..line_end].trim_end().to_string();
        let char_index = self.current_position - line_start;

        self.errors.push(LexerError {
            line: self.line_number,
            file: file_info,
            error: message.to_string(),
            line_content,
            char_index: char_index - 1,
        });
    }
}
