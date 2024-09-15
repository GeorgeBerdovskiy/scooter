use std::fmt::Display;

use crate::shared::Span;

/// Represents a token.
#[derive(Debug, Clone)]
pub struct Token {
    /// The kind of this token.
    pub kind: TokenKind,

    /// The (optional) span of this token.
    pub span: Option<Span>,
}

impl Token {
    /// Create a token that definitely has a span.
    pub fn spanned(kind: TokenKind, span: Span) -> Self {
        Token {
            kind,
            span: Some(span),
        }
    }
}

/// Represents a token kind.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    KwFn,          // "fn"
    KwLet,         // "let"
    KwRet,         // "return"
    Ident(String), // "foo", "bar", "baz"
    LitNum(i32),   // "123", "0", "5555"
    Plus,          // +
    Star,          // *
    Equal,         // =
    Colon,         // :
    Semicolon,     // ;
    LParen,        // (
    RParen,        // )
    LBrace,        // {
    RBrace,        // }
    RArrow,        // ->
    EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KwFn => write!(f, "'fn'"),
            Self::KwLet => write!(f, "'let'"),
            Self::KwRet => write!(f, "'return'"),
            Self::Ident(str) => write!(f, "identifier '{str}'"),
            Self::LitNum(lit) => write!(f, "literal number '{lit}'"),
            Self::Plus => write!(f, "'+'"),
            Self::Star => write!(f, "'*'"),
            Self::Equal => write!(f, "'='"),
            Self::Colon => write!(f, "':'"),
            Self::Semicolon => write!(f, "';'"),
            Self::LParen => write!(f, "'('"),
            Self::RParen => write!(f, "')'"),
            Self::LBrace => write!(f, "'{{'"),
            Self::RBrace => write!(f, "'}}'"),
            Self::RArrow => write!(f, "'->'"),
            Self::EOF => write!(f, "<EOF>"),
        }
    }
}
