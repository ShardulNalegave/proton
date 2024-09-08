
#[derive(Error, Debug)]
pub enum LexerError {
  #[error("Unexpected character encountered at line {line}")]
  UnexpectedCharacter { line: usize },
  #[error("Expected a character but found EOF at line {line}")]
  ExpectedCharacter { line: usize },
  #[error("Character literal can only contain one char at line {line}")]
  MultipleCharsInCharLiteral { line: usize },
  #[error("Empty character literal at line {line}")]
  EmptyCharacterLiteral { line: usize },
  #[error("Invalid escape sequence at line {line}")]
  InvalidEscapeSequence { line: usize },
}