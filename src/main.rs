use std::process::id;

use crate::Token::Ident;

struct Parser;

#[derive(Debug)]
enum OperationType {
    Add,
    Subtract,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
}

#[derive(Debug)]
enum ExpressionType {
    Assignment,
    OperationAssign(OperationType),
    Operation(OperationType),
}

#[derive(Debug)]
enum Token {
    Whitespace,
    Ident(String),
    Block(Vec<Token>),
    Expression(Box<Token>, Box<Token>, ExpressionType),
}

impl Parser {
    fn parse(text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = text.chars();
        while let Some(mut next_char) = chars.next() {
            if next_char.is_whitespace() { // Encountered whitespace
                while let Some(char) = chars.next() { // Consume all whitespace
                    if !char.is_whitespace() {
                        next_char = char;
                        break;
                    }
                }
                tokens.push(Token::Whitespace);
            }
            if !next_char.is_whitespace() {
                if next_char.is_alphabetic() { // Try parsing ident
                    let mut ident = String::from(next_char);
                    while let Some(char) = chars.next() {
                        if char.is_alphabetic() || char.is_alphanumeric() {
                            ident.push(char);
                        } else {
                            next_char = char;
                            break;
                        }
                    }
                    tokens.push(Ident(ident));
                }
            }
        }
        return tokens;
    }
}

const SRC: &str = "hello world123       test";

fn main() {
    let tokens = Parser::parse(SRC);
    println!("{:?}", tokens);
}
