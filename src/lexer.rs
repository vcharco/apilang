use crate::token::{Token, TokenType};
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

    pub fn next_token(&mut self) -> Option<Token> {
        if !self.token_queue.is_empty() {
            return Some(self.token_queue.remove(0));
        }

        self.skip_whitespace_and_comments();

        if self.current_char.is_none() {
            while self.indent_stack.len() > 1 {
                self.indent_stack.pop();
                self.token_queue
                    .push(self.new_token_literal(TokenType::Dedent, ""));
            }
            if !self.token_queue.is_empty() {
                return Some(self.token_queue.remove(0));
            }
            return Some(self.new_token_literal(TokenType::Eof, ""));
        }

        if self.current_char == Some('\n') {
            self.line_number += 1;
            self.advance_char();
            self.process_indentation();
            if !self.token_queue.is_empty() {
                return Some(self.token_queue.remove(0));
            }
            return self.next_token();
        }

        let token = match self.current_char {
            Some(':') => self.new_token(TokenType::Colon),
            Some('(') => self.new_token(TokenType::ParenOpen),
            Some(')') => self.new_token(TokenType::ParenClose),
            Some(',') => self.new_token(TokenType::Comma),
            Some('=') => self.new_token(TokenType::Equal),
            Some('+') => self.new_token(TokenType::Plus),
            Some('"') => self.read_string_double_quote(),
            Some('\'') => self.read_string_single_quote(),
            Some(ch) if ch.is_alphabetic() => {
                let literal = self.read_identifier();
                let token_type = self.lookup_identifier(&literal);
                return Some(self.new_token_literal(token_type, &literal));
            }
            Some(ch) if ch.is_numeric() => {
                let literal = self.read_number();
                let number_value = literal.parse::<f64>().unwrap_or(0.0);
                let token_type = TokenType::Number(number_value);
                return Some(self.new_token_literal(token_type, &literal));
            }
            Some(ch) => {
                self.report_error(&format!("Unexpected character: '{}'", ch.escape_default()));
                self.new_token(TokenType::Illegal)
            }
            None => {
                return Some(self.new_token_literal(TokenType::Eof, ""));
            }
        };

        Some(token)
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
        self.advance_char();
        let start_pos = self.current_position;
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
        self.advance_char();
        let start_pos = self.current_position;
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
                self.report_error("Indentation error: must be a multiple of 4 spaces.");
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
                self.report_error("Indentation error: mismatched indentation level.");
            }
        }
    }

    fn report_error(&self, message: &str) {
        let file_info = self
            .file_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("unknown file");

        eprintln!(
            "Error in file '{}', line {}: {}.",
            file_info, self.line_number, message
        );
    }
}
