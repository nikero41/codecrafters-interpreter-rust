use crate::{
    FANCY_ERROR,
    debug::{DebugInfo, Location},
    stages::StageResult,
};
use std::{iter::Peekable, str::Chars};

use miette::SourceCode;

use crate::{
    stages::errors::ScanError,
    token::{Keyword, SPECIAL_START_CHARS, Token, TokenType},
};

pub struct Scanner<'a> {
    cursor: Peekable<Chars<'a>>,
    results: Vec<Result<Token, ScanError>>,
    location: Location,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            cursor: content.chars().peekable(),
            location: Location::new(1, -1, 0),
            results: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        let next_char = self.cursor.next();
        if let Some(char) = next_char {
            self.update_position(char);
        }
        next_char
    }

    fn next_if_eq(&mut self, expected: &char) -> Option<char> {
        let next_char = self.cursor.next_if_eq(expected);
        if let Some(char) = next_char {
            self.update_position(char);
        }
        next_char
    }

    fn update_position(&mut self, char: char) {
        self.location.advance(char);
    }

    fn parse_until_before_eq(&mut self, terminator: &[char]) -> String {
        let mut literal = String::new();
        while let Some(char) = self.cursor.next_if(|char| !terminator.contains(char)) {
            self.update_position(char);
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
            self.update_position(char);
            literal.push(char)
        }
        literal
    }

    fn string(&mut self) -> Result<TokenType, ScanError> {
        let start_offset = self.location.offset;
        let literal = self.parse_until_before_eq(&['\n', '"']);

        match self.next_if_eq(&'"') {
            Some(_) => Ok(TokenType::String(literal)),
            None => Err(ScanError::UnterminatedString {
                line: self.location.line,
                span: (start_offset+1, literal.len()).into(),
                string: literal,
            }),
        }
    }

    fn number(&mut self, initial: char) -> Result<TokenType, ScanError> {
        let result = self.parse_until(|char| char.is_ascii_digit());

        let mut literal = String::from(initial);
        literal.push_str(&result);

        if self.next_if_eq(&'.').is_some()
            && let Some(char) = self.cursor.peek()
        {
            if !char.is_ascii_digit() {
                return Ok(TokenType::Number(literal));
                // NOTE: add this if alphabetical characters are not allowed
                // return Err(ScanError::InvalidCharacter {
                //     line: self.current_line,
                //     character: *char,
                //     span: self.location.offset.into(),
                // });
            }

            literal.push('.');
            let result = self.parse_until(|char| char.is_ascii_digit());
            literal.push_str(&result);
        }

        Ok(TokenType::Number(literal))
        // NOTE: add this if alphabetical characters are not allowed
        // match self.cursor.peek() {
        //     Some(char) if char.is_ascii_alphabetic() => Err(ScanError::InvalidCharacter {
        //         line: self.current_line,
        //         character: *char,
        //         span: (self.location.offset + 1).into(),
        //     }),
        //     _ => Ok(TokenType::Number(literal)),
        // }
    }

    fn identifier(&mut self, initial: char) -> Result<TokenType, ScanError> {
        // println!("🪚 self.location: {:?}", self.location);
        let result = self.parse_until(|char| char.is_ascii_alphanumeric() || char == &'_');

        let mut literal = String::from(initial);
        literal.push_str(&result);

        match literal.parse::<Keyword>() {
            Ok(keyword) => Ok(TokenType::Keyword(keyword)),
            Err(_) => Ok(TokenType::Identifier(literal)),
        }
    }

    pub fn scan(&mut self) {
        while let Some(char) = self.next() {
            let debug = DebugInfo::new(self.location);

            let token_type = match char {
                ' ' | '\t' | '\n' | '\r' => continue,
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
                '/' => match self.next_if_eq(&'/') {
                    Some(_) => {
                        let _ = self.parse_until_before_eq(&['\n']);
                        continue;
                    }
                    None => Ok(TokenType::Slash),
                },
                '=' => match self.next_if_eq(&'=') {
                    Some(_) => Ok(TokenType::Equal),
                    None => Ok(TokenType::Assign),
                },
                '!' => match self.next_if_eq(&'=') {
                    Some(_) => Ok(TokenType::BangEqual),
                    None => Ok(TokenType::Bang),
                },
                '<' => match self.next_if_eq(&'=') {
                    Some(_) => Ok(TokenType::LessEqual),
                    None => Ok(TokenType::Less),
                },
                '>' => match self.next_if_eq(&'=') {
                    Some(_) => Ok(TokenType::GreaterEqual),
                    None => Ok(TokenType::Greater),
                },
                '"' => self.string(),
                x if x.is_ascii_digit() => self.number(x),
                x if x.is_ascii_alphanumeric() || SPECIAL_START_CHARS.contains(&x) => {
                    self.identifier(x)
                }
                x => Err(ScanError::InvalidCharacter {
                    line: self.location.line,
                    character: x,
                    span: self.location.offset.into(),
                }),
            };

            let token = token_type.map(|token_type| Token::new(token_type, debug));

            self.results.push(token);
        }

        let debug = DebugInfo::new(self.location);
        self.results.push(Ok(Token::new(TokenType::Eof, debug)));
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.results
            .clone()
            .into_iter()
            .flat_map(Result::ok)
            .collect()
    }
}

impl StageResult for Scanner<'_> {
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
        });
    }

    fn print_errors(&self, source_code: impl SourceCode + Clone + 'static) {
        self.results.iter().for_each(|token| {
            if let Err(err) = token {
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
