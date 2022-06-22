use lazy_static::lazy_static;

use crate::{KeywordType, Literal, Modifier, Symbol, Token, TokenSet};

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
    // pub static ref VOID_TYPE: DataType = DataType::new("void", Vec::with_capacity(0));
    // pub static ref ANY_TYPE: DataType = DataType::new("any", Vec::with_capacity(0));
    // pub static ref OBJECT_TYPE: DataType = DataType::new("obj", vec![ANY_TYPE]);
    // pub static ref STRING_TYPE: DataType = DataType::new("str", vec![OBJECT_TYPE]);
    // pub static ref NUMBER_TYPE: DataType = DataType::new("num", vec![OBJECT_TYPE]);
    // pub static ref BOOLEAN_TYPE: DataType = DataType::new("bool", vec![OBJECT_TYPE]);
    // pub static ref NULL_TYPE: DataType = DataType::new("null", vec![OBJECT_TYPE]);
    // pub static ref NOT_DEFINED: DataType = DataType::new("ndef", vec![OBJECT_TYPE]);
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
pub enum AST {
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


#[derive(Debug)]
pub struct ASTSource {
    values: Vec<AST>,
}

#[derive(Debug)]
pub struct ASTState {
    result: Vec<AST>,
    token_set: TokenSet,
}

impl ASTState {
    fn next_token(&mut self) -> Option<&Token> {
        return self.token_set.next_token();
    }

    fn push_ast(&mut self, value: AST) {
        self.result.push(value);
    }

    fn expect_symbol(&mut self, symbol: Symbol) -> ASTResult<()> {
        if let Some(token) = self.next_token() {
            if let Token::Symbol(other_symbol) = token {
                match symbol {
                    Symbol::Colon => {}
                    _ => {}
                }
                if &symbol == other_symbol {
                    Ok(())
                } else {
                    Err(ASTError::UnexpectedToken)
                }
            } else {
                Err(ASTError::UnexpectedToken)
            }
        } else {
            Err(ASTError::Incomplete)
        }
    }
}

macro_rules! expect_ident {
    ($state:ident) => {
         if let Some(token) = $state.next_token() {
            if let Token::Ident(ident) = token {
                ident
            } else {
                return Err(ASTError::UnexpectedToken);
            }
        } else {
            return Err(ASTError::Incomplete);
        }
    };
}
macro_rules! expect_symbol {
    ($state:ident, $symbol:ident) => {
         if let Some(token) = $state.next_token() {
            if let Token::Symbol(symbol) = token {
                match symbol {
                    Symbol::$symbol => {},
                    _ => return Err(ASTError::UnexpectedToken)
                }
            } else {
               return Err(ASTError::UnexpectedToken)
            }
        } else {
            return Err(ASTError::Incomplete)
        }
    };
}

#[derive(Debug)]
pub enum ASTError {
    UnexpectedToken,
    Incomplete,
}

type ASTResult<T> = Result<T, ASTError>;

impl ASTSource {
    fn parse_struct(state: &mut ASTState) -> ASTResult<&mut ASTState> {
        let name = expect_ident!(state);
        expect_symbol!(state, OpenCurly);

        let mut content = false;
        let mut properties: Vec<StructProperty> = Vec::new();

        loop {
            let next_token = state.next_token();
            if let Some(next_token) = next_token {
                match next_token {
                    Token::Ident(name) => {
                        expect_symbol!(state, Colon);
                        let type_name = expect_ident!(state);
                        println!("Property {} {}", name, type_name)
                    }
                    Token::Symbol(symbol) => {

                        if let Symbol::CloseCurly = symbol {
                            break;
                        } else {
                            return Err(ASTError::UnexpectedToken);
                        }
                    }
                    _ => return Err(ASTError::UnexpectedToken)
                }
            } else {
                break;
            }
        }

        expect_symbol!(state, CloseCurly);

        println!("Struct named {}", name);

        Ok(state)
    }

    pub fn parse_ast(value: TokenSet) -> ASTResult<Vec<AST>> {
        let mut token_set = value;
        let mut under_state = ASTState {
            result: Vec::new(),
            token_set,
        };
        let mut state = &mut under_state;
        while let Some(token) = state.next_token() {
            match token {
                Token::Comment(_) => {
                    // Comments are ignored
                }
                Token::Keyword(keyword) => {
                    match keyword {
                        KeywordType::Constant => {}
                        KeywordType::Let => {}
                        KeywordType::Function => {}
                        KeywordType::Unknown => {}
                        KeywordType::Static => {}
                        KeywordType::Struct => {
                            state = Self::parse_struct(state)?;
                        }
                        KeywordType::Trait => {}
                        KeywordType::Impl => {}
                        KeywordType::For => {}
                        KeywordType::Modifier(_) => {}
                    }
                }
                Token::Ident(ident) => {}
                Token::Symbol(_) => {}
                Token::Literal(_) => {}
            }
        }
        let result = under_state.result;
        Ok(result)
    }
}
