use crate::{
    ast::{declaration::Declaration, expression::ExpressionParser, statement::StatementParser},
    debug::Debugable,
    stages::ParseError,
    token::{Keyword, Token, TokenStream, TokenType},
};

pub struct DeclarationParser<'a>(&'a mut TokenStream);

impl<'a> DeclarationParser<'a> {
    pub fn new(stream: &'a mut TokenStream) -> Self {
        Self(stream)
    }

    pub fn parse(stream: &'a mut TokenStream) -> Result<Declaration, ParseError> {
        let mut parser = Self(stream);

        match parser.0.peek() {
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Var),
                ..
            }) => parser.var_declaration(),
            _ => {
                let stmt = StatementParser::parse(parser.0)?;
                parser.0.match_tokens(&[TokenType::SemiColon]);
                Ok(Declaration::Statement(stmt))
            }
        }
    }

    pub fn var_declaration(&mut self) -> Result<Declaration, ParseError> {
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
        Ok(Declaration::Var { name, expr })
    }
}
