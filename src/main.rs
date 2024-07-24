
pub mod lexer;

// ===== Imports =====
#[macro_use] extern crate thiserror;
use lexer::tokens::TokenKind;
// ===================

fn main() {
  let source = include_str!("../test.prt");
  let mut l = lexer::Lexer::new(source.to_string(), "test.prt".to_string());
  loop {
    let tok = l.next_token().unwrap();
    println!("{:?}", tok);
    if tok.kind == TokenKind::EOF {
      break;
    }
  }
}
