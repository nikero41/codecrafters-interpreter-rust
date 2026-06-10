use crate::lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Literal::Number(number) => write!(f, "{}", number),
            Literal::String(string) => write!(f, "\"{}\"", string),
            Literal::Bool(boolean) => write!(f, "{}", boolean),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

// TODO:
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

// expression     → literal | unary | binary | grouping ;
#[derive(Debug)]
pub enum Expr {
    // literal → NUMBER | STRING | "true" | "false" | "nil" ;
    Literal(Literal),
    // grouping → "(" expression ")" ;
    Grouping(Box<Expr>),
    // unary → ( "-" | "!" ) expression ;
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    // binary → expression operator expression ;
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::Literal(literal) => match literal {
                Literal::Number(float) => write!(f, "{:?}", float),
                Literal::String(literal) => write!(f, "{}", literal),
                Literal::Bool(boolean) => {
                    if *boolean {
                        write!(f, "true")
                    } else {
                        write!(f, "false")
                    }
                }
                Literal::Nil => write!(f, "nil"),
            },
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.token_type.lexeme(), right)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.token_type.lexeme(), left, right)
            }
        }
    }
}

impl Expr {
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
        }
    }
}

pub trait Visitor<R> {
    fn visit_literal(&mut self, literal: &Literal) -> R;
    fn visit_grouping(&mut self, expr: &Expr) -> R;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> R;
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
}
