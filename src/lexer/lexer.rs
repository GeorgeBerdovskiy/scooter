use super::{Token, TokenKind};
use crate::shared::{Location, Span};

/// Represents an error that occured during lexing.
pub struct LexError {
    /// The cause of this error.
    pub reason: String,

    /// The (optional) span of this error.
    pub span: Option<Span>,
}

/// Represents the result of lexing.
type LexResult<T> = Result<T, LexError>;

/// Represents the lexing engine.
pub struct Lexer<'a> {
    /// The source code to be lexed.
    source: &'a [char],

    /// Our current index in the `source` slice.
    index: usize,

    /// Our current line (starting at one).
    line: usize,

    /// Our current column (starting at one).
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer instance.
    pub fn new(source: &'a [char]) -> Self {
        Lexer {
            source,
            index: 0,
            line: 1,
            column: 1,
        }
    }

    /// Lex the entire input.
    pub fn lex(&mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut token = self.next()?;

        while token.kind != TokenKind::EOF {
            tokens.push(token.clone());
            token = self.next()?;
        }

        Ok(tokens)
    }

    /// Return the next token.
    pub fn next(&mut self) -> LexResult<Token> {
        while self.current() != '\0' && self.current().is_whitespace() {
            self.step(1);
        }

        if self.current() == '\0' {
            return Ok(Token::spanned(
                TokenKind::EOF,
                Span::single(self.line, self.column),
            ));
        }

        let current = self.current();
        if current.is_alphabetic() || current == '_' {
            let start = self.location();
            let mut end = self.location();
            let mut raw = String::from(self.current());

            self.step(1);

            while self.current().is_alphanumeric() || self.current() == '_' {
                raw += &self.current().to_string();
                end = self.location();
                self.step(1);
            }

            let span = Span::new(start, end);
            match raw.as_str() {
                "fn" => Ok(Token::spanned(TokenKind::KwFn, span)),
                "struct" => Ok(Token::spanned(TokenKind::KwStruct, span)),
                "impl" => Ok(Token::spanned(TokenKind::KwImpl, span)),
                "self" => Ok(Token::spanned(TokenKind::KwSelf, span)),
                "let" => Ok(Token::spanned(TokenKind::KwLet, span)),
                "return" => Ok(Token::spanned(TokenKind::KwRet, span)),
                _ => Ok(Token::spanned(TokenKind::Ident(raw), span)),
            }
        } else if current.is_numeric() {
            let start = self.location();
            let mut end = self.location();
            let mut raw = String::from(self.current());

            self.step(1);

            while self.current().is_numeric() {
                raw += &self.current().to_string();
                end = self.location();
                self.step(1);
            }

            let value: i32 = raw.parse().map_err(|_| LexError {
                reason: format!("Couldn't convert {raw} into an i32"),
                span: Some(Span::new(start.clone(), end.clone())),
            })?;

            Ok(Token::spanned(
                TokenKind::LitNum(value),
                Span::new(start, end),
            ))
        } else {
            // Must be a symbol of some kind
            let start = Location::new(self.line, self.column);
            let mut end: Location = Location::new(self.line, self.column);
            let kind;

            match current {
                // No lookahead (this character is enough)
                '+' => {
                    self.expect('+')?;
                    kind = TokenKind::Plus;
                }

                '*' => {
                    self.expect('*')?;
                    kind = TokenKind::Star;
                }

                '(' => {
                    self.expect('(')?;
                    kind = TokenKind::LParen;
                }

                ')' => {
                    self.expect(')')?;
                    kind = TokenKind::RParen;
                }

                '{' => {
                    self.expect('{')?;
                    kind = TokenKind::LBrace;
                }

                '}' => {
                    self.expect('}')?;
                    kind = TokenKind::RBrace;
                }

                ':' => {
                    self.expect(':')?;
                    kind = TokenKind::Colon;
                }

                ';' => {
                    self.expect(';')?;
                    kind = TokenKind::Semicolon;
                }

                '=' => {
                    self.expect('=')?;
                    kind = TokenKind::Equal;
                }

                ',' => {
                    self.expect(',')?;
                    kind = TokenKind::Comma
                }

                // Single character lookahead (we need to look at the next one)
                '-' => {
                    self.expect('-')?;
                    end = Location::new(self.line, self.column);

                    let current = self.current();
                    if current == '>' {
                        self.expect('>')?;
                        kind = TokenKind::RArrow
                    } else {
                        return Err(Self::unexpected(current, Span::new(start, end)));
                    }
                }

                _ => {
                    return Err(Self::unexpected(current, Span::new(start, end)));
                }
            }

            Ok(Token::spanned(kind, Span::new(start, end)))
        }
    }

    /// Returns a `LexError` for an unexpected character with a span.
    pub fn unexpected(c: char, span: Span) -> LexError {
        LexError {
            reason: format!("Unexpected character '{c}'"),
            span: Some(span),
        }
    }

    /// Return the current character.
    pub fn current(&self) -> char {
        self.lookahead(0)
    }

    /// Return the current location.
    pub fn location(&self) -> Location {
        Location::new(self.line, self.column)
    }

    /// Step to the next valid character.
    fn step(&mut self, n: usize) {
        for _ in 0..n {
            self.index += 1;

            if self.index >= self.source.len() {
                break;
            }

            self.column += 1;

            if self.source[self.index] == '\n' {
                while self.current() == '\n' {
                    self.index += 1;
                    self.line += 1;
                }
                self.column = 1;
            }
        }
    }

    /// Look `n` characters ahead of the cursor.
    fn lookahead(&self, n: usize) -> char {
        if self.index + n >= self.source.len() {
            '\0'
        } else {
            self.source[self.index + n]
        }
    }

    /// Move the cursor if the character matches, return an error otherwise.
    pub fn expect(&mut self, expected: char) -> LexResult<()> {
        let current = self.current();

        if current == expected {
            self.step(1);
            Ok(())
        } else {
            Err(LexError {
                reason: format!("Expected character '{expected}', found '{current}'"),
                span: Some(Span::single(self.line, self.column)),
            })
        }
    }
}
