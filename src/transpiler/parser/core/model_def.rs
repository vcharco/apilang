use super::Parser;
use crate::transpiler::TokenType;
use crate::transpiler::errors::ParseError;
use crate::transpiler::parser::ast::{Constraint, FieldDef, FieldType, ModelDef, TopLevel};

impl Parser {
    pub fn model_definition(&mut self) -> Result<TopLevel, ParseError> {
        // Consumir "model"
        self.consume(TokenType::KeywordModel, "Expected 'model'")?;

        // Obtener nombre del modelo
        let name = self.get_identifier()?;

        // Consumir ":"
        self.consume(TokenType::Colon, "Expected ':' after model name")?;

        // Consumir indentación
        self.consume(
            TokenType::Indent,
            "Expected indentation after model declaration",
        )?;

        // Procesar campos
        let mut fields = Vec::new();

        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            fields.push(self.field_definition()?);
        }

        // Consumir dedentación si existe
        if self.check(&TokenType::Dedent) {
            self.advance();
        }

        Ok(TopLevel::Model(ModelDef { name, fields }))
    }

    fn field_definition(&mut self) -> Result<FieldDef, ParseError> {
        // Obtener nombre del campo
        let name = self.get_identifier()?;

        // Consumir ":"
        self.consume(TokenType::Colon, "Expected ':' after field name")?;

        // Obtener tipo del campo
        let field_type = self.parse_field_type()?;

        // Procesar constraints y tags si están presentes
        let mut constraints = Vec::new();
        let mut tags = Vec::new();

        if self.check(&TokenType::ParenOpen) {
            self.advance(); // consume '('

            while !self.check(&TokenType::ParenClose) && !self.is_at_end() {
                // Intentar parsear constraint
                if let Some(constraint) = self.parse_constraint()? {
                    constraints.push(constraint);
                } else if let Some(tag_list) = self.parse_tags()? {
                    tags = tag_list;
                }

                if self.check(&TokenType::Comma) {
                    self.advance();
                }
            }

            self.consume(TokenType::ParenClose, "Expected ')'")?;
        }

        Ok(FieldDef {
            name,
            field_type,
            constraints,
            tags,
        })
    }

    fn parse_field_type(&mut self) -> Result<FieldType, ParseError> {
        match &self.advance().token_type {
            TokenType::KeywordString => Ok(FieldType::String),
            TokenType::KeywordNumber => Ok(FieldType::Number),
            TokenType::KeywordBoolean => Ok(FieldType::Boolean),
            TokenType::KeywordList => {
                // TODO: Implementar parsing de List(Type) si es necesario
                Ok(FieldType::List(Box::new(FieldType::String)))
            }
            TokenType::Identifier(name) => Ok(FieldType::Custom(name.clone())),
            _ => Err(self.create_error("Expected field type")),
        }
    }

    fn parse_constraint(&mut self) -> Result<Option<Constraint>, ParseError> {
        match &self.peek().token_type {
            TokenType::KeywordMin => {
                self.advance();
                self.consume(TokenType::Equal, "Expected '=' after 'min'")?;
                if let TokenType::Number(value) = &self.advance().token_type {
                    Ok(Some(Constraint::Min(*value)))
                } else {
                    Err(self.create_error("Expected number after 'min='"))
                }
            }
            TokenType::KeywordMax => {
                self.advance();
                self.consume(TokenType::Equal, "Expected '=' after 'max'")?;
                if let TokenType::Number(value) = &self.advance().token_type {
                    Ok(Some(Constraint::Max(*value)))
                } else {
                    Err(self.create_error("Expected number after 'max='"))
                }
            }
            TokenType::KeywordEmail => {
                self.advance();
                Ok(Some(Constraint::Email))
            }
            TokenType::KeywordPhone => {
                self.advance();
                Ok(Some(Constraint::Phone))
            }
            TokenType::KeywordRegex => {
                self.advance();
                self.consume(TokenType::Equal, "Expected '=' after 'regex'")?;
                let token = self.advance();
                let regex_pattern = if let TokenType::String(s) = &token.token_type {
                    s.clone()
                } else {
                    return Err(self.create_error("Expected string literal"));
                };
                Ok(Some(Constraint::Regex(regex_pattern)))
            }
            _ => Ok(None),
        }
    }

    fn parse_tags(&mut self) -> Result<Option<Vec<String>>, ParseError> {
        if let TokenType::Identifier(potential_tag) = &self.peek().token_type {
            if potential_tag == "tags" {
                self.advance();
                self.consume(TokenType::Equal, "Expected '=' after 'tags'")?;
                self.consume(TokenType::SquareBracketOpen, "Expected '[' for tags")?;

                let mut tags = Vec::new();

                while !self.check(&TokenType::SquareBracketClose) && !self.is_at_end() {
                    if let TokenType::Identifier(tag) = &self.advance().token_type {
                        tags.push(tag.clone());
                    }

                    if self.check(&TokenType::Comma) {
                        self.advance();
                    }
                }

                self.consume(TokenType::SquareBracketClose, "Expected ']'")?;
                return Ok(Some(tags));
            }
        }
        Ok(None)
    }
}
