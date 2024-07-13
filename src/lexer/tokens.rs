
#[derive(Clone, Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub literal: String,
  pub line: usize,
  pub filename: String,
}

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
  EOF,
  Invalid,

  Identifier,
  Number,
  String,
  Character,

  Let,
  Const,
  Fn,
  If,
  Else,
  While,
  For,
  Enum,
  Struct,
  True,
  False,
  Return,

  LeftParen, // (
  RightParen, // )
  LeftBrace, // {
  RightBrace, // }
  LeftBracket, // [
  RightBracket, // ]

  Semicolon, // ;
  Dot, // .
  Comma, // ,
  Plus, // +
  Minus, // -
  Asterisk, // *
  FrontSlash, // /
  FloorDivide, // //
  Assign, // =
  Equals, // ==
  LessThan, // <
  LessThanEqualTo, // <=
  GreaterThan, // >
  GreaterThanEqualTo, // >=

  And, // &&
  Or, // ||
  Not, // !
  BitwiseAnd, // &
  BitwiseOr, // |
  BitwiseNot, // ~
  BitwiseLeftShift, // <<
  BitwiseRightShift, // >>
}