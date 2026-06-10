use std::{iter::Peekable, vec::IntoIter};

use crate::{
    lexer::{
        keyword::Keyword,
        token::{Token, TokenType},
    },
    parser::{
        errors::ParseError,
        expressions::{Expr, Literal},
    },
};
pub mod expressions;

mod errors;

struct Parser {
    cursor: Peekable<IntoIter<Token>>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            cursor: tokens.into_iter().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.cursor.peek()
    }

    fn next(&mut self) -> Option<Token> {
        self.cursor.next()
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<Token> {
        match self.cursor.peek() {
            Some(token) if token_types.contains(&token.token_type) => self.cursor.next(),
            _ => None,
        }
    }

    // expression → equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_tokens(&[TokenType::BangEqual, TokenType::Equal]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(self.comparison()?),
            }
        }

        Ok(expr)
    }

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while let Some(operator) = self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(self.term()?),
            }
        }

        Ok(expr)
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(self.factor()?),
            }
        }

        Ok(expr)
    }

    // factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(self.unary()?),
            }
        }

        Ok(expr)
    }

    // unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(operator) = self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Ok(Expr::Unary {
                operator,
                right: Box::new(self.unary()?),
            })
        } else {
            self.primary()
        }
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek();
        let mut should_advance = false;

        if let Some(token) = token {
            let expr = match &token.token_type {
                TokenType::Number(float) => {
                    should_advance = true;
                    Ok(Expr::Literal(Literal::Number(
                        float.parse::<f64>().unwrap(),
                    )))
                }
                TokenType::String(literal) => {
                    should_advance = true;
                    Ok(Expr::Literal(Literal::String(literal.clone())))
                }
                TokenType::Keyword(Keyword::True) => {
                    should_advance = true;
                    Ok(Expr::Literal(Literal::Bool(true)))
                }
                TokenType::Keyword(Keyword::False) => {
                    should_advance = true;
                    Ok(Expr::Literal(Literal::Bool(false)))
                }
                TokenType::Keyword(Keyword::Nil) => {
                    should_advance = true;
                    Ok(Expr::Literal(Literal::Nil))
                }
                TokenType::Identifier(identifier) => {
                    should_advance = true;
                    Ok(Expr::Literal(Literal::String(identifier.to_string())))
                }
                TokenType::LeftParen => {
                    let token_line = token.line;
                    self.next();
                    let expr = self.expression()?;
                    if self.match_tokens(&[TokenType::RightParen]).is_some() {
                        Ok(Expr::Grouping(Box::new(expr)))
                    } else {
                        Err(ParseError::UnterminatedParen { line: token_line })
                    }
                }
                _ => Err(ParseError::Placeholder),
            };

            if should_advance {
                self.next();
            }
            expr
        } else {
            Err(ParseError::Placeholder)
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.expression()
}
