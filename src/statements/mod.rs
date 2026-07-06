use std::fmt::{Display, Formatter};

use crate::{
    expression::{Expr, ExpressionParser, InterpretError, interpret::Interpretable},
    stages::ParseError,
    token::{Keyword, Token, TokenStream, TokenType},
};

/// expression     → literal | unary | binary | grouping ;
#[derive(Debug, Clone)]
pub enum Stmt {
    /// printStmt      → "print" expression ";" ;
    Print(Expr),
    /// exprStmt       → expression ";" ;
    Expr,
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Print(_) => write!(f, "PRINT"),
            Stmt::Expr => write!(f, "EXPR"),
        }
    }
}

pub struct StatementParser<'a> {
    stream: &'a mut TokenStream,
}

impl<'a> StatementParser<'a> {
    pub fn parse(stream: &'a mut TokenStream) -> Result<Stmt, ParseError> {
        let parser = Self { stream };

        match parser.stream.next() {
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Print),
                ..
            }) => {
                let expr = ExpressionParser::parse(parser.stream)?;
                parser.stream.match_tokens(&[TokenType::SemiColon]);
                Ok(Stmt::Print(expr))
            }
            _ => Ok(Stmt::Expr),
        }
    }
}

pub trait Executable {
    fn execute(&self) -> Result<(), InterpretError>;
}

impl Executable for Stmt {
    fn execute(&self) -> Result<(), InterpretError> {
        match self {
            Stmt::Print(expr) => {
                let value = expr.interpret()?;
                println!("{}", value);
            }
            Stmt::Expr => todo!(),
        }
        Ok(())
    }
}
