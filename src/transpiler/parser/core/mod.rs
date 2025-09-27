mod config_def;
mod endpoint_def;
mod grammar;
mod model_def;
mod parser;

use crate::transpiler::Token;
use crate::transpiler::errors::ParseError;
use crate::transpiler::parser::ast::TopLevel;
pub use parser::Parser;

pub fn parse(
    tokens: Vec<Token>,
    file_path: &str,
    content: Vec<String>,
) -> Result<Vec<TopLevel>, ParseError> {
    let mut parser = Parser::new(tokens, file_path, content);
    parser.parse()
}
