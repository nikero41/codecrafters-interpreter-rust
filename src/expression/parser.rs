use crate::{
    expression::{BinaryOp, Expr, UnaryOp},
    stages::ParseError,
    token::{TokenStream, TokenType},
    values::LoxValue,
};

pub struct ExpressionParser<'a> {
    stream: &'a mut TokenStream,
}

impl<'a> ExpressionParser<'a> {
    pub fn parse(stream: &'a mut TokenStream) -> Result<Expr, ParseError> {
        let mut parser = Self { stream };
        parser.expression()
    }

    fn match_binary(
        &mut self,
        operant: fn(&mut Self) -> Result<Expr, ParseError>,
        token_types: &[TokenType],
    ) -> Result<Expr, ParseError> {
        let mut expr = operant(self)?;

        while let Some(operator) = self.stream.match_tokens(token_types) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(operant(self)?),
            }
        }

        Ok(expr)
    }

    /// expression → equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        self.match_binary(Self::comparison, &[TokenType::BangEqual, TokenType::Equal])
    }

    /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        self.match_binary(
            Self::term,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    /// term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        self.match_binary(Self::factor, &[TokenType::Minus, TokenType::Plus])
    }

    /// factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        self.match_binary(Self::unary, &[TokenType::Slash, TokenType::Star])
    }

    /// unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, ParseError> {
        match self
            .stream
            .match_tokens(&[TokenType::Bang, TokenType::Minus])
        {
            Some(operator) => Ok(Expr::Unary {
                operator: UnaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(self.unary()?),
            }),
            _ => self.primary(),
        }
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.stream.next() {
            Some(token) if token.token_type == TokenType::LeftParen => {
                let expr = self.expression()?;
                if self.stream.match_tokens(&[TokenType::RightParen]).is_some() {
                    Ok(Expr::Grouping(Box::new(expr)))
                } else {
                    let token = self.stream.next().unwrap();

                    Err(ParseError::UnterminatedParen {
                        line: token.line(),
                        span: token.span(),
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
                        span: token.span(),
                    }
                }),

            _ => Err(ParseError::Eof {
                line: 0,
                message: "Unexpected end of file.".to_string(),
            }),
        }
    }
}
