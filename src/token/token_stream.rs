use std::iter::Peekable;
use std::vec::IntoIter;

use crate::token::{Token, TokenType};

pub struct TokenStream {
    cursor: Peekable<IntoIter<Token>>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            cursor: tokens.into_iter().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.cursor.peek()
    }

    pub fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<Token> {
        match self.cursor.peek() {
            Some(token) if token_types.contains(&token.token_type) => self.cursor.next(),
            _ => None,
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.next()
    }
}
