
pub mod lexer;

// ===== Imports =====
use lexer::tokens::TokenKind;
// ===================

fn main() {
  let source = "hello.world = == => <= ; : 3439 854.324 454.2392.3545";
  let mut l = lexer::Lexer::new(source.to_string(), "test.prt".to_string());
  loop {
    let tok = l.next_token();
    if tok.kind == TokenKind::EOF {
      break;
    }

    println!("{:?}", tok);
  }
}
