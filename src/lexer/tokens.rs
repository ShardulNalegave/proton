
// ===== Imports =====
use super::kw::Keyword;
// ===================

#[derive(Clone, Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub line: usize,
  pub filename: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
  EOF,

  LeftParen, // (
  RightParen, // )
  LeftSquare, // [
  RightSquare, // ]
  LeftBracket, // {
  RightBracket, // }

  Dot, // .
  Semicolon, // ;
  Colon, // :
  Comma, // ,

  Assign, // =
  Equals, // ==

  Plus, // +
  Minus, // -
  Mul, // *
  Div, // /
  Exponent, // ^

  PlusEquals, // +=
  MinusEquals, // -=
  MulEquals, // *=
  DivEquals, // /=

  And, // &&
  Or, // ||
  Not, // !

  NotEquals, // !=

  BitwiseAnd, // &
  BitwiseOr, // |
  BitwiseNot, // ~

  BitwiseAndEquals, // &=
  BitwiseOrEquals, // |=

  LessThan, // <
  GreaterThan, // >

  LessThanEqualTo, // <=
  GreaterThanEqualTo, // >=

  Identifier(String),
  Char(char),
  String(String),
  Number(String),
  Keyword(Keyword),
}