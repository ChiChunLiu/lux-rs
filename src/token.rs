use std::fmt;

#[derive(Debug)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,

    Semicolon,
    Slash,
    Star,

    //OneOrTwoCharacterTokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //Literals.
    Identifier,
    String(&'a str),
    Number(f64),

    //Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl<'a> fmt::Display for TokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> ToString for Token<'a> {
    fn to_string(&self) -> String {
        format!("{} {}", self.token_type, self.lexeme)
    }
}
