use crate::{
    debug::Debugable,
    expression::{BinaryOp, Expr, UnaryOp},
    stages::ParseError,
    token::{Token, TokenStream, TokenType},
    values::LoxValue,
};

pub struct ExpressionParser<'a>(&'a mut TokenStream);

impl<'a> ExpressionParser<'a> {
    pub fn parse(stream: &'a mut TokenStream) -> Result<Expr, ParseError> {
        let mut parser = Self(stream);
        parser.expression()
    }

    /// binary → operant ( ( token_types ) operrant )* ;
    fn match_binary(
        &mut self,
        operant: fn(&mut Self) -> Result<Expr, ParseError>,
        token_types: &[TokenType],
    ) -> Result<Expr, ParseError> {
        let mut expr = operant(self)?;

        while let Some(operator) = self.0.match_tokens(token_types) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(operant(self)?),
            }
        }

        Ok(expr)
    }

    /// expression → assignment ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    /// assignment → IDENTIFIER "=" assignment | equality ;
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality()?;

        if self.0.match_tokens(&[TokenType::Assign]).is_some() {
            if let Expr::Variable(token) = expr {
                let assignment = self.assignment()?;
                Ok(Expr::Assign {
                    token,
                    value: Box::new(assignment),
                })
            } else {
                Err(ParseError::InvalidAssignment {
                    line: expr.line(),
                    span: expr.span(),
                })
            }
        } else {
            Ok(expr)
        }
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
        match self.0.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Some(operator) => Ok(Expr::Unary {
                operator: UnaryOp::try_from(&operator.token_type).unwrap(),
                right: Box::new(self.unary()?),
            }),
            _ => self.primary(),
        }
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.0.next() {
            Some(token) if token.token_type == TokenType::LeftParen => {
                let expr = self.expression()?;
                if self.0.match_tokens(&[TokenType::RightParen]).is_some() {
                    Ok(Expr::Grouping(Box::new(expr)))
                } else {
                    Err(ParseError::UnterminatedParen {
                        line: expr.line(),
                        span: expr.span(),
                    })
                }
            }

            Some(
                token @ Token {
                    token_type: TokenType::Identifier(_),
                    ..
                },
            ) => Ok(Expr::Variable(token)),

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
                        message: "Expect expression.",
                        span: token.span(),
                    }
                }),

            _ => panic!("Unexpected end of file."),
        }
    }
}
