use miette::SourceCode;

use crate::FANCY_ERROR;
use crate::ast::declaration::{Declaration, DeclarationParser};
use crate::{
    ast::expression::Expr,
    stages::StageResult,
    stages::errors::ParseError,
    token::TokenStream,
    token::{Keyword, Token, TokenType},
};

mod expr_parser;

pub struct Parser {
    cursor: TokenStream,
    expr_results: Vec<Result<Expr, ParseError>>,
    results: Vec<Result<Declaration, ParseError>>,
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
            if self.cursor.next().is_none() {
                return;
            }
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.cursor.peek()
            && token.token_type != TokenType::Eof
        {
            let stmt = DeclarationParser::parse(&mut self.cursor);

            if let Err(parse_error) = &stmt
                && parse_error.needs_sync()
            {
                self.synchronize();
            }
            self.results.push(stmt.clone());
        }
    }

    pub fn results(&self) -> Vec<Declaration> {
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
