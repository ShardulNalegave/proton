
pub mod lexer;
pub mod parser;

// ===== Imports =====
#[macro_use] extern crate anyhow;
#[macro_use] extern crate thiserror;
// ===================

fn main() {
  let source = include_str!("../test.prt");
  let mut l = lexer::Lexer::new("test.prt".to_string(), source.to_string());
  loop {
    match l.next_token() {
      Ok(tok) => {
        println!("{:?}", tok);
        if tok.kind == lexer::tokens::TokenKind::EOF {
          break;
        }
      },
      Err(e) => eprint!("\n{:?}\n\n", e),
    }
  }
}
