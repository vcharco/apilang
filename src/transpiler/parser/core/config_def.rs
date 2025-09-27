use super::Parser;
use crate::transpiler::TokenType;
use crate::transpiler::errors::ParseError;
use crate::transpiler::parser::ast::{ConfigSetting, ConfigureDef, TopLevel};

impl Parser {
    pub(crate) fn configure_definition(&mut self) -> Result<TopLevel, ParseError> {
        // Consumir "configure"
        self.consume(TokenType::KeywordConfigure, "Expected 'configure'")?;

        // Consumir ":"
        self.consume(TokenType::Colon, "Expected ':' after 'configure'")?;

        // Consumir indentación
        self.consume(TokenType::Indent, "Expected indentation after configure")?;

        let mut settings = Vec::new();

        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            if let Some(setting) = self.parse_config_setting()? {
                settings.push(setting);
            }
        }

        if self.check(&TokenType::Dedent) {
            self.advance();
        }

        Ok(TopLevel::Configure(ConfigureDef { settings }))
    }

    fn parse_config_setting(&mut self) -> Result<Option<ConfigSetting>, ParseError> {
        let key = self.get_identifier()?;
        self.consume(TokenType::Colon, "Expected ':' after config key")?;

        match key.as_str() {
            "basepath" => {
                let literal = self.advance().literal.clone();
                let value = self.get_string_literal(&literal);
                Ok(Some(ConfigSetting::BasePath(value)))
            }
            "throttle" => {
                if let TokenType::Number(value) = &self.advance().token_type {
                    Ok(Some(ConfigSetting::Throttle(*value)))
                } else {
                    Err(self.create_error("Expected number for throttle"))
                }
            }
            "private" => {
                self.consume(
                    TokenType::SquareBracketOpen,
                    "Expected '[' for private list",
                )?;
                let mut roles = Vec::new();

                while !self.check(&TokenType::SquareBracketClose) && !self.is_at_end() {
                    let literal = self.advance().literal.clone();
                    let role = self.get_string_literal(&literal);
                    roles.push(role);

                    if self.check(&TokenType::Comma) {
                        self.advance();
                    }
                }

                self.consume(TokenType::SquareBracketClose, "Expected ']'")?;
                Ok(Some(ConfigSetting::Private(roles)))
            }
            _ => {
                // Ignorar configuraciones desconocidas por ahora
                self.advance(); // consumir valor
                Ok(None)
            }
        }
    }
}
