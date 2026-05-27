use std::fmt::Display;

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: Option<&'a str>,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match self.literal {
            Some(literal) => literal,
            None => "null",
        };

        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}

enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    SemiColon,
    Minus,
    Slash,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = match self {
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBrace => "LEFT_BRACE",
            Self::RightBrace => "RIGHT_BRACE",
            Self::Star => "STAR",
            Self::Dot => "DOT",
            Self::Comma => "COMMA",
            Self::Plus => "PLUS",
            Self::SemiColon => "SEMICOLON",
            Self::Minus => "MINUS",
            Self::Slash => "SLASH",
            Self::EOF => "EOF",
        };

        write!(f, "{}", format)
    }
}

pub fn tokenize(content: String) {
    let mut current_line = 1;

    let tokens: Vec<Option<Token>> = content
        .chars()
        .map(|char| match char {
            '(' => Some(Token {
                token_type: TokenType::LeftParen,
                lexeme: "(",
                literal: None,
            }),
            ')' => Some(Token {
                token_type: TokenType::RightParen,
                lexeme: ")",
                literal: None,
            }),
            '{' => Some(Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{",
                literal: None,
            }),
            '}' => Some(Token {
                token_type: TokenType::RightBrace,
                lexeme: "}",
                literal: None,
            }),
            '*' => Some(Token {
                token_type: TokenType::Star,
                lexeme: "*",
                literal: None,
            }),
            '.' => Some(Token {
                token_type: TokenType::Dot,
                lexeme: ".",
                literal: None,
            }),
            ',' => Some(Token {
                token_type: TokenType::Comma,
                lexeme: ",",
                literal: None,
            }),
            '+' => Some(Token {
                token_type: TokenType::Plus,
                lexeme: "+",
                literal: None,
            }),
            ';' => Some(Token {
                token_type: TokenType::SemiColon,
                lexeme: ";",
                literal: None,
            }),
            '-' => Some(Token {
                token_type: TokenType::Minus,
                lexeme: "-",
                literal: None,
            }),
            '/' => Some(Token {
                token_type: TokenType::Slash,
                lexeme: "/",
                literal: None,
            }),
            '\n' => {
                current_line += 1;
                None
            }
            x => {
                eprintln!("[line {}] Error: Unexpected character: {}", current_line, x);
                None
            }
        })
        .chain([Some(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            literal: None,
        })])
        .collect();

    let failed = tokens.iter().any(|token| token.is_none());

    tokens
        .into_iter()
        .filter_map(|token| token)
        .for_each(|token| println!("{}", token));

    if failed {
        std::process::exit(65)
    }
}
