
mod types;

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

    fn step_back(&mut self) {
        self.offset -= 1;
    }

    fn next_char(&mut self) -> Option<char> {
        if self.offset < self.chars.len() {
            let char = *self.chars.get(self.offset)?;
            self.offset += 1;
            if char == '\n' {
                self.line += 1;
                self.line_offset = 0
            } else if char == '\r' {
                self.line_offset = 0;
                return self.next_char();
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
                self.offset -= 1;
                break;
            }
            out.push(char);
        }
        return out;
    }

    fn skip_while<P>(&mut self, mut predicate: P)
    where
        Self: Sized,
        P: FnMut(&char) -> bool,
    {
        while let Some(char) = self.next_char() {
            let result = predicate(&char);
            if !result {
                self.offset -= 1;
                break;
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum KeywordType {
    Constant,
    Let,
    LetMutable,
    Function,
    Unknown,
    Static,
    Modifier(Modifier),
}

impl KeywordType {
    fn from(value: &String) -> Option<KeywordType> {
        return match value.as_ref() {
            "const" => Some(KeywordType::Constant),
            "fun" => Some(KeywordType::Function),
            "let" => Some(KeywordType::Let),
            "mut" => Some(KeywordType::LetMutable),
            v => {
                return if let Some(modifier) = Modifier::from(v) {
                    Some(KeywordType::Modifier(modifier))
                } else {
                    None
                }
            }
        };
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Modifier {
    Public,
    Static,
    Inline,
    Compile,
}

impl Modifier {
    fn from(value: &str) -> Option<Modifier> {
        return match value.as_ref() {
            "pub" => Some(Modifier::Public),
            "static" => Some(Modifier::Static),
            "inline" => Some(Modifier::Inline),
            "compile" => Some(Modifier::Compile),
            _ => None,
        };
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Symbol {
    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    Plus,
    Minus,
    Left,
    Right,
    Underscore,
    Exclamation,
    Equals,
    And,
    Pipe,
    Period,
    Multiply,
    Percent,
    Divide,
    Colon,
    Comma,
}

impl Symbol {
    fn from(value: char) -> Option<Symbol> {
        return match value {
            '{' => Some(Symbol::OpenCurly),
            '}' => Some(Symbol::CloseCurly),
            '(' => Some(Symbol::OpenParen),
            ')' => Some(Symbol::CloseParen),
            '[' => Some(Symbol::OpenSquare),
            ']' => Some(Symbol::CloseSquare),
            '+' => Some(Symbol::Plus),
            '-' => Some(Symbol::Minus),
            '<' => Some(Symbol::Left),
            '>' => Some(Symbol::Right),
            '_' => Some(Symbol::Underscore),
            '!' => Some(Symbol::Exclamation),
            '=' => Some(Symbol::Equals),
            '&' => Some(Symbol::And),
            '|' => Some(Symbol::Pipe),
            '.' => Some(Symbol::Period),
            '*' => Some(Symbol::Multiply),
            '%' => Some(Symbol::Percent),
            '/' => Some(Symbol::Divide),
            ':' => Some(Symbol::Colon),
            ',' => Some(Symbol::Comma),
            _ => None,
        };
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Literal {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
    Undefined,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Token {
    Comment(String),
    Keyword(KeywordType),
    Ident(String),
    Block(Vec<Token>),
    Expression(Box<Token>, Box<Token>),
    Symbol(Symbol),
    Literal(Literal),
}

#[derive(Debug)]
#[allow(dead_code)]
enum ParserError {
    UnexpectedToken(char, String),
    Expected(String),
    Fail(String),
    IncompleteLiteral(String, String),
}

type ParseResult<T> = Result<T, ParserError>;

impl Parser {
    fn consume_comment<'a>(
        context: &'a mut ParserContext<'a>,
    ) -> ParseResult<&'a mut ParserContext<'a>> {
        return if let Some(char) = context.next_char() {
            if char == '/' {
                // Full line comment
                let comment_text = context.take_while(|char| char != &'\n');
                context.push_token(Token::Comment(comment_text));
                Ok(context)
            } else if char == '*' {
                // Multi-line comment
                let mut comment_text = String::new();
                while let Some(first_char) = context.next_char() {
                    if first_char == '*' {
                        if let Some(second_char) = context.next_char() {
                            if second_char == '/' {
                                break;
                            } else {
                                comment_text.push(second_char)
                            }
                        } else {
                            comment_text.push(first_char)
                        }
                    } else {
                        comment_text.push(first_char)
                    }
                }
                println!("{}", comment_text);
                context.push_token(Token::Comment(comment_text));
                Ok(context)
            } else {
                println!("{} {}", context.line, context.line_offset);
                Err(ParserError::UnexpectedToken(
                    char,
                    String::from("Expected '/' for line comment or '*' for multiline comment"),
                ))
            }
        } else {
            Err(ParserError::Expected(String::from("/ for comment at")))
        };
    }

    fn consume_ident<'a>(
        context: &'a mut ParserContext<'a>,
    ) -> ParseResult<&'a mut ParserContext<'a>> {
        context.step_back();
        let ident = context
            .take_while(|char| char.is_alphabetic() || char.is_alphanumeric() || char == &'_');
        if let Some(keyword) = KeywordType::from(&ident) {
            context.push_token(Token::Keyword(keyword));
        } else if ident == "true" || ident == "false" {
            let is_true = ident == "true";
            context.push_token(Token::Literal(Literal::Boolean(is_true)))
        } else if ident == "null" {
            context.push_token(Token::Literal(Literal::Null))
        } else if ident == "ndef" {
            context.push_token(Token::Literal(Literal::Undefined))
        } else {
            context.push_token(Token::Ident(ident));
        }
        Ok(context)
    }

    fn consume_number_literal<'a>(
        context: &'a mut ParserContext<'a>,
    ) -> ParseResult<&'a mut ParserContext<'a>> {
        context.step_back();
        let mut out = String::new();
        let mut decimal = false;
        while let Some(char) = context.next_char() {
            if char.is_numeric() {
                out.push(char);
            } else if char == '.' && !decimal {
                decimal = true;
            } else {
                context.step_back();
                break;
            }
        }
        context.push_token(Token::Literal(Literal::Number(out)));
        Ok(context)
    }

    fn consume_string_literal<'a>(
        context: &'a mut ParserContext<'a>,
    ) -> ParseResult<&'a mut ParserContext<'a>> {
        let mut str = String::new();
        let mut last_char: char = '"';
        while let Some(char) = context.next_char() {
            if char == '"' && last_char != '\\' {
                break;
            } else {
                str.push(char);
                last_char = char;
            }
        }
        context.push_token(Token::Literal(Literal::String(str)));
        Ok(context)
    }

    fn parse(text: &str) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars: Vec<char> = text.chars().collect();

        let mut context = &mut ParserContext::new(&mut chars, &mut tokens);

        while let Some(next_char) = context.next_char() {
            if next_char.is_whitespace() {
                // Consuming whitespace
                context.skip_while(|char| char.is_whitespace());
            } else if next_char == '/' {
                // Consume comments
                context = Parser::consume_comment(context)?;
            } else if next_char.is_alphabetic() {
                // Consume idents
                context = Parser::consume_ident(context)?;
            } else if next_char == '"' {
                context = Parser::consume_string_literal(context)?;
            } else if next_char.is_numeric() {
                context = Parser::consume_number_literal(context)?;
            } else if let Some(symbol) = Symbol::from(next_char) {
                context.push_token(Token::Symbol(symbol))
            }
        }
        return Ok(tokens);
    }
}

const SOURCE: &str = include_str!("../example.spew");

fn main() {
    let tokens = Parser::parse(SOURCE).expect("Failed to parse");
    println!("{:?}", tokens);
}
