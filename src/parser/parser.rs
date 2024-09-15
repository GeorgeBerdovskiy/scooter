use crate::ast::{
    BinaryOp, Block, CallFn, Expr, ExprBin, ExprCall, ExprLit, File, Ident, Item, ItemFn, LitNum,
    Local, OpKind, Return, Stmt, Ty,
};
use crate::lexer::{Token, TokenKind};
use crate::shared::Span;

/// Represents an error that occured during parsing].
pub struct ParseError {
    /// The cause of this error.
    pub reason: String,

    /// The (optional) span of this error.
    pub span: Option<Span>,
}

/// Represents the result of parsing.
type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    /// The tokens of an entire file.
    input: &'a [Token],

    /// The index of the current token.
    index: usize,

    /// The current span.
    starts: Vec<Span>,
}

impl<'a> Parser<'a> {
    /// Create a new parser.
    pub fn new(input: &'a [Token]) -> Self {
        Parser {
            input,
            index: 0,
            starts: vec![],
        }
    }

    /// Parse an entire file.
    pub fn parse_file(&mut self) -> ParseResult<File> {
        self.start();

        let mut items: Vec<Item> = Vec::new();

        while self.current_kind() != &TokenKind::EOF && self.current_kind() != &TokenKind::RBrace {
            items.push(self.parse_item()?);
        }

        Ok(File {
            items,
            span: self.end(),
        })
    }

    /// Parse an item.
    pub fn parse_item(&mut self) -> ParseResult<Item> {
        let kind = self.current_kind();

        if kind == &TokenKind::KwFn {
            self.parse_item_fn()
        } else {
            Err(ParseError {
                reason: format!("Expected 'fn' or 'mod', found '{kind}'"),
                span: Some(self.end()),
            })
        }
    }

    /// Parse a function declaration.
    pub fn parse_item_fn(&mut self) -> ParseResult<Item> {
        // Start a new span
        self.start();

        // Consume the `fn` token
        let kw = self.expect(TokenKind::KwFn)?.clone();

        // Read the identifier
        let ident = self.parse_ident()?;

        // Read the parameters
        let lp = self.expect(TokenKind::LParen)?.clone();
        let rp = self.expect(TokenKind::RParen)?.clone();

        // Read the return type
        let arrow = self.expect(TokenKind::RArrow)?.clone();
        let ty = self.parse_ty()?;

        // Read the brackets and function body
        let body = self.parse_block()?;

        Ok(Item::Fn(ItemFn {
            kw,
            ident,
            lp,
            rp,
            arrow,
            ty,
            body,
            span: self.end(),
        }))
    }

    /// Parse an identifier.
    pub fn parse_ident(&mut self) -> ParseResult<Ident> {
        let current = self.current().clone();

        match current.kind {
            TokenKind::Ident(raw) => {
                self.advance(1);
                Ok(Ident {
                    repr: raw,
                    span: current.span.unwrap(),
                })
            }
            _ => Err(ParseError {
                reason: format!("Expected an identifier, found {}", current.kind),
                span: current.span,
            }),
        }
    }

    /// Parse a block of statements enclosed by curly braces.
    pub fn parse_block(&mut self) -> ParseResult<Block> {
        self.start();

        // Get the left curly brace
        let lc = self.expect(TokenKind::LBrace)?.clone();

        // Collect the statements
        let mut stmts = Vec::new();
        while self.current_kind() != &TokenKind::RBrace {
            stmts.push(self.parse_stmt()?);
            self.expect(TokenKind::Semicolon)?;
        }

        // Get the right curly brace
        let rc = self.expect(TokenKind::RBrace)?.clone();

        Ok(Block { lc, stmts, rc })
    }

    /// Parse a statement.
    fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        let current = self.current().clone();

        match current.kind {
            TokenKind::KwLet => Ok(Stmt::Local(self.parse_local()?)),
            TokenKind::KwRet => Ok(Stmt::Return(self.parse_return()?)),
            _ => Err(ParseError {
                reason: format!("Unknown statement beginning with {}", current.kind),
                span: current.span,
            }),
        }
    }

    /// Parse a return statement.
    fn parse_return(&mut self) -> ParseResult<Return> {
        self.start();

        Ok(Return {
            kw: self.expect(TokenKind::KwRet)?.clone(),
            expr: self.parse_expr()?,
            span: self.end(),
        })
    }

    /// Parse a local `let` binding.
    fn parse_local(&mut self) -> ParseResult<Local> {
        self.start();

        Ok(Local {
            kw: self.expect(TokenKind::KwLet)?.clone(),
            ident: self.parse_ident()?,
            colon: self.expect(TokenKind::Colon)?.clone(),
            ty: self.parse_ty()?,
            eq: self.expect(TokenKind::Equal)?.clone(),
            expr: self.parse_expr()?,
            span: self.end(),
        })
    }

    /// Parse an expression (`expr ::= term { "+" term }`).
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_term()?;

        while self.current_kind() == &TokenKind::Plus {
            let op = self.expect(TokenKind::Plus)?;
            let op = BinaryOp {
                kind: OpKind::Add,
                span: op.span.clone().unwrap(),
            };

            let rhs = self.parse_term()?;
            let start = expr.span().clone().start;
            let end = rhs.span().clone().end;

            expr = Expr::Binary(ExprBin {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                span: Span::new(start, end),
            })
        }

        Ok(expr)
    }

    /// Parse a term (`term ::= factor { "*" factor }`).
    pub fn parse_term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_factor()?;

        while self.current_kind() == &TokenKind::Star {
            let op = self.expect(TokenKind::Star)?;
            let op = BinaryOp {
                kind: OpKind::Multiply,
                span: op.span.clone().unwrap(),
            };

            let rhs = self.parse_factor()?;
            let start = expr.span().clone().start;
            let end = rhs.span().clone().end;

            expr = Expr::Binary(ExprBin {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                span: Span::new(start, end),
            })
        }

        Ok(expr)
    }

    /// Parse a factor (`factor ::= lit-num | ident | call-fn | "(" expr ")"`).
    fn parse_factor(&mut self) -> ParseResult<Expr> {
        self.start();
        let current = self.current().clone();

        match current.kind {
            TokenKind::LitNum(value) => {
                self.advance(1);

                Ok(Expr::Lit(ExprLit::Num(LitNum {
                    value,
                    span: self.end(),
                })))
            }

            TokenKind::Ident(repr) => {
                let ident = self.parse_ident()?;

                if self.current_kind() == &TokenKind::LParen {
                    Ok(Expr::Call(ExprCall::Fn(CallFn {
                        ident,
                        lp: self.expect(TokenKind::LParen)?.clone(),
                        rp: self.expect(TokenKind::RParen)?.clone(),
                        span: self.end(),
                    })))
                } else {
                    Ok(Expr::Ident(Ident {
                        repr,
                        span: current.span.unwrap(),
                    }))
                }
            }

            _ => Err(ParseError {
                reason: format!(
                    "Expected an identifier, literal or function call... found {}",
                    current.kind
                ),
                span: Some(self.end()),
            }),
        }
    }

    /// Parse a type.
    fn parse_ty(&mut self) -> ParseResult<Ty> {
        self.start();

        Ok(Ty {
            ident: self.parse_ident()?,
            span: self.end(),
        })
    }

    /// Start a span at the current location.
    fn start(&mut self) {
        let span = self.input[self.index].clone().span.unwrap();
        self.starts.push(span);
    }

    /// End a span at the current location.
    fn end(&mut self) -> Span {
        let from = self.starts.last().cloned().unwrap();
        let to = self.input[self.index].clone().span.unwrap();

        self.starts.pop();
        Span::new(from.start, to.end)
    }

    /// Get the kind of the current token.
    fn current_kind(&self) -> &TokenKind {
        &self.input[self.index].kind
    }

    /// Get the current token.
    fn current(&self) -> &Token {
        &self.input[self.index]
    }

    /// Advance the token `n` times.
    fn advance(&mut self, n: usize) {
        if self.index + n >= self.input.len() {
            self.index = self.input.len() - 1;
        } else {
            self.index += n;
        }
    }

    /// Return the current token if its kind matches `kind`, or an error otherwise.
    fn expect(&mut self, kind: TokenKind) -> ParseResult<&Token> {
        if self.current_kind() == &kind {
            self.advance(1);
            return Ok(self.current());
        }

        Err(ParseError {
            reason: format!("Expected {kind} but found {}", self.current_kind()),
            span: self.current().span.clone(),
        })
    }
}
