use peeking_take_while::PeekableExt;

use crate::lexer::tokens::{
    *,
    Token::*,
    Identifier::*,
    Keyword::*,
    Separator::*,
    Operator::*,
    Literal::*,
};
// use crate::lexer::tokens::*;

const ESC_CHAR: char = '\\';

pub fn lex(input: String) -> Vec<Token> {
    println!("{input}");
    // let t = Token::Literal(Literal::Integer(73));
    // println!("t = {t:?}");

    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();
    while let Some(c) = iter.peek() {
        // comments
        if *c == '#' {
            // iter.by_ref
            while iter.peek() != Some(&'\n') {
                iter.next();
            }
            print!("(comment) ");
        }
        // whitespace
        else if *c == ' ' || *c == '\t' {
            iter.next();
        }
        // alphabetic
        else if c.is_alphabetic() || *c == '_' {
            let word:String = iter
                .peeking_take_while(|ch| is_alphanumplus(*ch))
                .collect();
            print!("({word}) ");
            let token:Token = match word.as_str() {
                "bool" => Keyword(Bool),
                "int" => Keyword(Int),
                "float" => Keyword(Float),
                "string" => Keyword(StringType),
                "struct" => Keyword(Struct),
                "enum" => Keyword(Enum),

                "if" => Keyword(If),
                "else" => Keyword(Else),
                "switch" => Keyword(Switch),
                "case" => Keyword(Case),
                "default" => Keyword(Default),
                "while" => Keyword(While),
                "for" => Keyword(For),
                "forever" => Keyword(Forever),

                "type" => Keyword(Type),
                "fn" => Keyword(Fn),
                "return" => Keyword(Return),

                _ => Identifier(Unknown{name:word}),
            };
            tokens.push(token);
        }
        // number
        else if c.is_numeric() || *c == '.' {
            let mut found_dot = false;
            let word:String = iter
                .peeking_take_while(|ch| {
                    if *ch == '.' {
                        if !found_dot {
                            found_dot = true;
                            return true;
                        }
                        else {
                            return false;
                        }
                    }
                    ch.is_numeric()
                })
                .collect();
            if found_dot {
                let float:f64 = match word.parse::<f64>() {
                    Ok(value) => value,
                    Err(why) => panic!("Invalid floating point value: {word}: {why}"),
                };
                tokens.push(Literal(FloatLit(float)));
            }
            else {
                let int:i64 = match word.parse::<i64>() {
                    Ok(value) => value,
                    Err(why) => panic!("Invalid integer value: {word}: {why}"),
                };
                tokens.push(Literal(IntLit(int)));
            }
            print!("({word}) ");
        }
        // string
        else if *c == '\"' {
            iter.next();
            let mut escaped = false;
            let word:String = iter
                .peeking_take_while(|ch| {
                    if escaped {
                        escaped = false;
                        true
                    }
                    else if *ch == ESC_CHAR {
                        escaped = true;
                        true
                    }
                    else {
                        *ch != '\"'
                    }
                })
                .collect();
            // print!("({word}) ");
            let final_string = escape(word);
            print!("({final_string}) ");
            tokens.push(Literal(StringLit(final_string)));
            iter.next();
        }
        // EOL
        else if *c == '\n' {
            let _word:String = c.to_string();
            iter.next();
            tokens.push(Separator(EOL));
            println!();
        }
        // punctuation
        else if !c.is_alphanumeric() {
            // let word:String = c.to_string();
            let first = *c;
            iter.next(); // next to allow peeking to check for multi-char punctuatiun tokens

            // if there is another char, get it. otherwise, set second to Y,
            // which will never be part of a punctuation sequence
            let second = *iter.peek().unwrap_or(&'Y');
            let token = match first {
                ',' => Separator(Comma),
                '.' => Separator(Period),
                '-' => {
                    print!("\nasdf: {second}\n");
                    if second == '>' {
                        iter.next();
                        Separator(Arrow)
                    }
                    else {
                        Operator(Minus)
                    }
                },
                '{' => Separator(LeftCurly),
                '}' => Separator(RightCurly),
                '[' => Separator(LeftBrack),
                ']' => Separator(RightBrack),
                '(' => Separator(LeftParen),
                ')' => Separator(RightParen),
                '\n' => Separator(EOL),
                '+' => Operator(Plus),
                // minus already covered
                '*' => Operator(Star),
                '/' => Operator(Slash),
                '%' => Operator(Mod),
                '!' => {
                    if second == '=' {
                        iter.next();
                        Operator(NEQ)
                    }
                    else {
                        Operator(Not)
                    }
                },
                '|' => {
                    if second == '|' {
                        iter.next();
                        Operator(Or)
                    }
                    else {
                        Operator(Bwor)
                    }
                },
                '&' => {
                    if second == '&' {
                        iter.next();
                        Operator(And)
                    }
                    else {
                        Operator(Bwand)
                    }
                },
                '^' => {
                    if second == '^' {
                        iter.next();
                        Operator(Xor)
                    }
                    else {
                        Operator(Bwxor)
                    }
                },
                '=' => {
                    if second == '=' {
                        iter.next();
                        Operator(EQ)
                    }
                    else {
                        Keyword(Assign)
                    }
                },
                '<' => {
                    if second == '=' {
                        iter.next();
                        Operator(LTE)
                    }
                    else {
                        Operator(LT)
                    }
                },
                '>' => {
                    if second == '=' {
                        iter.next();
                        Operator(GTE)
                    }
                    else {
                        Operator(GT)
                    }
                },
                ch => panic!("Invalid token: {ch}"),
            };
            tokens.push(token);
            print!("({first}) ");
        }
        // should never happen, but want to avoid infinite loop
        else {
            iter.next();
        }
    }
    tokens.push(Separator(EOF));
    println!();
    // println!("{tokens:?}");
    for t in tokens.clone() {
        match t {
            Separator(EOL) => print!("\n\n"),
            _ => print!("{t:?} "),
        }
    }
    println!();

    tokens
}

/// Returns whether or not the character is valid for use in an identifier name
fn is_alphanumplus(c: char) -> bool {
    if c.is_alphanumeric() {return true};
    matches!(c, '_' | '-')
}

/// Replaces all instances of escape characters with the proper character
fn escape(s:String) -> String {
    let mut output = String::with_capacity(s.len());
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        match c {
            '\n' | '\t' => continue,
            ESC_CHAR => {
                if let Some(ch) = iter.next() {
                    let esc_char = match ch {
                        'n' => '\n',
                        't' => '\t',
                        '\\' => '\\',
                        '\"' => '\"',
                        _ => panic!("Invalid escape character: \\{ch}"),
                    };
                    output = [output, esc_char.to_string()].join("");
                }
                else {
                    panic!("No escape character found");
                }
            },
            _ => output = [output, c.to_string()].join(""),
        };
    }
    output
}

// let x = Literal::String("hello".to_string());
// tokens.push(Test);
// let s = &buffer[0..=5];

pub mod tokens { // maybe move this to its own file to be used by parser in AST as well
    #[derive(Debug, Clone)]
    pub enum Token {
        Identifier(Identifier),
        Keyword(Keyword),
        Separator(Separator),
        Operator(Operator),
        Literal(Literal),
    }

    #[derive(Debug, Clone)]
    pub enum Identifier {
        Unknown{name:String}, // type of identifier cannot be determined by lexer, leave to parser
    }                         // keep unknown type to preserve same structure as other tokens
    #[derive(Debug, Clone, Copy)]
    pub enum Keyword {
        // primitives
        Bool, Int, Float,
        StringType,
        Struct, Enum,
        // control flow
        If, Else, Switch, Case, Default,
        While, For, Forever,
        // other
        Assign,
        Type,
        Fn, Return, 
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Separator {
        // delimiters
        // single
        Period, Comma, Arrow/* -> */,
        // paired
        LeftCurly, RightCurly, LeftBrack, RightBrack, LeftParen, RightParen,
        // other
        EOL, EOF,
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Operator {
        // int ops
        Plus, Minus, Star, Slash, Mod,
        Bwnot, Bwor, Bwand, Bwxor,
        // bool ops
        Not, Or, And, Xor,
        // equality
        EQ, NEQ, GT, GTE, LT, LTE,
    }
    #[derive(Debug, Clone)]
    pub enum Literal {
        Null,
        IntLit(i64),
        FloatLit(f64),
        StringLit(String),
    }
}
