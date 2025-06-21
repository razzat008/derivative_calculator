pub mod tokens;
use std::{iter::Peekable, str::Chars};

use tokens::{Token, WhiteSpace};

#[derive(PartialEq, Debug)]
/// Storing the location of individual expression in a given expression
pub(crate) struct Location {
    pub expr_idx: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { expr_idx: 0 }
    }
}

#[derive(Debug, PartialEq)]
/// Storing the [Token] variant with the location of the token
pub(super) struct TokenWithLocation {
    pub variant: Token,
    pub location: Location,
}

impl TokenWithLocation {
    /// donot reference the location  
    pub fn token_only(self) -> Token {
        self.variant
    }

    /// Reference to &Token
    pub fn token(&self) -> &Token {
        &self.variant
    }
}

/// Handling/storing the token stream:probably from the main function
struct Stream<'i> {
    input: &'i str,
    location: Location,
    chars: Peekable<Chars<'i>>,
}
