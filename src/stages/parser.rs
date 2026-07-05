use std::iter::Peekable;
use std::vec::IntoIter;

use crate::FANCY_ERROR;
use crate::expression::{BinaryOp, UnaryOp};
use crate::values::LoxValue;
use crate::{
    expression::Expr,
    source_file::SourceFile,
    stages::StageResult,
    stages::errors::ParseError,
    token::{Keyword, Token, TokenType},
};

pub struct Parser<'a> {
    source_file: &'a SourceFile,
    cursor: Peekable<IntoIter<Token>>,
    results: Vec<Result<Expr, ParseError>>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a SourceFile, tokens: Vec<Token>) -> Self {
        Self {
            cursor: tokens.into_iter().peekable(),
            source_file: file,
            results: Vec::new(),
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

    fn synchronize(&mut self) {
        while self
            .match_tokens(&[
                TokenType::Keyword(Keyword::Class),
                TokenType::Keyword(Keyword::Fun),
                TokenType::Keyword(Keyword::Var),
                TokenType::Keyword(Keyword::For),
                TokenType::Keyword(Keyword::If),
                TokenType::Keyword(Keyword::While),
                TokenType::Keyword(Keyword::Print),
                TokenType::Keyword(Keyword::Return),
                TokenType::Eof,
            ])
            .is_none()
        {
            self.next();
        }
    }

    /// expression → equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_tokens(&[TokenType::BangEqual, TokenType::Equal]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(self.comparison()?),
            }
        }

        Ok(expr)
    }

    /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
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
                operator: BinaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(self.term()?),
            }
        }

        Ok(expr)
    }

    /// term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(self.factor()?),
            }
        }

        Ok(expr)
    }

    /// factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(self.unary()?),
            }
        }

        Ok(expr)
    }

    /// unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, ParseError> {
        match self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Some(operator) => Ok(Expr::Unary {
                operator: UnaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(self.unary()?),
            }),
            _ => self.primary(),
        }
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.next() {
            Some(token) if token.token_type == TokenType::LeftParen => {
                let expr = self.expression()?;
                if self.match_tokens(&[TokenType::RightParen]).is_some() {
                    Ok(Expr::Grouping(Box::new(expr)))
                } else {
                    let token = self.next().unwrap();

                    Err(ParseError::UnterminatedParen {
                        line: token.line(),
                        src: self.source_file.named_source.clone(),
                        span: token.span(self.source_file),
                    })
                }
            }

            Some(token) => LoxValue::try_from(&token)
                .map(|value| Expr::Literal {
                    value,
                    token: token.clone(),
                })
                .map_err(|_| {
                    let lexeme = token.token_type.lexeme();
                    ParseError::ExpressionExpected {
                        line: token.line(),
                        lexeme,
                        message: "Expect expression.".to_string(),
                        src: self.source_file.named_source.clone(),
                        span: token.span(self.source_file),
                    }
                }),

            _ => Err(ParseError::Eof {
                line: 0,
                message: "Unexpected end of file.".to_string(),
            }),
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.peek()
            && token.token_type != TokenType::Eof
        {
            let expr = self.expression().inspect_err(|_| self.synchronize());
            self.results.push(expr);
        }
    }

    pub fn expressions(&self) -> Vec<Expr> {
        self.results
            .clone()
            .into_iter()
            .flat_map(Result::ok)
            .collect()
    }
}

impl StageResult for Parser<'_> {
    fn print(&self) {
        self.results.iter().for_each(|token| match token {
            Ok(token) => println!("{}", token),
            Err(err) => {
                if FANCY_ERROR {
                    eprintln!("{:?}", miette::Report::new(err.clone()))
                } else {
                    eprintln!("{}", err)
                }
            }
        })
    }

    fn print_errors(&self) {
        self.results.iter().for_each(|expr| {
            if let Err(err) = expr {
                if FANCY_ERROR {
                    eprintln!("{:?}", miette::Report::new(err.clone()))
                } else {
                    eprintln!("{}", err)
                }
            }
        });
    }

    fn has_errors(&self) -> bool {
        self.results.iter().any(Result::is_err)
    }
}
