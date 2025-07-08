#![allow(unused_variables)]
pub mod tokens;
use std::fmt::Display;
use std::{iter::Peekable, str::Chars};

use tokens::{Token, WhiteSpace};

/// Token location: tracks only index for single-line input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Location {
    /// Index in the input string, starting at 0.
    pub expr_idx: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { expr_idx: 0 }
    }
}

/// Stores both the [`Token`] and its starting location in the input string.
#[derive(Debug, PartialEq)]
pub(super) struct TokenWithLocation {
    pub variant: Token,
    pub location: Location,
}

impl TokenWithLocation {
    /// Discards the location.
    pub fn token_only(self) -> Token {
        self.variant
    }

    /// Reference to [`Token`].
    pub fn token(&self) -> &Token {
        &self.variant
    }
}

/// Token stream for symbolic math expressions.
struct Stream<'i> {
    /// Original string input.
    input: &'i str,
    /// Current location in the stream.
    location: Location,
    /// Character input.
    chars: Peekable<Chars<'i>>,
}

impl<'i> Stream<'i> {
    /// Creates a new stream over `input`.
    fn new(input: &'i str) -> Self {
        Self {
            input,
            location: Location { expr_idx: 0 },
            chars: input.chars().peekable(),
        }
    }

    /// Consumes the next value updating [`Self::location`] in the process.
    fn next(&mut self) -> Option<char> {
        self.chars.next().inspect(|_chr| {
            self.location.expr_idx += 1;
        })
    }

    /// Returns a reference to the next character in the stream without consuming it.
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Consumes one character in the stream and returns a reference to the next one without consuming it.
    fn peek_next(&mut self) -> Option<&char> {
        self.next();
        self.peek()
    }

    /// Take characters while predicate is true, return as String.
    fn take_while<P: FnMut(&char) -> bool>(&mut self, mut predicate: P) -> String {
        let mut result = String::new();
        while let Some(&chr) = self.peek() {
            if predicate(&chr) {
                result.push(chr);
                self.next();
            } else {
                break;
            }
        }
        result
    }

    /// Current location in the stream.
    fn location(&self) -> Location {
        self.location
    }
}

/// Possible syntax errors for the symbolic math tokenizer.
#[derive(Debug, PartialEq)]
pub(crate) enum ErrorKind {
    UnexpectedOrUnsupportedToken(char),
    OperatorNotClosed(Token),
    Other(String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedOrUnsupportedToken(token) => {
                write!(f, "unexpected or unsupported token '{token}'")
            }
            ErrorKind::OperatorNotClosed(operator) => write!(f, "'{operator}' operator not closed"),
            ErrorKind::Other(message) => f.write_str(message),
        }
    }
}

/// If the tokenizer finds an error it means the syntax is not correct.
#[derive(Debug, PartialEq)]
pub(super) struct TokenizeError {
    pub kind: ErrorKind,
    pub location: Location,
    pub input: String,
}

/// Main parsing structure for symbolic math expressions.
pub(super) struct Tokenizer<'i> {
    stream: Stream<'i>,
    reached_eof: bool,
}

type TokenResult = Result<Token, TokenizeError>;

impl<'i> Tokenizer<'i> {
    /// Creates a new tokenizer for the given `input`.
    pub fn new(input: &'i str) -> Self {
        Self {
            stream: Stream::new(input),
            reached_eof: false,
        }
    }

    /// Creates an iterator over self.
    pub fn iter<'t>(&'t mut self) -> Iter<'t, 'i> {
        Iter { tokenizer: self }
    }

    /// Tokenizes the entire input and returns a vector of tokens or the first error.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizeError> {
        let mut tokens = Vec::new();
        for result in self.iter() {
            match result {
                Ok(token_with_loc) => tokens.push(token_with_loc.token_only()),
                Err(e) => return Err(e),
            }
        }
        Ok(tokens)
    }

    /// Returns None once Token::EOF has been returned.
    fn optional_next_token_with_location(
        &mut self,
    ) -> Option<Result<TokenWithLocation, TokenizeError>> {
        if !self.reached_eof {
            Some(self.next_token_with_location())
        } else {
            None
        }
    }

    /// Returns the next token and its starting location.
    fn next_token_with_location(&mut self) -> Result<TokenWithLocation, TokenizeError> {
        let location = self.stream.location();
        self.next_token().map(|token| TokenWithLocation {
            variant: token,
            location,
        })
    }

    /// Consumes and returns the next Token variant in the stream.
    fn next_token(&mut self) -> TokenResult {
        let Some(chr) = self.stream.peek() else {
            self.reached_eof = true;
            return Ok(Token::EOF);
        };

        match chr {
            ' ' => self.consume(Token::WhiteSpace(WhiteSpace::SPACE)),
            '\t' => self.consume(Token::WhiteSpace(WhiteSpace::TAB)),
            '\n' => self.consume(Token::WhiteSpace(WhiteSpace::NEWLINE)),
            '+' => self.consume(Token::PLUS),
            '-' => self.consume(Token::MINUS),
            '*' => self.consume(Token::MUL),
            '/' => self.consume(Token::DIV),
            '^' => self.consume(Token::POW),
            '(' => self.consume(Token::LEFTPAREN),
            ')' => self.consume(Token::RIGHTPAREN),
            '0'..='9' => self.tokenize_number(),
            'x' => self.tokenize_variable(),
            'a'..='w' | 'y'..='z' | 'A'..='W' | 'Y'..='Z' => self.tokenize_constants(),
            _ => {
                let error_kind = ErrorKind::UnexpectedOrUnsupportedToken(*chr);
                self.error(error_kind)
            }
        }
    }

    /// Consumes one character and returns Ok(token).
    fn consume(&mut self, token: Token) -> TokenResult {
        self.stream.next();
        Ok(token)
    }

    /// Builds an error with the current location.
    fn error(&self, kind: ErrorKind) -> TokenResult {
        Err(TokenizeError {
            kind,
            location: self.stream.location(),
            input: self.stream.input.to_owned(),
        })
    }

    /// Tokenizes a multi-digit integer.
    fn tokenize_number(&mut self) -> TokenResult {
        let num_str = self.stream.take_while(|chr| chr.is_ascii_digit());
        if let Ok(val) = num_str.parse::<i32>() {
            Ok(Token::NUMBER(val))
        } else {
            self.error(ErrorKind::Other("Invalid number".to_string()))
        }
    }

    /// Tokenizes a variable (single alphabetic character).
    fn tokenize_variable(&mut self) -> TokenResult {
        let var_str = self.stream.take_while(|&chr| chr == 'x');
        if var_str.len() == 1 {
            Ok(Token::VARIABLE(var_str.chars().next().unwrap()))
        } else {
            self.error(ErrorKind::Other("Invalid variable".to_string()))
        }
    }

    fn tokenize_constants(&mut self) -> TokenResult {
        if let Some(&chr) = self.stream.peek() {
            if ('a'..='z').contains(&chr) && chr != 'x' {
                self.stream.next();
                Ok(Token::CONSTANT(chr))
            } else {
                self.error(ErrorKind::Other(
                    "Neither a constant nor a variable!".to_string(),
                ))
            }
        } else {
            self.error(ErrorKind::Other("Unexpected end of input!".to_string()))
        }
    }
}

/// Iterator over Tokenizer yielding Result<TokenWithLocation, TokenizeError>
pub(super) struct Iter<'t, 'i> {
    tokenizer: &'t mut Tokenizer<'i>,
}

impl<'t, 'i> Iterator for Iter<'t, 'i> {
    type Item = Result<TokenWithLocation, TokenizeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenizer.optional_next_token_with_location()
    }
}
