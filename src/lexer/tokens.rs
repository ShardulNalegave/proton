
#[derive(Clone, Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub line: usize,
  pub filename: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
  EOF,
  Invalid(char),

  Identifier(String),
  Number { literal: String, suffix: Option<String> },
  String(String),
  Character(char),
  Keyword(Keyword),

  LeftParen, // (
  RightParen, // )
  LeftBrace, // {
  RightBrace, // }
  LeftBracket, // [
  RightBracket, // ]

  Semicolon, // ;
  Colon, // :
  Dot, // .
  Comma, // ,
  Plus, // +
  Minus, // -
  Asterisk, // *
  FrontSlash, // /
  Assign, // =
  Equals, // ==
  LessThan, // <
  LessThanEqualTo, // <=
  GreaterThan, // >
  GreaterThanEqualTo, // >=
  FatArrow, // =>

  And, // &&
  Or, // ||
  Not, // !
  BitwiseAnd, // &
  BitwiseOr, // |
  BitwiseNot, // ~
  BitwiseLeftShift, // <<
  BitwiseRightShift, // >>

  AddAssign, // +=
  SubAssign, // -=
  MulAssign, // *=
  DivAssign, // /=
  BitwiseAndAssign, // &=
  BitwiseOrAssign, // |=
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
  Let,
  Const,

  Fn,
  Return,

  If,
  Else,
  Match,

  While,
  For,

  Enum,
  Struct,
  Interface,
  Impl,
  As,

  Bool,
  True,
  False,

  Type,
  Str,
  Char,

  I8,
  I16,
  I32,
  I64,
  U8,
  U16,
  U32,
  U64,
  F32,
  F64,
}

impl Keyword {
  pub fn from_str(val: &str) -> Option<Self> {
    Some(match val {
      "let" => Self::Let,
      "const" => Self::Const,
      "fn" => Self::Fn,
      "return" => Self::Return,
      "if" => Self::If,
      "else" => Self::Else,
      "match" => Self::Match,
      "while" => Self::While,
      "for" => Self::For,
      "enum" => Self::Enum,
      "struct" => Self::Enum,
      "interface" => Self::Interface,
      "impl" => Self::Impl,
      "as" => Self::As,
      "bool" => Self::Bool,
      "true" => Self::True,
      "false" => Self::False,
      "type" => Self::Type,
      "str" => Self::Str,
      "char" => Self::Char,
      "i8" => Self::I8,
      "i16" => Self::I16,
      "i32" => Self::I32,
      "i64" => Self::I64,
      "u8" => Self::U8,
      "u16" => Self::U16,
      "u32" => Self::U32,
      "u64" => Self::U64,
      "f32" => Self::F32,
      "f64" => Self::F64,
      _ => return None,
    })
  }
}