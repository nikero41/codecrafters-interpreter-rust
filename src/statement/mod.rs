use std::fmt::{Display, Formatter};

use crate::{
    environment::Environment,
    expression::{Expr, ExpressionParser},
    interpreter::RuntimeError,
    stages::ParseError,
    token::{Keyword, Token, TokenStream, TokenType},
    values::LoxValue,
};

#[derive(Debug, Clone)]
pub enum Stmt {
    /// varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
    DeclareVar { name: Token, expr: Option<Expr> },
    /// printStmt → "print" expression ";" ;
    Print(Expr),
    /// exprStmt → expression ";" ;
    Expr(Expr),
}

impl Stmt {
    pub fn execute(self, env: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Stmt::Print(expr) => {
                let value = expr.eval(env)?;
                println!("{}", value);
            }
            Stmt::Expr(expr) => expr.eval(env).map(|_| ())?,
            Stmt::DeclareVar { name, expr } => {
                let value = if let Some(expr) = expr {
                    expr.eval(env)?
                } else {
                    LoxValue::Nil {
                        token: name.clone(),
                    }
                };
                env.define(name.token_type.lexeme(), value.clone());
            }
        }
        Ok(())
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Print(_) => write!(f, "PRINT"),
            Stmt::Expr(_) => write!(f, "EXPR"),
            Stmt::DeclareVar { .. } => write!(f, "DECLARE"),
        }
    }
}

pub struct StatementParser<'a>(&'a mut TokenStream);

impl<'a> StatementParser<'a> {
    pub fn parse(stream: &'a mut TokenStream) -> Result<Stmt, ParseError> {
        let parser = Self(stream);

        match parser.0.peek() {
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Var),
                ..
            }) => {
                parser.0.next();
                let name = parser.0.next().ok_or(ParseError::Eof {
                    message: "Unexpected EOF",
                })?;

                let expr = if parser.0.match_tokens(&[TokenType::Assign]).is_some() {
                    Some(ExpressionParser::parse(parser.0)?)
                } else {
                    None
                };

                parser.0.match_tokens(&[TokenType::SemiColon]);
                Ok(Stmt::DeclareVar { name, expr })
            }
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Print),
                ..
            }) => {
                parser.0.next();
                let expr = ExpressionParser::parse(parser.0)?;
                parser.0.match_tokens(&[TokenType::SemiColon]);
                Ok(Stmt::Print(expr))
            }
            _ => {
                let expr = ExpressionParser::parse(parser.0)?;
                parser.0.match_tokens(&[TokenType::SemiColon]);
                Ok(Stmt::Expr(expr))
            }
        }
    }
}
