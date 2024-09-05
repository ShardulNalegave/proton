
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
  Let,
  Mut,
  If,
  Else,
  For,
  While,
  As,
  Pub,
  Struct,
  Enum,
  Fn,

  Type,
  Void,
  Str,
  Bool,
  True,
  False,
}

impl Keyword {
  pub fn from_str(s: &str) -> Option<Keyword> {
    Some(match s {
      "let" => Keyword::Let,
      "mut" => Keyword::Mut,
      "if" => Keyword::If,
      "else" => Keyword::Else,
      "for" => Keyword::For,
      "while" => Keyword::While,
      "as" => Keyword::As,
      "pub" => Keyword::Pub,
      "struct" => Keyword::Struct,
      "enum" => Keyword::Enum,
      "fn" => Keyword::Fn,
      "type" => Keyword::Type,
      "void" => Keyword::Void,
      "str" => Keyword::Str,
      "bool" => Keyword::Bool,
      "true" => Keyword::True,
      "false" => Keyword::False,
      _ => return None,
    })
  }
}