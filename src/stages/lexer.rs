use crate::{FANCY_ERROR, debug::DebugInfo, stages::StageResult};
use std::{iter::Peekable, str::Chars};

use miette::{SourceOffset, SourceSpan};

use crate::{
    source_file::SourceFile,
    stages::errors::ScanError,
    token::{Keyword, SPECIAL_START_CHARS, Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a SourceFile,
    cursor: Peekable<Chars<'a>>,
    current_line: u32,
    current_column: u32,
    results: Vec<Result<Token, ScanError>>,
}

impl<'a> Scanner<'a> {
    pub fn new(file: &'a SourceFile) -> Self {
        Self {
            source: file,
            cursor: file.content.chars().peekable(),
            current_line: 1,
            current_column: 0,
            results: Vec::new(),
        }
    }

    fn location(&self) -> SourceOffset {
        let start_line = self.current_line as usize;
        let start_column = self.current_column as usize;
        SourceOffset::from_location(&self.source.content, start_line, start_column)
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
        self.current_column += 1;
        if char == '\n' {
            self.current_line += 1;
            self.current_column = 0;
        }
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
        let start_location = self.location();
        let literal = self.parse_until_before_eq(&['\n', '"']);

        match self.next_if_eq(&'"') {
            Some(_) => Ok(TokenType::String(literal)),
            None => Err(ScanError::UnterminatedString {
                line: self.current_line,
                src: self.source.named_source.clone(),
                span: SourceSpan::new(
                    SourceOffset::from(start_location.offset() + 1),
                    literal.len(),
                ),
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
                //     src: self.source.named_source.clone(),
                //     span: self.location().into(),
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
        //         src: self.source.named_source.clone(),
        //         span: (self.location().offset() + 1).into(),
        //     }),
        //     _ => Ok(TokenType::Number(literal)),
        // }
    }

    fn identifier(&mut self, initial: char) -> Result<TokenType, ScanError> {
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
            let start_line = self.current_line;
            let start_column = self.current_column;

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
                    line: self.current_line,
                    character: x,
                    src: self.source.named_source.clone(),
                    span: SourceSpan::new(self.location(), 1),
                }),
            };

            let debug = DebugInfo::new(start_line, start_column);
            let token = token_type.map(|token_type| Token::new(token_type, debug));

            self.results.push(token);
        }

        let debug = DebugInfo::new(self.current_line + 1, 1);
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
    fn print(&self) {
        self.results.iter().for_each(|token| match token {
            Ok(token) => println!("{}", token),
            Err(err) => {
                if FANCY_ERROR {
                    eprintln!("{:?}", miette::Report::new(err.clone()))
                } else {
                    eprintln!("{}", err)
                }
            }
        });
    }

    fn print_errors(&self) {
        self.results.iter().for_each(|token| {
            if let Err(err) = token {
                if FANCY_ERROR {
                    eprintln!("{:?}", miette::Report::new(err.clone()))
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
