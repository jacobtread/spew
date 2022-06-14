use crate::Modifier;

#[derive(Debug)]
#[allow(dead_code)]
enum Operator {
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    Or,
    Xor,
    And,
    AndAnd,
    OrOr,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Operation {
  left: AST,
  operator: Operator,
  right: Option<AST>
}

#[derive(Debug)]
#[allow(dead_code)]
enum ConditionType {
  If,
  IfElse,
  Else, 
}


struct Variable {
  name: String,
  modifiers: Vec<Modifier>,
}

struct TraitProperty {
  name: String,
  modifiers: Vec<Modifier>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum AST {
    Variable {
        name: String,
        modifiers: Vec<Modifier>,
        value: Option<Box<AST>>,
    },
    Operation(Box<Operation>),
    ConditionBlock {
        condition: Box<Vec<Operation>>,
        contents: Box<Vec<AST>>
    },
    Block(Box<Vec<AST>),
}
