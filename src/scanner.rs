use ouroboros::self_referencing;
use std::iter::Peekable;
use std::ops::Range;
use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub data: String,
    pub line: usize,
}

#[derive(Debug)]
pub enum TokenType {
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
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    ERROR,
    EOF,
}

#[self_referencing]
pub struct InnerSrc {
    src: String,
    #[borrows(src)]
    #[not_covariant]
    src_chars: Peekable<Chars<'this>>,
}

impl InnerSrc {
    fn from(src: String) -> Self {
        InnerSrcBuilder {
            src,
            src_chars_builder: |src: &String| src.chars().peekable(),
        }
        .build()
    }

    fn get(&self, range: Range<usize>) -> Option<&str> {
        self.borrow_src().get(range)
    }

    fn peek(&mut self) -> Option<&char> {
        self.with_src_chars_mut(|chars: &mut Peekable<Chars>| chars.peek())
    }

    fn next(&mut self) -> Option<char> {
        self.with_src_chars_mut(|chars: &mut Peekable<Chars>| chars.next())
    }
}

pub struct Scanner {
    source: InnerSrc,
    start: usize,
    offset: usize,
    line: usize,
    end: usize,
}

impl Scanner {
    pub fn from(source: String) -> Self {
        let end = source.chars().count();
        Self {
            source: InnerSrc::from(source),
            start: 0,
            offset: 0,
            line: 1,
            end,
        }
    }

    pub fn end(&self) -> bool {
        self.offset >= self.end
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            data: self
                .source
                .get(self.start..self.offset)
                .unwrap_or("")
                .to_string(),
            line: self.line,
        }
    }

    fn error_token(&self, data: String) -> Token {
        Token {
            token_type: TokenType::ERROR,
            data,
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        self.offset += 1;
        self.source.next().unwrap_or('\0')
    }

    fn match_char(&mut self, c: char) -> bool {
        !self.end() && c.eq(self.source.peek().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.source.peek().unwrap_or(&' ').is_whitespace() {
            self.advance();
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.offset;
        if self.end() {
            return self.make_token(TokenType::EOF);
        }
        let token_type = match self.advance() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            '!' => {
                if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Equal
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            _ => TokenType::EOF,
        };
        self.make_token(token_type)
    }
}
