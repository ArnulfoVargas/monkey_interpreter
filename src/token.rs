use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbraces,
    Rbraces,

    Function,
    Let,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Assign => write!(f, "="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Illegal => write!(f, "Ilegal"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Lbraces => write!(f, "{{"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Lparen => write!(f, ")"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Rbraces => write!(f, "}}"),
            TokenKind::Rparen => write!(f, ")"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::GreaterThan => write!(f, ">"),
        }
    }
}

pub fn lookup_ident(ident : &String) -> TokenKind {
    match ident.as_str(){
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        _ => TokenKind::Ident,
    }
}
