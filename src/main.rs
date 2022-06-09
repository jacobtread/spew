#![feature(iter_advance_by)]

use std::iter::TakeWhile;
use std::str::Chars;

use crate::Token::Ident;

struct Parser;

struct ParserContext<'a> {
    chars: &'a mut Vec<char>,
    tokens: &'a mut Vec<Token>,

    line: usize,
    line_offset: usize,
    offset: usize,
}

impl ParserContext<'_> {
    fn new<'a>(chars: &'a mut Vec<char>, tokens: &'a mut Vec<Token>) -> ParserContext<'a> {
        ParserContext {
            chars,
            tokens,
            line: 0,
            offset: 0,
            line_offset: 0,
        }
    }

    fn push_token(&mut self, token: Token) {
        self.tokens.push(token)
    }

    fn step_back(&mut self) -> ParseResult<()> {
        self.offset -= 1;
        if self.offset < 0 {
            Err(ParserError::Fail(String::from("Tried to step back too far")))
        } else {
            Ok(())
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.offset < self.chars.len() {
            let char = *self.chars.get(self.offset)?;
            self.offset += 1;
            if char == '\n' {
                self.line += 0;
                self.line_offset = 0
            } else if char == '\r' {
                return self.next_char()
            } else {
                self.line_offset += 1;
            }
            return Some(char);
        }
        return None;
    }

    fn take_while<P>(&mut self, mut predicate: P) -> String
        where
            Self: Sized,
            P: FnMut(&char) -> bool,
    {
        let mut out = String::new();
        while let Some(char) = self.next_char() {
            let result = predicate(&char);
            if !result {
                break;
            }
            out.push(char);
        }
        return out;
    }
}

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
    Fail(String),
}

type ParseResult<T> = Result<T, ParserError>;

impl Parser {
    fn consume_comment<'a>(context: &'a mut ParserContext<'a>) -> ParseResult<&'a mut ParserContext<'a>> {
        return if let Some(char) = context.next_char() {
            if char == '/' { // Full line comment
                let comment_text = context
                    .take_while(|char| char != &'\n');
                context.push_token(Token::Comment(comment_text));
                Ok(context)
            } else if char == '*' { // Multi-line comment
                todo!("Parse comment");
                Ok(context)
            } else {
                Err(ParserError::UnexpectedToken(char, String::from("Expected '/' for line comment or '*' for multiline comment")))
            }
        } else {
            Err(ParserError::Expected(String::from("/ for comment at")))
        };
    }

    fn consume_ident<'a>(context: &'a mut ParserContext<'a>) -> ParseResult<&'a mut ParserContext<'a>> {
        context.step_back()?;
        let ident = context
            .take_while(|char| char.is_alphabetic() || char.is_alphanumeric());
        context.push_token(Ident(ident));
        Ok(context)
    }

    fn parse(text: &str) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars: Vec<char> = text.chars().collect();

        let mut context = &mut ParserContext::new(&mut chars, &mut tokens);

        'parse_loop: while let Some(next_char) = context.next_char() {
            if next_char.is_whitespace() { // Consuming whitespace
                context.push_token(Token::Whitespace);  // Push whitespace token
                let _ = context.take_while(|char| char.is_whitespace());
            }

            if next_char == '/' { // Consume comments
                context = Parser::consume_comment(context)?;
            } else if next_char.is_alphabetic() { // Consume idents
                context = Parser::consume_ident(context)?;
            }
        }
        return Ok(tokens);
    }
}

const SOURCE: &str = include_str!("../example.spew");

fn main() {
    let tokens = Parser::parse(SOURCE)
        .expect("Failed to parse");


    println!("{:?}", tokens);
}
