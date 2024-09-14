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

    /// Create a token that definitely doesn't have a span.
    pub fn unspanned(kind: TokenKind) -> Self {
        Token { kind, span: None }
    }
}

/// Represents a token kind.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    KwFn,          // "fn"
    Ident(String), // "foo", "bar", "baz"
    LitNum(i32),   // "123", "0", "5555"
    Plus,          // +
    Minus,         // -
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
