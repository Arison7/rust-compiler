use core::panic;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    // Keywords
    If,
    Else,
    Int,

    // Literals
    IntegerLiteral(usize),
    StringLiteral(String),

    // Identifiers
    Identifier(String),

    // Operators
    Plus,
    Assign,
    Negation,
    BitwiseComplement,
    LogicalNegation,

    // Punctuation
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    // Keywords
    ReturnKeyword,

    // Logical Operators
    GreaterThan,
    LessThan,

    Err(String),
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    // Keywords
    If,
    Else,
    Int,
    ReturnKeyword,

    // Literals
    IntegerLiteral,
    StringLiteral,

    // Identifiers
    Identifier,

    // Operators
    Plus,
    Assign,
    Negation,
    BitwiseComplement,
    LogicalNegation,

    // Punctuation
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    // Logical Operators
    GreaterThan,
    LessThan,

    Err,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match self {
            // Keywords
            Token::If => TokenKind::If,
            Token::Else => TokenKind::Else,
            Token::Int => TokenKind::Int,
            Token::ReturnKeyword => TokenKind::ReturnKeyword,

            // Literals
            Token::IntegerLiteral(_) => TokenKind::IntegerLiteral,
            Token::StringLiteral(_) => TokenKind::StringLiteral,

            // Identifiers
            Token::Identifier(_) => TokenKind::Identifier,

            // Operators
            Token::Plus => TokenKind::Plus,
            Token::Assign => TokenKind::Assign,
            Token::Negation => TokenKind::Negation,
            Token::BitwiseComplement => TokenKind::BitwiseComplement,
            Token::LogicalNegation => TokenKind::LogicalNegation,

            // Punctuation
            Token::Semicolon => TokenKind::Semicolon,
            Token::OpenParen => TokenKind::OpenParen,
            Token::CloseParen => TokenKind::CloseParen,
            Token::OpenBrace => TokenKind::OpenBrace,
            Token::CloseBrace => TokenKind::CloseBrace,

            // Logical Operators
            Token::GreaterThan => TokenKind::GreaterThan,
            Token::LessThan => TokenKind::LessThan,

            Token::Err(_) => TokenKind::Err,
        }
    }

    pub fn get_token(token: &str, value: &str) -> Token {
        match token {
            // Keywords
            "If" => Token::If,
            "Else" => Token::Else,
            "Int" => Token::Int,
            "ReturnKeyword" => Token::ReturnKeyword,

            // Operators
            "Plus" => Token::Plus,
            "Assign" => Token::Assign,
            "Negation" => Token::Negation,
            "BitwiseComplement" =>Token::BitwiseComplement,
            "LogicalNegation" =>Token::LogicalNegation,

            // Punctuation
            "Semicolon" => Token::Semicolon,
            "OpenParen" => Token::OpenParen,
            "CloseParen" => Token::CloseParen,
            "OpenBrace" => Token::OpenBrace,
            "CloseBrace" => Token::CloseBrace,

            // Logical Operators
            "GreaterThan" => Token::GreaterThan,
            "LessThan" => Token::LessThan,

            // Literals and Identifiers
            "IntegerLiteral" => Token::IntegerLiteral(value.parse::<usize>().unwrap()),
            "StringLiteral" => Token::StringLiteral(value.to_string()),
            "Identifier" => Token::Identifier(value.to_string()),

            // Default fallback
            _ => Token::Err(value.to_string()),
        }
    }

    pub fn get_regex(token: &str) -> &str {
        match token {
            "OpenBrace" => r"^\{",
            "CloseBrace" => r"^}",
            "OpenParen" => r"^\(",
            "CloseParen" => r"^\)",
            "Semicolon" => r"^;",
            "Int" => r"^int",
            "ReturnKeyword" => r"^return",
            "Negation" => r"^-",
            "BitwiseComplement" => r"^~",
            "LogicalNegation" => r"^!",
            "Identifier" => r"^[a-zA-Z]\w*",
            "IntegerLiteral" => r"^[0-9]+",
            _ => panic!("Non existing token passed"),
        }
    }

    pub fn get_options() -> Vec<&'static str> {
        vec![
            "OpenBrace",
            "CloseBrace",
            "OpenParen",
            "CloseParen",
            "Semicolon",
            "Int",
            "ReturnKeyword",
            "Identifier",
            "IntegerLiteral",
            "Negation",
            "BitwiseComplement",
            "LogicalNegation",
        ]
    }
}
