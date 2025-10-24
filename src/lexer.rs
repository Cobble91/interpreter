// use std::io::Write as _;
use std::fmt::Write as _;

pub fn lex(input: String) -> Vec<Token> {
    // println!("lexing: {input}");
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line_number = 1; // for displaying line number in errors
    while chars.peek().is_some() {
        // println!();
        if let Some(new_token) = get_token(&mut chars, &mut line_number) {
            // println!("token: {:?}", &new_token);
            tokens.push(new_token);
        }
    }
    tokens
}

fn get_token(chars: &mut std::iter::Peekable<std::str::Chars>, line_number: &mut usize) -> Option<Token> {
    chars.peek()?; // return none if nothing left if no chars left
    let mut word = String::new();
    if chars.peek().unwrap().is_whitespace() { // skip whitespace
        // println!("WS");
        if chars.peek().unwrap() == &'\n' {
            *line_number += 1;
        }
        chars.next();
        return None;
    }

    if !is_good_char(chars.peek().unwrap()) { // check if next token is not a literal/identifier
        // println!("bad char: {}", chars.peek().unwrap());
        return Some(Token{
            line: *line_number,
            value: match chars.next().unwrap() {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '[' => TokenType::LeftBrack,
                ']' => TokenType::RightBrack,
                '{' => TokenType::LeftCurly,
                '}' => TokenType::RightCurly,
                ',' => TokenType::Comma,
                '.' => {
                    if chars.peek().is_some_and(|x| x.is_numeric()) {
                        // this is copied from the section handling floats but idc
                        word.push('.');
                        while chars.peek().is_some_and(is_good_char) {
                            word.push(chars.next().unwrap());
                        }
                        let flt: f64 = word.parse().unwrap_or_else(|_|
                            panic!("({line_number}) invalid float literal: {word}"));
                        TokenType::FloatLit(flt)
                    }
                    else {
                        TokenType::Period
                    }
                },
                ':' => TokenType::Colon,
                ';' => TokenType::Semicolon,
                '\'' => TokenType::SQuote,
                '\"' => lex_string(chars, line_number), // string literal
                '\\' => TokenType::Backslash,
                '+' => TokenType::Plus,
                '-' => if chars.peek().is_some_and(|x| x == &'>') { chars.next(); TokenType::Arrow }
                else { TokenType::Minus },
                '*' => TokenType::Star,
                '/' => TokenType::Slash, // check for comments, skip until newline
                '%' => TokenType::Mod,
                '~' => TokenType::BwNot,
                '|' => if chars.peek().is_some_and(|x| x == &'|') { chars.next(); TokenType::Or }
                else { TokenType::BwOr }
                '&' => if chars.peek().is_some_and(|x| x == &'&') { chars.next(); TokenType::And }
                else { TokenType::BwAnd }
                '^' => if chars.peek().is_some_and(|x| x == &'^') { chars.next(); TokenType::Xor }
                else { TokenType::BwXor }
                '!' => if chars.peek().is_some_and(|x| x == &'=') { chars.next(); TokenType::Neq }
                else { TokenType::Not }
                '=' => if chars.peek().is_some_and(|x| x == &'=') { chars.next(); TokenType::Eq }
                else { TokenType::Assign }
                '<' => if chars.peek().is_some_and(|x| x == &'=') { chars.next(); TokenType::Lte }
                else { TokenType::Lt }
                '>' => if chars.peek().is_some_and(|x| x == &'=') { chars.next(); TokenType::Gte }
                else { TokenType::Gt }
                other => {
                    panic!("unrecognized character: {other}")
                }
            }
        });
    }

    // if next token is a literal/identifier/keyword, read the whole next word
    while chars.peek().is_some_and(is_good_char) {
        word.push(chars.next().unwrap());
    }

    // println!("word: {word}");

    // check for keywords
    let new_token_type: Option<TokenType> = match word.as_str() {
        "const" => Some(TokenType::Const),
        "var" => Some(TokenType::Var),
        "true" => Some(TokenType::BoolLit(true)),
        "false" => Some(TokenType::BoolLit(false)),
        "int" => Some(TokenType::Int),
        "float" => Some(TokenType::Float),
        "bool" => Some(TokenType::Bool),
        "string" => Some(TokenType::String),
        "enum" => Some(TokenType::Enum),
        "struct" => Some(TokenType::Struct),
        "fn" => Some(TokenType::Function),
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "while" => Some(TokenType::While),
        "return" => Some(TokenType::Return),
        _ => None,
    };
    if new_token_type.is_some() {
        return Some(Token{
            line: *line_number,
            value: new_token_type.unwrap(),
        });
    }

    // check for ints/floats
    // dont immediately return number vals if word is a number so that the
    let mut is_float = false; // invalid literal message can display the full literal
    if word.chars().next().unwrap().is_numeric() {
        if chars.peek().is_some_and(|x| x == &'.') {
            is_float = true;
            chars.next();
            word.push('.');
            while chars.peek().is_some_and(is_good_char) {
                word.push(chars.next().unwrap());
            }
        }
        if is_float { // float found
            let flt: f64 = word.parse().unwrap_or_else(|_|
                panic!("({line_number}) invalid float literal: {word}"));
            return Some(Token{
                line: *line_number,
                value: TokenType::FloatLit(flt),
            });
        }
        else { // int found
            let int: i64 = word.parse().unwrap_or_else(|_|
                panic!("({line_number}) invalid integer literal: {word}"));
            return Some(Token{
                line: *line_number,
                value: TokenType::IntLit(int)
            });
        }
    }

    // must be identifier
    Some(Token{
        line: *line_number,
        value: TokenType::Identifier(word)
    })
}

/// returns true if c is a valid character in an idenfier or a literal
/// identifier cant start with '-' because confuse for subtract
/// float lits cant start with '.' because hard
fn is_good_char(c: &char) -> bool {
    let other_chars = ['_'];
    c.is_alphanumeric() || other_chars.contains(c)
}

fn lex_string(chars: &mut std::iter::Peekable<std::str::Chars>, line_number: &mut usize) -> TokenType {
    let mut output_string = String::new();
    let mut invalid = false;
    loop {
        let next = chars.next();
        match next {
            Some(char) => {
                match char {
                    '\"' => { break } // end of string
                    '\\' => { // esc char found
                        let next = chars.next();
                        match next {
                            Some(esc_char) => {
                                match esc_char {
                                    't' => { output_string.push('\t') }
                                    'n' => { output_string.push('\n') }
                                    '\\' => { output_string.push('\\') }
                                    '"' => { output_string.push('\"') }
                                    _ => {
                                        panic!("({line_number}) invalid escape sequence: '\\{esc_char}'")
                                    }
                                }
                            },
                            None => { // file ends before esc seq does
                                invalid = true;
                                break;
                            }
                        }
                    }
                    char => {
                        if char == '\n' { *line_number += 1; }
                        output_string.push(char)
                    }, // unremarkable char found
                }
            },
            None => { // file ends before string does
                invalid = true;
                break;
            },
        }
    }
    if invalid {
        panic!("({line_number}) invalid string literal: \"{output_string}");
    }
    TokenType::StringLit(output_string)
}

// symbols that dont need to be separated by spaces to be counted as separate tokens
// const SEPARATORS: [char; 28] = [' ', '\t','\n','(', ')', '[', ']', '{', '}',
//                                 ',', '.', ';', '\'','\"','\\',
//                                 '+', '-', '*', '/', '%',
//                                 '~', '|', '&', '^', '!', '=', '<', '>'];

#[derive(Clone)]
pub struct Token {
    pub line: usize,
    pub value: TokenType,
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(&self.value) == std::mem::discriminant(&other.value)
    }
}
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
#[derive(Debug, Clone)]
pub enum TokenType {
    // delimiter
    LeftParen, RightParen,
    LeftBrack, RightBrack,
    LeftCurly, RightCurly,
    Comma, Period, Colon, Semicolon,
    SQuote, DQuote, Backslash, Arrow, // '->'
    Assign, // single '='
    NewLine, // only used to tell the parser which line its currently on
    // operator
    Plus, Minus, Star, Slash, Mod,
    BwNot, BwOr, BwAnd, BwXor,
    Not, Or, And, Xor,
    Eq, Lt, Gt, Neq, Lte, Gte, // Eq: '=='
    // keyword
    Const, Var,
    Int, Float, Bool, String, Void, // Array,
    Enum, Struct, Function,
    If, Else, While,
    // to be added
    // switch, cond, import, break, continue, for, forever
    // more tentative
    // print, input, defer
    Return,
    // literal
    IntLit(i64), FloatLit(f64), BoolLit(bool), StringLit(String), // no arraylit token bc hard
    // identifier (incl variable, struct, fn, etc.)
    Identifier(String),
}
impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrack => "[",
            TokenType::RightBrack => "]",
            TokenType::LeftCurly => "{",
            TokenType::RightCurly => "}",
            TokenType::Comma => ",",
            TokenType::Period => ".",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::SQuote => "'",
            TokenType::DQuote => "\"",
            TokenType::Backslash => "\\",
            TokenType::Arrow => "->",
            TokenType::Assign => "=",
            TokenType::NewLine => "\n",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::Mod => "%",
            TokenType::BwNot => "~",
            TokenType::BwOr => "|",
            TokenType::BwAnd => "&",
            TokenType::BwXor => "^",
            TokenType::Not => "!",
            TokenType::Or => "||",
            TokenType::And => "&&",
            TokenType::Xor => "^^",
            TokenType::Eq => "==",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Neq => "!=",
            TokenType::Lte => "<=",
            TokenType::Gte => ">=",
            TokenType::Const => "const",
            TokenType::Var => "var",
            TokenType::Int => "int",
            TokenType::Float => "float",
            TokenType::Bool => "bool",
            TokenType::String => "string",
            TokenType::Void => "void",
            TokenType::Enum => "enum",
            TokenType::Struct => "struct",
            TokenType::Function => "fn",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::While => "while",
            TokenType::Return => "return",
            TokenType::IntLit(val) => &val.to_string(),
            TokenType::FloatLit(val) => &val.to_string(),
            TokenType::BoolLit(val) => &val.to_string(),
            TokenType::StringLit(val) => &val.to_string(),
            TokenType::Identifier(val) => &val.to_string(),
        };
        write!(f, "{string}")
    }
}
