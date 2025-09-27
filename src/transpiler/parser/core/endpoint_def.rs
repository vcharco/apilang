use super::Parser;
use crate::transpiler::errors::ParseError;
use crate::transpiler::parser::ast::{
    BodyDef, CacheConfig, EndpointDef, HttpMethod, ParamsDef, QueryDef, TopLevel, Visibility,
};

use crate::transpiler::TokenType;

impl Parser {
    pub(crate) fn endpoint_definition(&mut self) -> Result<TopLevel, ParseError> {
        // Parsear visibilidad (public/private)
        let visibility = self.parse_visibility()?;

        // Parsear configuración de cache si existe
        let cache_config = self.parse_cache_config()?;

        // Parsear método HTTP
        let method = self.parse_http_method()?;

        // Parsear path del endpoint
        let literal = self.advance().literal.clone();
        let path = self.get_string_literal(&literal);

        // Consumir ":"
        self.consume(TokenType::Colon, "Expected ':' after endpoint path")?;

        // Consumir indentación
        self.consume(TokenType::Indent, "Expected indentation after endpoint")?;

        // Inicializar propiedades del endpoint
        let mut params = None;
        let mut query = None;
        let mut body = None;
        let mut call = String::new();

        // Procesar propiedades del endpoint
        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            let property = self.get_identifier()?;
            self.consume(TokenType::Colon, "Expected ':' after endpoint property")?;

            match property.as_str() {
                "params" => {
                    params = Some(self.parse_params_def()?);
                }
                "query" => {
                    query = Some(self.parse_query_def()?);
                }
                "body" => {
                    body = Some(self.parse_body_def()?);
                }
                "call" => {
                    call = self.get_identifier()?;
                }
                _ => {
                    // Ignorar propiedades desconocidas
                    self.advance();
                }
            }
        }

        if self.check(&TokenType::Dedent) {
            self.advance();
        }

        Ok(TopLevel::Endpoint(EndpointDef {
            visibility,
            cache_config,
            method,
            path,
            params,
            query,
            body,
            call,
        }))
    }

    fn parse_visibility(&mut self) -> Result<Visibility, ParseError> {
        match &self.advance().token_type {
            TokenType::KeywordPublic => Ok(Visibility::Public),
            TokenType::KeywordPrivate => {
                if self.check(&TokenType::ParenOpen) {
                    self.advance(); // consume '('
                    let mut roles = Vec::new();

                    while !self.check(&TokenType::ParenClose) && !self.is_at_end() {
                        let literal = self.advance().literal.clone();
                        let role = self.get_string_literal(&literal);
                        roles.push(role);

                        if self.check(&TokenType::Comma) {
                            self.advance();
                        }
                    }

                    self.consume(TokenType::ParenClose, "Expected ')'")?;
                    Ok(Visibility::Private(roles))
                } else {
                    Ok(Visibility::Private(vec![]))
                }
            }
            _ => Err(self.create_error("Expected visibility modifier")),
        }
    }

    fn parse_cache_config(&mut self) -> Result<Option<CacheConfig>, ParseError> {
        match &self.peek().token_type {
            TokenType::KeywordCache => {
                self.advance();
                if self.check(&TokenType::ParenOpen) {
                    self.advance();
                    let key = self.parse_cache_key()?;
                    self.consume(TokenType::ParenClose, "Expected ')' after cache key")?;
                    Ok(Some(CacheConfig::Cached(key)))
                } else {
                    Ok(Some(CacheConfig::Cached("default".to_string())))
                }
            }
            TokenType::KeywordEvict => {
                self.advance();
                if self.check(&TokenType::ParenOpen) {
                    self.advance();
                    let key = self.parse_cache_key()?;
                    self.consume(TokenType::ParenClose, "Expected ')' after evict key")?;
                    Ok(Some(CacheConfig::Evict(key)))
                } else {
                    Ok(Some(CacheConfig::Evict("default".to_string())))
                }
            }
            _ => Ok(None),
        }
    }

    fn parse_cache_key(&mut self) -> Result<String, ParseError> {
        let mut key = String::new();

        while !self.check(&TokenType::ParenClose) && !self.is_at_end() {
            match &self.advance().token_type {
                TokenType::Identifier(s) => key.push_str(&s),
                TokenType::Plus => key.push('+'),
                TokenType::String(_) => key.push_str(&self.tokens[self.current - 1].literal),
                _ => {}
            }
        }

        Ok(key)
    }

    fn parse_http_method(&mut self) -> Result<HttpMethod, ParseError> {
        match &self.advance().token_type {
            TokenType::KeywordGet => Ok(HttpMethod::Get),
            TokenType::KeywordPost => Ok(HttpMethod::Post),
            TokenType::KeywordPatch => Ok(HttpMethod::Patch),
            TokenType::KeywordPut => Ok(HttpMethod::Put),
            TokenType::KeywordDelete => Ok(HttpMethod::Delete),
            TokenType::KeywordOptions => Ok(HttpMethod::Options),
            TokenType::KeywordHead => Ok(HttpMethod::Head),
            _ => Err(self.create_error("Expected HTTP method")),
        }
    }

    fn parse_params_def(&mut self) -> Result<ParamsDef, ParseError> {
        let name = self.get_identifier()?;

        // Verificar si hay "as tipo"
        let param_type = if self.check(&TokenType::Identifier(String::new())) {
            let next_token = self.advance();
            if next_token.literal == "as" {
                self.get_identifier()?
            } else {
                "string".to_string()
            }
        } else {
            "string".to_string()
        };

        Ok(ParamsDef { name, param_type })
    }

    fn parse_query_def(&mut self) -> Result<QueryDef, ParseError> {
        let query_type = self.get_identifier()?;
        Ok(QueryDef { query_type })
    }

    fn parse_body_def(&mut self) -> Result<BodyDef, ParseError> {
        let body_type = self.get_identifier()?;

        // Verificar si hay tags como (c)
        let tags = if self.check(&TokenType::ParenOpen) {
            self.advance(); // consume '('
            let mut tag_list = Vec::new();

            while !self.check(&TokenType::ParenClose) && !self.is_at_end() {
                if let TokenType::Identifier(tag) = &self.advance().token_type {
                    tag_list.push(tag.clone());
                }
                if self.check(&TokenType::Comma) {
                    self.advance();
                }
            }

            self.consume(TokenType::ParenClose, "Expected ')'")?;
            Some(tag_list)
        } else {
            None
        };

        Ok(BodyDef { body_type, tags })
    }
}
