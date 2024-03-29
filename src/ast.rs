use lazy_static::lazy_static;

use crate::{KeywordType, Literal, Modifier, Symbol, Token, TokenSet};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Operation {
    left: AST,
    operator: Operator,
    right: Option<AST>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum ConditionType {
    If,
    IfElse,
    Else,
}

#[derive(Debug, Clone)]
struct Variable {
    constant: bool,
    name: String,
    modifiers: Vec<Modifier>,
    type_of: DataType,
}

#[derive(Debug, Clone)]
struct StructProperty {
    name: String,
    type_of: DataType,
}

#[derive(Debug, Clone)]
struct SpewStruct {
    name: String,
    properties: Vec<StructProperty>,
}

#[derive(Debug, Clone)]
pub struct DataType {
    name: String,
    nullable: bool,
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub struct FunctionStub {
    pub name: String,
    pub modifiers: Vec<Modifier>,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Option<DataType>,
}

#[derive(Debug, Clone)]
pub struct SpewFunction {
    pub stub: FunctionStub,
    pub body: Vec<AST>,
}

#[derive(Debug, Clone)]
pub struct SpewImpl {
    pub functions: Vec<SpewFunction>,
}

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
pub struct ASTSource {
    values: Vec<AST>,
}

#[derive(Debug, Clone)]
pub struct ASTState {
    result: Vec<AST>,
    token_set: TokenSet,
}

impl ASTState {
    fn next_token(&mut self) -> Option<Token> {
        return self.token_set.next_token();
    }

    fn back_token(&mut self) {
        self.token_set.back(1)
    }

    fn push_ast(&mut self, value: AST) {
        self.result.push(value);
    }
}

macro_rules! expect_ident {
    ($state:ident) => {
         if let Some(token) = $state.next_token() {
            if let Token::Ident(ident) = token {
                ident
            } else {
                return Err(ASTError::UnexpectedToken(Some(token)));
            }
        } else {
            return Err(ASTError::Incomplete);
        }
    };
}
macro_rules! expect_symbol {
    ($state:ident, $symbol:ident) => {
         if let Some(token) = $state.next_token() {
            if let Token::Symbol(ref symbol) = token {
                match symbol {
                    Symbol::$symbol => {},
                    _ => return Err(ASTError::UnexpectedToken(Some(token.clone())))
                }
            } else {
               return Err(ASTError::UnexpectedToken(Some(token)))
            }
        } else {
            return Err(ASTError::Incomplete)
        }
    };
}
macro_rules! is_symbol_next {
    ($state:ident, $symbol:ident) => {
         if let Some(token) = $state.next_token() {
            if let Token::Symbol(ref symbol) = token {
                match symbol {
                    Symbol::$symbol => true,
                    _ => {
                        $state.back_token();
                        false
                    }
                }
            } else {
                $state.back_token();
                false
            }
        } else {
            $state.back_token();
            false
        }
    };
}

#[derive(Debug, Clone)]
pub enum ASTError {
    UnexpectedToken(Option<Token>),
    Incomplete,
}

type ASTResult<T> = Result<T, ASTError>;

impl ASTSource {
    fn parse_datatype(state: &mut ASTState) -> ASTResult<DataType> {
        let base_name = expect_ident!(state);
        let mut nullable = false;
        if let Some(next_token) = state.next_token() {
            if let Token::Symbol(symbol) = next_token {
                if let Symbol::Question = symbol {
                    nullable = true
                }
            }
        }
        if !nullable {
            state.back_token();
        }

        Ok(DataType {
            name: base_name,
            nullable,
        })
    }

    fn parse_struct(state: &mut ASTState) -> ASTResult<()> {
        let name = expect_ident!(state);
        expect_symbol!(state, OpenCurly);
        let mut closed = false;
        let mut properties: Vec<StructProperty> = Vec::new();
        while let Some(next_token) = state.next_token() {
            match next_token {
                Token::Ident(name) => {
                    expect_symbol!(state, Colon);
                    let type_of = Self::parse_datatype(state)?;
                    properties.push(StructProperty {
                        name,
                        type_of,
                    });
                }
                Token::Symbol(ref symbol) => {
                    if let Symbol::CloseCurly = symbol {
                        closed = true;
                        break;
                    } else {
                        return Err(ASTError::UnexpectedToken(Some(next_token.clone())));
                    }
                }
                token => return Err(ASTError::UnexpectedToken(Some(token)))
            }
        }
        if !closed {
            return Err(ASTError::UnexpectedToken(None));
        }

        state.push_ast(AST::Struct(SpewStruct {
            name,
            properties,
        }));
        Ok(())
    }

    fn parse_function_stub(state: &mut ASTState, modifiers: Vec<Modifier>) -> ASTResult<FunctionStub> {
        let name = expect_ident!(state);
        expect_symbol!(state, OpenParen);
        let mut closed = false;
        let mut arguments: Vec<FunctionArgument> = Vec::new();
        if let Some(next_token) = state.next_token() {
            match next_token {
                Token::Ident(argument_name) => {
                    expect_symbol!(state, Colon);
                    let data_type = Self::parse_datatype(state)?;
                    arguments.push(FunctionArgument {
                        name: argument_name,
                        data_type,
                    })
                }
                Token::Symbol(symbol) => {
                    if let Symbol::CloseParen = symbol {
                        closed = true
                    }
                }
                token => return Err(ASTError::UnexpectedToken(Some(token)))
            }
        }

        let return_type =
            if is_symbol_next!(state, Minus) {
                expect_symbol!(state, Right); // Parsing for the return type arrow
                Some(Self::parse_datatype(state)?)
            } else {
                None
            };


        Ok(FunctionStub {
            name,
            modifiers,
            arguments,
            return_type,
        })
    }

    fn parse_trait(state: &mut ASTState) -> ASTResult<()> {
        let name = expect_ident!(state);
        expect_symbol!(state, OpenCurly);


        Ok(())
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
                            Self::parse_struct(state)?;
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
