use std::fmt::Display;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
}

impl Display for Token {
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
    String,
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
            Self::String => "STRING",
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
                lexeme: "(".to_string(),
                literal: None,
            }),
            ')' => Some(Token {
                token_type: TokenType::RightParen,
                lexeme: ")".to_string(),
                literal: None,
            }),
            '{' => Some(Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{".to_string(),
                literal: None,
            }),
            '}' => Some(Token {
                token_type: TokenType::RightBrace,
                lexeme: "}".to_string(),
                literal: None,
            }),
            '*' => Some(Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
            }),
            '.' => Some(Token {
                token_type: TokenType::Dot,
                lexeme: ".".to_string(),
                literal: None,
            }),
            ',' => Some(Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
                literal: None,
            }),
            '+' => Some(Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
            }),
            ';' => Some(Token {
                token_type: TokenType::SemiColon,
                lexeme: ";".to_string(),
                literal: None,
            }),
            '-' => Some(Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
            }),
            '/' => match chars.peek() {
                Some('/') => {
                    chars.next();

                    let mut comment = String::new();
                    while let Some(char) = chars.next() {
                        if char == '\n' {
                            current_line += 1;
                            break;
                        }
                        comment.push(char);
                    }

                    None
                }
                _ => Some(Token {
                    token_type: TokenType::Slash,
                    lexeme: "/".to_string(),
                    literal: None,
                }),
            },
            '=' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::Assign,
                        lexeme: "==".to_string(),
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Equal,
                    lexeme: "=".to_string(),
                    literal: None,
                }),
            },
            '!' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::BangEqual,
                        lexeme: "!=".to_string(),
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Bang,
                    lexeme: "!".to_string(),
                    literal: None,
                }),
            },
            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::LessEqual,
                        lexeme: "<=".to_string(),
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Less,
                    lexeme: "<".to_string(),
                    literal: None,
                }),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    Some(Token {
                        token_type: TokenType::GreaterEqual,
                        lexeme: ">=".to_string(),
                        literal: None,
                    })
                }
                _ => Some(Token {
                    token_type: TokenType::Greater,
                    lexeme: ">".to_string(),
                    literal: None,
                }),
            },
            '"' => {
                let mut literal = String::new();
                let mut determined = false;
                while let Some(char) = chars.next() {
                    match char {
                        '\n' => break,
                        '"' => {
                            determined = true;
                            break;
                        }
                        char => literal.push(char),
                    };
                }

                if determined {
                    Some(Token {
                        token_type: TokenType::String,
                        lexeme: format!(r#""{}""#, literal.to_string()),
                        literal: Some(literal.to_string()),
                    })
                } else {
                    eprintln!("[line {}] Error: Unterminated string.", current_line);
                    failed = true;
                    None
                }
            }
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
        lexeme: String::new(),
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
