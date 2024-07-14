
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
        ':' => self.make_token(TokenKind::Colon, ';'.to_string()),
        '.' => self.make_token(TokenKind::Dot, '.'.to_string()),
        ',' => self.make_token(TokenKind::Comma, ','.to_string()),
        '+' => self.make_token(TokenKind::Plus, '+'.to_string()),
        '-' => self.make_token(TokenKind::Minus, '-'.to_string()),
        '*' => self.make_token(TokenKind::Asterisk, '*'.to_string()),
        '!' => self.make_token(TokenKind::Not, '!'.to_string()),
        '~' => self.make_token(TokenKind::BitwiseNot, '!'.to_string()),

        '/' => match self.peek() {
          Some('/') => {
            self.advance();
            self.make_token(TokenKind::FloorDivide, "//".to_string())
          },
          _ => self.make_token(TokenKind::FrontSlash, '/'.to_string()),
        },

        '=' => match self.peek() {
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::Equals, "==".to_string())
          },
          Some('>') => {
            self.advance();
            self.make_token(TokenKind::FatArrow, "=>".to_string())
          },
          _ => self.make_token(TokenKind::Assign, '='.to_string()),
        },

        '&' => match self.peek() {
          Some('&') => {
            self.advance();
            self.make_token(TokenKind::And, "&&".to_string())
          },
          _ => self.make_token(TokenKind::BitwiseAnd, '&'.to_string()),
        },

        '|' => match self.peek() {
          Some('|') => {
            self.advance();
            self.make_token(TokenKind::Or, "||".to_string())
          },
          _ => self.make_token(TokenKind::BitwiseOr, '|'.to_string()),
        },

        '<' => match self.peek() {
          Some('<') => {
            self.advance();
            self.make_token(TokenKind::BitwiseLeftShift, "<<".to_string())
          },
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::LessThanEqualTo, "<=".to_string())
          },
          _ => self.make_token(TokenKind::LessThan, '<'.to_string()),
        },

        '>' => match self.peek() {
          Some('>') => {
            self.advance();
            self.make_token(TokenKind::BitwiseRightShift, ">>".to_string())
          },
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::GreaterThanEqualTo, ">=".to_string())
          },
          _ => self.make_token(TokenKind::GreaterThan, '>'.to_string()),
        },

        c => if c.is_alphabetic() || c == '_' {
          match self.read_identifier() {
            ident => self.make_token(TokenKind::Identifier, ident)
          }
        } else if c.is_numeric() {
          let num = self.read_number();
          self.make_token(TokenKind::Number, num)
        } else {
          self.make_token(TokenKind::Invalid, c.to_string())
        },
      },
    }
  }

  fn read_identifier(&mut self) -> String {
    let pos = self.pos - 1;
    while let Some(c) = self.peek() {
      if !c.is_alphanumeric() && c != '_' {
        break;
      }
      self.advance();
    }

    self.source[pos..self.pos].to_string()
  }

  fn read_number(&mut self) -> String {
    let pos = self.pos - 1;
    let mut has_decimal = false;

    while let Some(c) = self.peek() {
      if !c.is_numeric() && c != '_' && c != '.' {
        break;
      }

      if c == '.' {
        if has_decimal {
          break;
        } else {
          has_decimal = true;
        }
      }

      self.advance();
    }

    self.source[pos..self.pos].to_string()
  }
}
