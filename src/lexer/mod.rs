use std::{iter::Peekable, str::Chars};

use errors::ParseError;
use token::TokenType;

use crate::lexer::keyword::{Keyword, SPECIAL_START_CHARS};

mod errors;
mod keyword;
mod token;

struct Scanner<'a> {
    cursor: Peekable<Chars<'a>>,
    pub current_line: u32,
    pub current_column: u32,
}

impl<'a> Scanner<'a> {
    fn new(content: &'a String) -> Self {
        Self {
            cursor: content.chars().peekable(),
            current_line: 1,
            current_column: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        let next_char = self.cursor.next();
        self.current_column += 1;
        if next_char == Some('\n') {
            self.current_line += 1;
            self.current_column = 0;
        }
        next_char
    }

    fn match_next(&mut self, mather: char) -> bool {
        match self.cursor.peek() {
            Some(next) if next == &mather => {
                self.cursor.next();
                true
            }
            _ => false,
        }
    }

    fn parse_until(&mut self, literal: &mut String, terminator: &[char]) -> Result<(), ParseError> {
        loop {
            match self.cursor.peek() {
                Some(char) if terminator.contains(&char) => return Ok(()),
                Some(_) => literal.push(self.cursor.next().unwrap()),
                None => return Err(ParseError::EOF),
            };
        }
    }

    fn parse_if<T>(&mut self, literal: &mut String, condition: T) -> Result<(), ParseError>
    where
        T: Fn(&char) -> bool,
    {
        loop {
            match self.cursor.peek() {
                Some(char) if !condition(char) => return Ok(()),
                Some(_) => literal.push(self.cursor.next().unwrap()),
                None => return Err(ParseError::EOF),
            };
        }
    }
}

pub fn tokenize(content: String) -> Vec<Result<TokenType, ParseError>> {
    let mut scanner = Scanner::new(&content);

    let mut tokens: Vec<Result<TokenType, ParseError>> = Vec::new();

    while let Some(char) = scanner.next() {
        let token = match char {
            ' ' | '\t' | '\n' => continue,
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            '*' => Ok(TokenType::Star),
            '.' => Ok(TokenType::Dot),
            ',' => Ok(TokenType::Comma),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::SemiColon),
            '-' => Ok(TokenType::Minus),
            '/' => match scanner.match_next('/') {
                true => {
                    let _ = scanner.parse_until(&mut String::new(), &['\n']);
                    continue;
                }
                false => Ok(TokenType::Slash),
            },
            '=' => match scanner.match_next('=') {
                true => Ok(TokenType::Equal),
                false => Ok(TokenType::Assign),
            },
            '!' => match scanner.match_next('=') {
                true => Ok(TokenType::BangEqual),
                false => Ok(TokenType::Bang),
            },
            '<' => match scanner.match_next('=') {
                true => Ok(TokenType::LessEqual),
                false => Ok(TokenType::Less),
            },
            '>' => match scanner.match_next('=') {
                true => Ok(TokenType::GreaterEqual),
                false => Ok(TokenType::Greater),
            },
            '"' => {
                let mut literal = String::new();
                let _ = scanner.parse_until(&mut literal, &['\n', '"']);

                match scanner.match_next('"') {
                    true => Ok(TokenType::String(literal)),
                    false => Err(ParseError::UnterminatedString {
                        line: scanner.current_line,
                        string: literal,
                    }),
                }
            }
            x if x.is_numeric() => {
                let mut literal = String::from(x);
                let _ = scanner.parse_if(&mut literal, |char| char.is_numeric());

                match scanner.match_next('.') {
                    true => {
                        literal.push('.');
                        let _ = scanner.parse_if(&mut literal, |char| char.is_numeric());
                        Ok(TokenType::Number(literal))
                    }
                    false => Ok(TokenType::Number(literal)),
                }
            }
            x if x.is_alphabetic() || SPECIAL_START_CHARS.contains(&x) => {
                let mut literal = String::from(x);
                let _ =
                    scanner.parse_if(&mut literal, |char| char.is_alphanumeric() || char == &'_');

                match literal.parse::<Keyword>() {
                    Ok(keyword) => Ok(TokenType::Keyword(keyword)),
                    Err(_) => Ok(TokenType::Identifier(literal)),
                }
            }
            x => Err(ParseError::InvalidCharacter {
                line: scanner.current_line,
                character: x,
            }),
        };

        tokens.push(token);
    }

    tokens.push(Ok(TokenType::EOF));

    tokens
}
