
pub mod tokens;
use tokens::{Keyword, Token, TokenKind};

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

  pub fn make_token(&self, kind: TokenKind) -> Token {
    Token { kind, line: self.line, filename: self.filename.clone() }
  }

  pub fn next_token(&mut self) -> Token {
    match self.advance() {
      None => self.make_token(TokenKind::EOF),
      Some(c) => match c {
        ' ' | '\t' => self.next_token(),
        '\n' => {
          self.line += 1;
          self.next_token()
        },

        '(' => self.make_token(TokenKind::LeftParen),
        ')' => self.make_token(TokenKind::RightParen),
        '{' => self.make_token(TokenKind::LeftBrace),
        '}' => self.make_token(TokenKind::RightBrace),
        '[' => self.make_token(TokenKind::LeftBracket),
        ']' => self.make_token(TokenKind::RightBracket),
        ';' => self.make_token(TokenKind::Semicolon),
        ':' => self.make_token(TokenKind::Colon),
        '.' => self.make_token(TokenKind::Dot),
        ',' => self.make_token(TokenKind::Comma),
        '!' => self.make_token(TokenKind::Not),
        '~' => self.make_token(TokenKind::BitwiseNot),

        '+' => match self.peek() {
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::AddAssign)
          },
          _ => self.make_token(TokenKind::Plus),
        },

        '-' => match self.peek() {
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::SubAssign)
          },
          _ => self.make_token(TokenKind::Minus),
        },

        '*' => match self.peek() {
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::MulAssign)
          },
          _ => self.make_token(TokenKind::Asterisk),
        },

        '/' => match self.peek() {
          Some('/') => {
            self.advance();
            unimplemented!()
          },
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::DivAssign)
          }
          _ => self.make_token(TokenKind::FrontSlash),
        },

        '=' => match self.peek() {
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::Equals)
          },
          Some('>') => {
            self.advance();
            self.make_token(TokenKind::FatArrow)
          },
          _ => self.make_token(TokenKind::Assign),
        },

        '&' => match self.peek() {
          Some('&') => {
            self.advance();
            self.make_token(TokenKind::And)
          },
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::BitwiseAndAssign)
          },
          _ => self.make_token(TokenKind::BitwiseAnd),
        },

        '|' => match self.peek() {
          Some('|') => {
            self.advance();
            self.make_token(TokenKind::Or)
          },
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::BitwiseOrAssign)
          },
          _ => self.make_token(TokenKind::BitwiseOr),
        },

        '<' => match self.peek() {
          Some('<') => {
            self.advance();
            self.make_token(TokenKind::BitwiseLeftShift)
          },
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::LessThanEqualTo)
          },
          _ => self.make_token(TokenKind::LessThan),
        },

        '>' => match self.peek() {
          Some('>') => {
            self.advance();
            self.make_token(TokenKind::BitwiseRightShift)
          },
          Some('=') => {
            self.advance();
            self.make_token(TokenKind::GreaterThanEqualTo)
          },
          _ => self.make_token(TokenKind::GreaterThan),
        },

        '\'' => {
          let c = self.read_char();
          self.make_token(TokenKind::Character(c))
        },

        c => if c.is_alphabetic() || c == '_' {
          let ident = self.read_identifier();
          match Keyword::from_str(&ident) {
            Some(kw) => self.make_token(TokenKind::Keyword(kw)),
            None => self.make_token(TokenKind::Identifier(ident))
          }
        } else if c.is_numeric() {
          let (literal, suffix) = self.read_number();
          self.make_token(TokenKind::Number { literal, suffix })
        } else {
          self.make_token(TokenKind::Invalid(c))
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

    match &self.source[pos..self.pos] {

      ident => ident.to_string(),
    }
  }

  fn read_number(&mut self) -> (String, Option<String>) {
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

    if let Some(c) = self.peek() {
      if c.is_alphabetic() {
        self.advance();
        let suffix = self.read_identifier();
        (self.source[pos..self.pos].to_string(), Some(suffix))
      } else {
        (self.source[pos..self.pos].to_string(), None)
      }
    } else {
      (self.source[pos..self.pos].to_string(), None)
    }
  }

  fn read_char(&mut self) -> char {
    match self.peek() {
      None | Some('\'') => panic!("Expected a char"),
      Some(c) => {
        self.advance();
        match self.peek() {
          None => panic!("char quotes not closed"),
          Some('\'') => {
            self.advance();
            c
          },
          Some(_) => panic!("char cannot contain multiple characters: {}", c),
        }
      },
    }
  }
}
