use crate::{
    ast::{
        declaration::{Declaration, DeclarationParser},
        expression::{Expr, ExpressionParser},
        statement::Stmt,
    },
    debug::Debugable,
    stages::ParseError,
    token::{Keyword, Token, TokenStream, TokenType},
    values::LoxValue,
};

pub struct StatementParser<'a>(&'a mut TokenStream);

impl<'a> StatementParser<'a> {
    pub fn parse(stream: &'a mut TokenStream) -> Result<Stmt, ParseError> {
        let mut parser = Self(stream);

        match parser.0.peek() {
            Some(Token {
                token_type: TokenType::Keyword(Keyword::For),
                ..
            }) => parser.for_statement(),
            Some(Token {
                token_type: TokenType::Keyword(Keyword::While),
                ..
            }) => parser.while_statement(),
            Some(Token {
                token_type: TokenType::Keyword(Keyword::If),
                ..
            }) => parser.if_statement(),
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
            }) => parser.block(),
            _ => {
                let expr = ExpressionParser::parse(parser.0)?;
                parser.0.match_tokens(&[TokenType::SemiColon]);
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        self.0.next();
        let mut stmts = Vec::new();
        while self.0.match_tokens(&[TokenType::RightBrace]).is_none() {
            if let Some(eof) = self.0.match_tokens(&[TokenType::Eof]) {
                return Err(ParseError::Eof {
                    message: "Expect '}'",
                    line: eof.line(),
                });
            }

            let declaration = DeclarationParser::parse(self.0)?;
            stmts.push(declaration);
        }

        Ok(Stmt::Block(stmts))
    }

    /// forStmt → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.0.next();
        self.0.match_tokens(&[TokenType::LeftParen]).ok_or({
            let token = self.0.peek().unwrap();
            ParseError::InvalidControlFlowSyntax {
                identifier_type: "(",
                before_type: "'for'",
                line: token.line(),
                span: token.span(),
            }
        })?;

        let initializer = match self.0.peek() {
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Var),
                ..
            }) => {
                let mut parser = DeclarationParser::new(self.0);
                Some(parser.var_declaration()?)
            }
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.0.next();
                None
            }
            _ => {
                let expr = Some(Stmt::Expr(ExpressionParser::parse(self.0)?));
                self.0.match_tokens(&[TokenType::SemiColon]);
                expr.map(Declaration::from)
            }
        };

        let condition = match self.0.peek() {
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.0.next();
                None
            }
            _ => {
                let expr = Some(ExpressionParser::parse(self.0)?);
                if expr.is_some() {
                    self.0.match_tokens(&[TokenType::SemiColon]);
                }
                expr
            }
        };

        let increment = match self.0.peek() {
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.0.next();
                None
            }
            Some(Token {
                token_type: TokenType::RightParen,
                ..
            }) => None,
            _ => {
                let expr = Some(ExpressionParser::parse(self.0)?);
                self.0.match_tokens(&[TokenType::SemiColon]);
                expr
            }
        };
        self.0.match_tokens(&[TokenType::RightParen]).ok_or({
            let token = self.0.peek().unwrap();
            ParseError::InvalidControlFlowSyntax {
                identifier_type: ")",
                before_type: "for condition",
                line: token.line(),
                span: token.span(),
            }
        })?;

        let body = StatementParser::parse(self.0)?;

        let while_body = if let Some(expr) = increment {
            Stmt::Block(vec![body.into(), expr.into()])
        } else {
            body
        };

        let while_stmt = Stmt::While {
            condition: condition.unwrap_or({
                let token = Token::new_dummy();
                Expr::Literal {
                    value: LoxValue::Bool {
                        value: true,
                        token: token.clone(),
                    },
                    token,
                }
            }),
            body: Box::new(while_body),
        };

        if let Some(initializer) = initializer {
            Ok(Stmt::Block(vec![initializer, while_stmt.into()]))
        } else {
            Ok(while_stmt)
        }
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.0.next();
        self.0.match_tokens(&[TokenType::LeftParen]).ok_or({
            let token = self.0.peek().unwrap();
            ParseError::InvalidControlFlowSyntax {
                identifier_type: "(",
                before_type: "'while'",
                line: token.line(),
                span: token.span(),
            }
        })?;
        let condition = ExpressionParser::parse(self.0)?;
        self.0.match_tokens(&[TokenType::RightParen]).ok_or({
            let token = self.0.peek().unwrap();
            ParseError::InvalidControlFlowSyntax {
                identifier_type: ")",
                before_type: "while condition",
                line: token.line(),
                span: token.span(),
            }
        })?;

        let body = Box::new(StatementParser::parse(self.0)?);

        Ok(Stmt::While { condition, body })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.0.next();
        self.0.match_tokens(&[TokenType::LeftParen]).ok_or(
            ParseError::InvalidControlFlowSyntax {
                identifier_type: "(",
                before_type: "'if'",
                line: self.0.peek().unwrap().line(),
                span: self.0.peek().unwrap().span(),
            },
        )?;
        let condition = ExpressionParser::parse(self.0)?;
        self.0.match_tokens(&[TokenType::RightParen]).ok_or(
            ParseError::InvalidControlFlowSyntax {
                identifier_type: ")",
                before_type: "if condition",
                line: self.0.peek().unwrap().line(),
                span: self.0.peek().unwrap().span(),
            },
        )?;

        let then_branch = Box::new(StatementParser::parse(self.0)?);

        let else_branch = if self
            .0
            .match_tokens(&[TokenType::Keyword(Keyword::Else)])
            .is_some()
        {
            Some(Box::new(StatementParser::parse(self.0)?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
}
