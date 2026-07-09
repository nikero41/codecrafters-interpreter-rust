use miette::SourceCode;

use crate::{FANCY_ERROR, ast::expression::ExpressionParser, stages::Parser, token::TokenType};

impl Parser {
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
}
