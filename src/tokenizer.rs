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
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = match self {
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBrace => "LEFT_BRACE",
            Self::RightBrace => "RIGHT_BRACE",
            Self::EOF => "EOF",
        };

        write!(f, "{}", format)
    }
}

pub fn tokenize(content: String) {
    let mut tokens: Vec<Token> = content
        .chars()
        .filter_map(|char| match char {
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
            x => None,
        })
        .collect();

    tokens.push(Token {
        token_type: TokenType::EOF,
        lexeme: "",
        literal: None,
    });

    tokens
        .iter()
        .for_each(|token| println!("{}", token))
}
