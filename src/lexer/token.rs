use crate::shared::Span;

/// Represents a token.
#[derive(Clone)]
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
#[derive(Clone, PartialEq)]
pub enum TokenKind {
    KwFn,      // "fn"
    Ident,     // "foo", "bar", "baz"
    LitNum,    // "123", "0", "5555"
    Plus,      // +
    Minus,     // -
    Star,      // *
    Equal,     // =
    Colon,     // :
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    EOF,
}
