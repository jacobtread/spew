use crate::Modifier;
use lazy_static::lazy_static;

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
    right: Option<AST>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ConditionType {
    If,
    IfElse,
    Else,
}

#[derive(Debug)]
struct Variable {
    constant: bool,
    name: String,
    modifiers: Vec<Modifier>,
}

#[derive(Debug)]
struct StructProperty {
    name: String,
    modifiers: Vec<Modifier>,
}

#[derive(Debug)]
struct SpewStruct {
    name: String,
    properties: Vec<StructProperty>,
}

lazy_static! {
    pub static ref NULL_TYPE: DataType = DataType::new("null");

}

#[derive(Debug)]
pub struct DataType {
    pub name: &'static str,
    pub inherit: Vec<DataType>,
}

impl DataType {
    fn new(name: &'static str) -> DataType {
        return Self::new_with(name, Vec::with_capacity(0));
    }

    fn new_with(name: &'static str, inherit: Vec<DataType>) -> DataType {
        return DataType {
            name: name,
            inherit: inherit,
        };
    }
}

#[derive(Debug)]
struct Function {
    name: String,
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
        contents: Box<Vec<AST>>,
    },
    Block(Box<Vec<AST>>),
    Struct(SpewStruct),
}
