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
    Comment(String),
    Ident(String),
    Block(Vec<Token>),
    Expression(Box<Token>, Box<Token>, ExpressionType),
}

#[derive(Debug)]
enum ParserError {
    UnexpectedToken(char, String),
    Expected(String),
}

type ParseResult<T> = Result<T, ParserError>;

impl Parser {
    fn parse(text: &str) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = text.chars();
        let column = 0;
        let line = 0;
        'parse_loop: while let Some(mut next_char) = chars.next() {
            // Whitespace consumption
            if next_char.is_whitespace() {
                tokens.push(Token::Whitespace); // Add whitespace token
                while let Some(char) = chars.next() { // Consume all whitespace
                    if !char.is_whitespace() {
                        next_char = char;
                        continue 'parse_loop;
                    }
                }
            }

            // Start Comment consumption

            // Line Comment
            if next_char == '/' {
                if let Some(char) = chars.next() {
                    if char == '/' { // Full line comment
                        let mut comment_text = String::new();
                        while let Some(char) = chars.next() {
                            if char != '\n' {
                                comment_text.push(char)
                            } else {
                                next_char = char;
                                break
                            }
                        }
                        tokens.push(Token::Comment(comment_text))
                    } else if next_char == '*' { // Multi-line comment
                        todo!("Implement multiline comment")
                    } else {
                        return Err(ParserError::UnexpectedToken(next_char, String::from("Expected '/' for line comment or '*' for multiline comment")));
                    }
                } else {
                    return Err(ParserError::Expected(String::from("/ for comment at")));
                }
            }

            // End Comment consumption


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
        return Ok(tokens);
    }
}

const SRC: &str = "hello world123       test";

const SOURCE: &str = concat!(
    "// Test this is a comment",
    "might work like this",
);

fn main() {
    let tokens = Parser::parse(SOURCE)
        .expect("Failed to parse");
    println!("{:?}", tokens);
}
