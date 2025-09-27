use super::Parser;
use crate::transpiler::TokenType;
use crate::transpiler::errors::ParseError;
use crate::transpiler::parser::ast::TopLevel;

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<TopLevel>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Ignorar indentaciones al nivel superior
            self.skip_whitespace_tokens();

            if self.is_at_end() {
                break;
            }

            statements.push(self.top_level_statement()?);
        }

        Ok(statements)
    }

    fn top_level_statement(&mut self) -> Result<TopLevel, ParseError> {
        match &self.peek().token_type {
            TokenType::KeywordModel => self.model_definition(),
            TokenType::KeywordConfigure => self.configure_definition(),
            TokenType::KeywordPublic | TokenType::KeywordPrivate => self.endpoint_definition(),
            _ => Err(self.create_error("Invalid Syntax")),
        }
    }
}
