use std::fmt::Display;

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: Option<String>,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.literal {
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
    Equal,
    Assign,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Comment,
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
            Self::Equal => "EQUAL",
            Self::Assign => "EQUAL_EQUAL",
            Self::Bang => "BANG",
            Self::BangEqual => "BANG_EQUAL",
            Self::Less => "LESS",
            Self::LessEqual => "LESS_EQUAL",
            Self::Greater => "GREATER",
            Self::GreaterEqual => "GREATER_EQUAL",
            Self::Comment => "COMMENT",
            Self::EOF => "EOF",
        };

        write!(f, "{}", format)
    }
}

pub fn tokenize(content: String) {
    let mut current_line = 1;
    let mut failed = false;

    let mut chars = content.chars().peekable();

    let mut tokens: Vec<Option<Token>> = Vec::new();

    while let Some(char) = chars.next() {
        let token = match char {
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
            '/' => match chars.peek() {
                Some('/') => {
                    chars.next();

                    let mut comment = String::new();
                    while let Some(char) = chars.next() {
                        if char == '\n' {
                            break;
                        }
                        comment.push(char);
                    }

                    None
                    // Some(Token {
                    //     token_type: TokenType::Comment,
                    //     lexeme: "//",
                    //     literal: Some(comment.trim().to_string()),
                    // })
                }
                _ => Some(Token {
                    token_type: TokenType::Slash,
                    lexeme: "/",
                    literal: None,
                }),
            },
            '=' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::Assign,
                        lexeme: "==",
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Equal,
                    lexeme: "=",
                    literal: None,
                }),
            },
            '!' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::BangEqual,
                        lexeme: "!=",
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Bang,
                    lexeme: "!",
                    literal: None,
                }),
            },
            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::LessEqual,
                        lexeme: "<=",
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Less,
                    lexeme: "<",
                    literal: None,
                }),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::GreaterEqual,
                        lexeme: ">=",
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Greater,
                    lexeme: ">",
                    literal: None,
                }),
            },
            ' ' | '\t' => None,
            '\n' => {
                current_line += 1;
                None
            }
            x => {
                eprintln!("[line {}] Error: Unexpected character: {}", current_line, x);
                failed = true;
                None
            }
        };

        tokens.push(token);
    }

    tokens.push(Some(Token {
        token_type: TokenType::EOF,
        lexeme: "",
        literal: None,
    }));

    tokens
        .into_iter()
        .filter_map(|token| token)
        .for_each(|token| println!("{}", token));

    if failed {
        std::process::exit(65)
    }
}
