use std::{iter::Peekable, str::Chars};

pub use errors::ScanError;
use keyword::{Keyword, SPECIAL_START_CHARS};
use token::{Token, TokenType};

mod errors;
pub mod keyword;
pub mod token;

struct Scanner<'a> {
    cursor: Peekable<Chars<'a>>,
    pub current_line: u32,
    pub current_column: u32,
}

impl<'a> Scanner<'a> {
    fn new(content: &'a str) -> Self {
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

    fn parse_until_eq(&mut self, terminator: &[char]) -> String {
        let mut literal = String::new();
        while let Some(char) = self.cursor.next_if(|char| !terminator.contains(char)) {
            literal.push(char);
        }
        literal
    }

    fn parse_until<T>(&mut self, condition: T) -> String
    where
        T: Fn(&char) -> bool,
    {
        let mut literal = String::new();
        while let Some(char) = self.cursor.next_if(&condition) {
            literal.push(char)
        }
        literal
    }
}

pub fn tokenize(content: &str) -> Vec<Result<Token, ScanError>> {
    let mut scanner = Scanner::new(content);

    let mut tokens: Vec<Result<Token, ScanError>> = Vec::new();

    while let Some(char) = scanner.next() {
        let token_type = match char {
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
            '/' => match scanner.cursor.next_if_eq(&'/') {
                Some(_) => {
                    let _ = scanner.parse_until_eq(&['\n']);
                    continue;
                }
                None => Ok(TokenType::Slash),
            },
            '=' => match scanner.cursor.next_if_eq(&'=') {
                Some(_) => Ok(TokenType::Equal),
                None => Ok(TokenType::Assign),
            },
            '!' => match scanner.cursor.next_if_eq(&'=') {
                Some(_) => Ok(TokenType::BangEqual),
                None => Ok(TokenType::Bang),
            },
            '<' => match scanner.cursor.next_if_eq(&'=') {
                Some(_) => Ok(TokenType::LessEqual),
                None => Ok(TokenType::Less),
            },
            '>' => match scanner.cursor.next_if_eq(&'=') {
                Some(_) => Ok(TokenType::GreaterEqual),
                None => Ok(TokenType::Greater),
            },
            '"' => {
                let literal = scanner.parse_until_eq(&['\n', '"']);

                match scanner.cursor.next_if_eq(&'"') {
                    Some(_) => Ok(TokenType::String(literal)),
                    None => Err(ScanError::UnterminatedString {
                        line: scanner.current_line,
                        span: (22..25).into(),
                        string: literal,
                    }),
                }
            }
            x if x.is_numeric() => {
                let result = scanner.parse_until(|char| char.is_numeric());

                let mut literal = String::from(x);
                literal.push_str(&result);

                match scanner.cursor.next_if_eq(&'.') {
                    Some(char) => {
                        literal.push(char);
                        let result = scanner.parse_until(|char| char.is_numeric());
                        literal.push_str(&result);
                        Ok(TokenType::Number(literal))
                    }
                    None => Ok(TokenType::Number(literal)),
                }
            }
            x if x.is_alphabetic() || SPECIAL_START_CHARS.contains(&x) => {
                let result = scanner.parse_until(|char| char.is_alphanumeric() || char == &'_');

                let mut literal = String::from(x);
                literal.push_str(&result);

                match literal.parse::<Keyword>() {
                    Ok(keyword) => Ok(TokenType::Keyword(keyword)),
                    Err(_) => Ok(TokenType::Identifier(literal)),
                }
            }
            x => Err(ScanError::InvalidCharacter {
                line: scanner.current_line,
                character: x,
            }),
        };

        let token = token_type
            .map(|token_type| token_type.to_token(scanner.current_line, scanner.current_column));

        tokens.push(token);
    }

    tokens.push(Ok(
        TokenType::EOF.to_token(scanner.current_line, scanner.current_column)
    ));

    tokens
}
