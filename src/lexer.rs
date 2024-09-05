
pub mod kw;
pub mod tokens;
pub mod errors;

// ===== Imports =====
use anyhow::Result;
use kw::Keyword;
use tokens::{Token, TokenKind};
use errors::LexerError;
// ===================

pub struct Lexer {
  source: String,
  chars: Vec<char>,
  filename: String,
  pos: usize,
  line: usize,
}

impl Lexer {
  pub fn new(
    filename: String,
    source: String,
  ) -> Self {
    let chars = source.chars().collect();
    Self {
      source, filename, chars,
      pos: 0,
      line: 1,
    }
  }

  pub fn peek(&self) -> Option<char> {
    self.chars.get(self.pos).copied()
  }

  pub fn advance(&mut self) -> Option<char> {
    let c = self.chars.get(self.pos).copied();
    self.pos += 1;
    c
  }

  fn make_token(
    &self,
    kind: TokenKind,
  ) -> Token {
    Token {
      kind,
      line: self.line,
      filename: self.filename.clone(),
    }
  }

  pub fn next_token(&mut self) -> Result<Token> {
    Ok(match self.advance() {
      None => self.make_token(TokenKind::EOF),
      Some(c) => match c {
        '\n' => {
          self.line += 1;
          return self.next_token();
        },
        ' ' | '\t' => return self.next_token(),

        '(' => self.make_token(TokenKind::LeftParen),
        ')' => self.make_token(TokenKind::RightParen),
        '[' => self.make_token(TokenKind::LeftSquare),
        ']' => self.make_token(TokenKind::RightSquare),
        '{' => self.make_token(TokenKind::LeftBracket),
        '}' => self.make_token(TokenKind::RightBracket),

        '.' => self.make_token(TokenKind::Dot),
        ';' => self.make_token(TokenKind::Semicolon),
        ':' => self.make_token(TokenKind::Colon),
        ',' => self.make_token(TokenKind::Comma),

        '^' => self.make_token(TokenKind::Exponent),
        '~' => self.make_token(TokenKind::BitwiseNot),

        '=' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::Equals)
        } else {
          self.make_token(TokenKind::Assign)
        },

        '+' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::PlusEquals)
        } else {
          self.make_token(TokenKind::Plus)
        },
        '-' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::MinusEquals)
        } else {
          self.make_token(TokenKind::Minus)
        },
        '*' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::MulEquals)
        } else {
          self.make_token(TokenKind::Mul)
        },
        '/' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::DivEquals)
        } else {
          self.make_token(TokenKind::Div)
        },

        '&' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::BitwiseAndEquals)
        } else if self.peek() == Some('&') {
          self.advance();
          self.make_token(TokenKind::And)
        } else {
          self.make_token(TokenKind::BitwiseAnd)
        },
        '|' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::BitwiseOrEquals)
        } else if self.peek() == Some('|') {
          self.advance();
          self.make_token(TokenKind::Or)
        } else {
          self.make_token(TokenKind::BitwiseOr)
        },
        '!' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::NotEquals)
        } else {
          self.make_token(TokenKind::Not)
        },

        '<' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::LessThanEqualTo)
        } else {
          self.make_token(TokenKind::LessThan)
        },
        '>' => if self.peek() == Some('=') {
          self.advance();
          self.make_token(TokenKind::GreaterThanEqualTo)
        } else {
          self.make_token(TokenKind::GreaterThan)
        },

        '\'' => {
          let c = self.read_char()?;
          self.make_token(TokenKind::Char(c))
        },
        '\"' => {
          let s = self.read_string()?;
          self.make_token(TokenKind::String(s))
        },

        c => if c.is_alphabetic() {
          let ident = self.read_identifier();
          match Keyword::from_str(&ident) {
            Some(kw) => self.make_token(TokenKind::Keyword(kw)),
            None => self.make_token(TokenKind::Identifier(ident))
          }
        } else if c.is_digit(10) {
          let num = self.read_number();
          self.make_token(TokenKind::Number(num))
        } else {
          bail!(LexerError::UnexpectedCharacter { line: self.line });
        },
      }, 
    })
  }

  fn read_identifier(&mut self) -> String {
    let start = self.pos - 1;
    while let Some(c) = self.peek() {
      if c.is_alphabetic() || c.is_digit(10) || c == '_' {
        self.advance();
      } else {
        break;
      }
    }

    self.source[start..self.pos].to_string()
  }

  fn read_number(&mut self) -> String {
    let start = self.pos - 1;
    let mut has_decimal = false;
    while let Some(c) = self.peek() {
      if c.is_digit(10) || c == '_' {
        self.advance();
      } else if c == '.' {
        if !has_decimal {
          self.advance();
          has_decimal = true;
        } else {
          break;
        }
      } else {
        break;
      }
    }

    self.source[start..self.pos].to_string()
  }

  fn read_escape_sequence(&mut self) -> Result<char> {
    match self.advance() {
      None => bail!(LexerError::ExpectedCharacter { line: self.line }),
      Some(c) => Ok(match c {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        '\'' => '\'',
        '\"' => '\"',
        '0' => '\0',
        _ => bail!(LexerError::InvalidEscapeSequence { line: self.line }),
      }),
    }
  }

  fn read_char(&mut self) -> Result<char> {
    match self.advance() {
      None => bail!(LexerError::ExpectedCharacter { line: self.line }),
      Some('\'') => bail!(LexerError::EmptyCharacterLiteral { line: self.line }),
      Some('\\') => {
        let c = self.read_escape_sequence()?;
        if self.peek() == Some('\'') {
          return Ok(c);
        } else {
          bail!(LexerError::MultipleCharsInCharLiteral { line: self.line });
        }
      },
      Some(c) => {
        if self.peek() == Some('\'') {
          return Ok(c);
        } else {
          bail!(LexerError::MultipleCharsInCharLiteral { line: self.line });
        }
      },
    }
  }

  fn read_string(&mut self) -> Result<String> {
    let mut s = String::new();
    while let Some(c) = self.advance() {
      match c {
        '\\' => {
          let esc = self.read_escape_sequence()?;
          s.push(esc);
        },
        '\"' => break,
        _ => s.push(c),
      }
    }

    Ok(s)
  }
}