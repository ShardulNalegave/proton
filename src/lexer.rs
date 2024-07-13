
pub mod tokens;
use tokens::{Token, TokenKind};

pub struct Lexer {
  pub source: String,
  pub pos: usize,
  pub line: usize,
  pub filename: String,
}

impl Lexer {
  pub fn new(source: String, filename: String) -> Self {
    Self {
      source, filename,
      line: 0,
      pos: 0,
    }
  }

  pub fn peek(&mut self) -> Option<char> {
    self.source.chars().nth(self.pos)
  }

  pub fn advance(&mut self) -> Option<char> {
    let pos = self.pos;
    self.pos += 1;
    self.source.chars().nth(pos)
  }

  pub fn make_token(&self, kind: TokenKind, literal: String) -> Token {
    Token { kind, literal, line: self.line, filename: self.filename.clone() }
  }

  pub fn next_token(&mut self) -> Token {
    match self.advance() {
      None => self.make_token(TokenKind::EOF, "".to_string()),
      Some(c) => match c {
        ' ' | '\t' => self.next_token(),
        '\n' => {
          self.line += 1;
          self.next_token()
        },

        '(' => self.make_token(TokenKind::LeftParen, '('.to_string()),
        ')' => self.make_token(TokenKind::RightParen, ')'.to_string()),
        '{' => self.make_token(TokenKind::LeftBrace, '{'.to_string()),
        '}' => self.make_token(TokenKind::RightBrace, '}'.to_string()),
        '[' => self.make_token(TokenKind::LeftBracket, '['.to_string()),
        ']' => self.make_token(TokenKind::RightBracket, ']'.to_string()),
        ';' => self.make_token(TokenKind::Semicolon, ';'.to_string()),
        '.' => self.make_token(TokenKind::Dot, '.'.to_string()),
        ',' => self.make_token(TokenKind::Comma, ','.to_string()),
        '+' => self.make_token(TokenKind::Plus, '+'.to_string()),
        '-' => self.make_token(TokenKind::Minus, '-'.to_string()),
        '*' => self.make_token(TokenKind::Asterisk, '*'.to_string()),
        '!' => self.make_token(TokenKind::Not, '!'.to_string()),
        '~' => self.make_token(TokenKind::BitwiseNot, '!'.to_string()),

        c => self.make_token(TokenKind::Invalid, c.to_string()),
      },
    }
  }
}