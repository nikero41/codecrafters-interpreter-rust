use miette::SourceCode;

use crate::FANCY_ERROR;
use crate::expression::{Expr, ExpressionParser};
use crate::{
    stages::StageResult,
    stages::errors::ParseError,
    statements::{StatementParser, Stmt},
    token::TokenStream,
    token::{Keyword, Token, TokenType},
};

pub struct Parser {
    cursor: TokenStream,
    expr_results: Vec<Result<Expr, ParseError>>,
    results: Vec<Result<Stmt, ParseError>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            cursor: TokenStream::new(tokens),
            results: Vec::new(),
            expr_results: Vec::new(),
        }
    }

    fn synchronize(&mut self) {
        while self
            .cursor
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
            self.cursor.next();
        }
    }

    pub fn parse_expr(&mut self) {
        while let Some(token) = self.cursor.peek()
            && token.token_type != TokenType::Eof
        {
            let expr =
                ExpressionParser::parse(&mut self.cursor).inspect_err(|_| self.synchronize());
            self.expr_results.push(expr);
        }
    }

    pub fn print_expr(&self, source_code: impl SourceCode + Clone + 'static) {
        self.expr_results.iter().for_each(|token| match token {
            Ok(token) => println!("{}", token),
            Err(err) => {
                if FANCY_ERROR {
                    eprintln!(
                        "{:?}",
                        miette::Report::new(err.clone()).with_source_code(source_code.clone())
                    )
                } else {
                    eprintln!("{}", err)
                }
            }
        })
    }

    pub fn print_expr_errors(&self, source_code: impl SourceCode + Clone + 'static) {
        self.expr_results.iter().for_each(|expr| {
            if let Err(err) = expr {
                if FANCY_ERROR {
                    eprintln!(
                        "{:?}",
                        miette::Report::new(err.clone()).with_source_code(source_code.clone())
                    )
                } else {
                    eprintln!("{}", err)
                }
            }
        });
    }

    pub fn has_expr_errors(&self) -> bool {
        self.expr_results.iter().any(Result::is_err)
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.cursor.peek()
            && token.token_type != TokenType::Eof
        {
            let expr = StatementParser::parse(&mut self.cursor);
            if expr.is_err() {
                self.synchronize();
            }
            self.results.push(expr);
        }
    }

    pub fn statements(&self) -> Vec<Stmt> {
        self.results
            .clone()
            .into_iter()
            .flat_map(Result::ok)
            .collect()
    }

    pub fn expressions(&self) -> Vec<Expr> {
        self.expr_results
            .clone()
            .into_iter()
            .flat_map(Result::ok)
            .collect()
    }
}

impl StageResult for Parser {
    fn print(&self, source_code: impl SourceCode + Clone + 'static) {
        self.results.iter().for_each(|token| match token {
            Ok(token) => println!("{}", token),
            Err(err) => {
                if FANCY_ERROR {
                    eprintln!(
                        "{:?}",
                        miette::Report::new(err.clone()).with_source_code(source_code.clone())
                    )
                } else {
                    eprintln!("{}", err)
                }
            }
        })
    }

    fn print_errors(&self, source_code: impl SourceCode + Clone + 'static) {
        self.results.iter().for_each(|expr| {
            if let Err(err) = expr {
                if FANCY_ERROR {
                    eprintln!(
                        "{:?}",
                        miette::Report::new(err.clone()).with_source_code(source_code.clone())
                    )
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
