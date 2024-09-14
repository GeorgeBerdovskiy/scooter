/// Indicates the start and end locations of a construct in the source code.
#[derive(Debug, Clone)]
pub struct Span {
    /// Starting location of this construct.
    pub start: Location,

    /// Ending location of this construct.
    pub end: Location,
}

impl Span {
    /// Create a span that covers multiple characters.
    pub fn new(start: Location, end: Location) -> Self {
        Span { start, end }
    }

    /// Create a span that only covers one character. This is mostly used by the lexer.
    pub fn single(line: usize, column: usize) -> Self {
        Span {
            start: Location::new(line, column),
            end: Location::new(line, column),
        }
    }
}

/// Represents a location in the source code.
#[derive(Debug, Clone)]
pub struct Location {
    /// Line of this location (starting at one).
    pub line: usize,

    /// Column of this location (starting at one).
    pub column: usize,
}

impl Location {
    /// Create a new location given its `line` and `column`.
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }
}
