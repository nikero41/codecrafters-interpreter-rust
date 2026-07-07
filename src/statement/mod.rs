use std::fmt::{Display, Formatter};

use crate::{
    debug::Debugable,
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
    /// block → "{" declaration* "}" ;
    Block(Vec<Stmt>),
}

impl Stmt {
    pub fn execute(self, env: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Stmt::Print(expr) => {
                let value = expr.eval(env)?;
                println!("{}", value);
            }
            Stmt::Expr(expr) => expr.eval(env).map(|_| ())?,
            Stmt::Block(stmts) => {
                let mut block_env = Environment::new(Some(env));
                stmts
                    .into_iter()
                    .try_for_each(|stmt| stmt.execute(&mut block_env))?
            }
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
            Stmt::Block { .. } => write!(f, "DECLARE"),
        }
    }
}

pub struct StatementParser<'a>(&'a mut TokenStream);

impl<'a> StatementParser<'a> {
    pub fn parse(stream: &'a mut TokenStream) -> Result<Stmt, ParseError> {
        let mut parser = Self(stream);

        match parser.0.peek() {
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Var),
                ..
            }) => parser.var_declaration(),
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Print),
                ..
            }) => {
                parser.0.next();
                ExpressionParser::parse(parser.0).map(|expr| {
                    parser.0.match_tokens(&[TokenType::SemiColon]);
                    Stmt::Print(expr)
                })
            }
            Some(Token {
                token_type: TokenType::LeftBrace,
                ..
            }) => {
                parser.0.next();
                let mut stmts = Vec::new();
                while parser.0.match_tokens(&[TokenType::RightBrace]).is_none() {
                    if let Some(eof) = parser.0.match_tokens(&[TokenType::Eof]) {
                        return Err(ParseError::Eof {
                            message: "Expect '}'",
                            line: eof.line(),
                        });
                    }

                    let stmt = StatementParser::parse(parser.0)?;
                    stmts.push(stmt);
                }

                Ok(Stmt::Block(stmts))
            }
            _ => {
                let expr = ExpressionParser::parse(parser.0)?;
                parser.0.match_tokens(&[TokenType::SemiColon]);
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let var_token = self.0.next().unwrap();

        let name = match self.0.next() {
            Some(
                token @ Token {
                    token_type: TokenType::Identifier(_),
                    ..
                },
            ) => Ok(token),
            Some(token) => Err(ParseError::IdentifierExpected {
                line: token.line(),
                identifier_type: "identifier",
                span: token.span(),
            }),
            None => Err(ParseError::Eof {
                line: var_token.line(),
                message: "Unexpected EOF",
            }),
        }?;

        let expr = if self.0.match_tokens(&[TokenType::Assign]).is_some() {
            Some(ExpressionParser::parse(self.0)?)
        } else {
            None
        };

        self.0.match_tokens(&[TokenType::SemiColon]);
        Ok(Stmt::DeclareVar { name, expr })
    }
}
