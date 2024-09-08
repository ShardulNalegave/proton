
pub struct Program {
  pub block: Vec<Expression>
}

pub struct FunctionSignature {
  pub name: String,
  pub return_type: Box<Expression>,
}

pub enum Expression {
  Program(Program),

  Let {
    mutable: bool,
    ident: String,
    value: Box<Expression>,
  },

  Return {
    value: Box<Expression>,
  },

  Function {
    signature: FunctionSignature,
    block: Program,
  },
}
