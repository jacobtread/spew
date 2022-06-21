use std::ptr::eq;
use std::vec;

use lazy_static::lazy_static;

use crate::{Literal, Modifier};

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
    type_of: DataType,
}

#[derive(Debug)]
struct StructProperty {
    name: String,
    modifiers: Vec<Modifier>,
    type_of: DataType,
}

#[derive(Debug)]
struct SpewStruct {
    name: String,
    properties: Vec<StructProperty>,
}

lazy_static! {
    pub static ref VOID_TYPE: DataType = DataType::new("void", Vec::with_capacity(0));
    pub static ref ANY_TYPE: DataType = DataType::new("any", Vec::with_capacity(0));
    pub static ref OBJECT_TYPE: DataType = DataType::new("obj", vec![&ANY_TYPE]);
    pub static ref STRING_TYPE: DataType = DataType::new("str", vec![&OBJECT_TYPE]);
    pub static ref NUMBER_TYPE: DataType = DataType::new("num", vec![&OBJECT_TYPE]);
    pub static ref BOOLEAN_TYPE: DataType = DataType::new("bool", vec![&OBJECT_TYPE]);
    pub static ref NULL_TYPE: DataType = DataType::new("null", vec![&OBJECT_TYPE]);
    pub static ref NOT_DEFINED: DataType = DataType::new("ndef", vec![&OBJECT_TYPE]);
    pub static ref NULLABLE_OBJECT_TYPE: DataType =
        DataType::new("obj?", vec![&ANY_TYPE, &NULL_TYPE]);
    pub static ref NULLABLE_STRING_TYPE: DataType =
        DataType::new("str?", vec![&NULLABLE_OBJECT_TYPE]);
    pub static ref NULLABLE_NUMBER_TYPE: DataType =
        DataType::new("num?", vec![&NULLABLE_OBJECT_TYPE]);
    pub static ref NULLABLE_BOOLEAN_TYPE: DataType =
        DataType::new("bool?", vec![&NULLABLE_OBJECT_TYPE]);
}

#[derive(Debug, Clone)]
pub struct DataType {
    pub name: String,
    pub inherit: Vec<DataType>,
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        return if self.name == other.name {
            true
        } else {
            if self.inherit
                .iter()
                .any(|value| value.name == other.name)
                || other.inherit
                .iter()
                .any(|value| value.name == self.name) {
                return true;
            }
            false
        };
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl DataType {
    fn new(name: &str, inherit: Vec<DataType>) -> DataType {
        return DataType {
            name: String::from(name),
            inherit,
        };
    }
}

#[derive(Debug)]
pub struct FunctionArgument {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug)]
pub struct SpewFunction {
    pub name: String,
    pub modifiers: Vec<Modifier>,
    pub arguments: FunctionArgument,
    pub return_type: DataType,
}

#[derive(Debug)]
pub struct SpewImpl {
    pub functions: Vec<SpewFunction>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum AST {
    Variable {
        name: String,
        modifiers: Vec<Modifier>,
        value: Option<Box<AST>>,
    },
    Literal(Literal),
    Operation(Box<Operation>),
    ConditionBlock {
        condition: Box<Vec<Operation>>,
        contents: Box<Vec<AST>>,
    },
    Block(Box<Vec<AST>>),
    Struct(SpewStruct),
    Impl(SpewImpl),
    Function(SpewFunction),
}
