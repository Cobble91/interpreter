use crate::lexer::{Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> Tree {
    let mut iter = tokens.iter().peekable();
    let mut tree = Tree{
        line: 1,
        value: TreeType::File,
        params: Vec::new(),
    };
    let mut line = 1;
    while peek(&mut iter, &mut line).is_some() {
        tree.params.push(get_tree(&mut iter, &mut line));
    }
    tree
}

fn get_tree(iter: &mut std::iter::Peekable<core::slice::Iter<Token>>, line: &mut usize) -> Tree {
    let next_token = next(iter, line);
    if next_token.is_none() {
        panic!("({line}) expected token");
    }
    match next_token.unwrap().value {
        TokenType::LeftParen => todo!(),
        TokenType::RightParen => todo!(),
        TokenType::LeftBrack => todo!(),
        TokenType::RightBrack => todo!(),
        TokenType::LeftCurly => todo!(),
        TokenType::RightCurly => todo!(),
        TokenType::Comma => todo!(),
        TokenType::Period => todo!(),
        TokenType::Semicolon => todo!(),
        TokenType::SQuote => todo!(),
        TokenType::DQuote => todo!(),
        TokenType::Backslash => todo!(),
        TokenType::Arrow => todo!(),
        TokenType::Assign => todo!(),
        TokenType::NewLine => panic!("error in lexing"), // should be impoosible if iter handled correctly
        TokenType::Plus => todo!(),
        TokenType::Minus => todo!(),
        TokenType::Star => todo!(),
        TokenType::Slash => todo!(),
        TokenType::Mod => todo!(),
        TokenType::BwNot => todo!(),
        TokenType::BwOr => todo!(),
        TokenType::BwAnd => todo!(),
        TokenType::BwXor => todo!(),
        TokenType::Not => todo!(),
        TokenType::Or => todo!(),
        TokenType::And => todo!(),
        TokenType::Xor => todo!(),
        TokenType::Eq => todo!(),
        TokenType::Lt => todo!(),
        TokenType::Gt => todo!(),
        TokenType::Neq => todo!(),
        TokenType::Lte => todo!(),
        TokenType::Gte => todo!(),
        TokenType::Const => todo!(),
        TokenType::Var => todo!(),
        TokenType::Int => todo!(),
        TokenType::Float => todo!(),
        TokenType::Bool => todo!(),
        TokenType::String => todo!(),
        TokenType::Void => panic!("({line}) attempted to parse void token"), // will never happen, just here so rust doesnt get mad
        TokenType::Enum => todo!(),
        TokenType::Struct => todo!(),
        TokenType::Function => get_function(iter, line),
        TokenType::If => todo!(),
        TokenType::Else => todo!(),
        TokenType::While => todo!(),
        TokenType::Return => todo!(),
        // V this is not true (e.g. punctuation) V, change to check for that, then make all else panic
        // only literals or identifiers
        _ => Tree::token_to_leaf(next_token.unwrap())
    }
}

fn get_assign(variable_type: TokenType, iter: &mut std::iter::Peekable<core::slice::Iter<Token>>, line: &mut usize) -> Tree {
    // init assign tree
    let mut new_asn = Tree{
        line: *line,
        value: TreeType::Assign,
        params: Vec::with_capacity(4)
    };

    // var or const
    new_asn.params.push(Tree::leaf(variable_type, *line));

    // variable name
    let mut next_token = next(iter, line);
    if next_token.is_none() {
        panic!("({line}) expected variable name in assignment");
    }
    match next_token.unwrap().value {
        TokenType::Identifier(_) => {
            new_asn.params.push(Tree::leaf(next_token.unwrap().value.clone(), *line));
        },
        _ => panic!("({line}) expected variable name in assignment"),
    }

    // check for ':'
    if next(iter, line).is_none_or(|x| x.value != TokenType::Colon) {
        panic!("({line}) expected ':' in assignment");
    }

    // variable type
    next_token = next(iter, line);
    if next_token.is_none() {
        panic!("({line}) expected variable type in assignment");
    }
    match next_token.unwrap().value {
        TokenType::Int | TokenType::Float | TokenType::Bool | TokenType::String |
        TokenType::Identifier(_) => {
            new_asn.params.push(Tree::leaf(next_token.unwrap().value.clone(), *line));
        },
        _ => panic!("({line}) expected variable type in assignment")
    }

    // check for ':'
    next_token = next(iter, line);
    if next_token.is_none() {
        panic!("({line}) expected '=' or ';' in assignment");
    }
    match next_token.unwrap().value {
        TokenType::Semicolon => {}, // do nothing, this is just a declare
        TokenType::Assign => {
            new_asn.params.push(get_expression(iter, line));
        },
        _ => panic!("({line}) expected '=' or ';' in assignment")
    }

    new_asn
}

fn get_expression(iter: &mut std::iter::Peekable<core::slice::Iter<Token>>, line: &mut usize) -> Tree {
    let mut new_exp = Tree{
        line: *line,
        value: TreeType::Expression,
        params: Vec::new()
    };

    new_exp
}

fn get_function(iter: &mut std::iter::Peekable<core::slice::Iter<Token>>, line: &mut usize) -> Tree {
    // init fn tree
    let mut new_fn = Tree{
        line: *line,
        value: TreeType::Function,
        params: Vec::with_capacity(4)
    };

    // fn name
    let fn_name = next(iter, line).unwrap_or_else(||
        panic!("({line}) expected function name")).value.clone();
    match fn_name {
        TokenType::Identifier(_) => new_fn.params.push(Tree::leaf(fn_name, *line)),
        token => panic!("({line}) {token} is not a valid function name"),
    }

    // check for '('
    if next(iter, line).is_none_or(|x| x.value != TokenType::LeftParen) {
        panic!("({line}) expected '(' in function declaration");
    }

    // params
    let mut mult_params = false; // whether there are multiple params, needed for checking commas
    let mut fn_params = Tree{
        line: *line,
        value: TreeType::Parameters,
        params: Vec::new(),
    };
    let mut next_token;
    loop {
        next_token = next(iter, line);
        // check for ')', finish params if found
        if next_token.is_some_and(|x| x.value == TokenType::RightParen) { break } // )
        // check for ',' whenever there are multiple parameters
        if mult_params {
            if next_token.is_none_or(|x| x.value != TokenType::Comma) {
                panic!("({line}) expected ',' in function declaration");
            }
            next_token = next(iter, line);
        }

        // ok, make param now
        let mut new_param = Tree{
            line: *line,
            value: TreeType::Parameter,
            params: Vec::new(),
        };

        // param name
        if next_token.is_none() {
            panic!("({line}) expected parameter name in function declaration");
        }
        match next_token.unwrap().value {
            TokenType::Identifier(_) => {
                new_param.params.push(Tree::leaf(next_token.unwrap().value.clone(), *line));
            },
            _ => panic!("({line}) expected parameter name in function declaration"),
        }

        // check for ':'
        if next(iter, line).is_none_or(|x| x.value != TokenType::Colon) {
            panic!("({line}) expected ':' in function declaration");
        }

        // param type
        next_token = next(iter, line);
        if next_token.is_none() {
            panic!("({line}) expected parameter type or ')' in function declaration");
        }
        match next_token.unwrap().value {
            TokenType::Int | TokenType::Float | TokenType::Bool | TokenType::String |
            TokenType::Identifier(_) => {
                new_param.params.push(Tree::leaf(next_token.unwrap().value.clone(), *line));
            },
            _ => panic!("({line}) expected parameter type or ')' in function declaration")
        }

        // add new param to params list
        fn_params.params.push(new_param);
        mult_params = true;
    }
    new_fn.params.push(fn_params);

    // return type
    next_token = next(iter, line);
    if next_token.is_none() {
        panic!("({line}) expected function body or '->'");
    }
    let mut ret_type_value = TokenType::Void;
    match next_token.unwrap().value {
        TokenType::Arrow => {
            next_token = next(iter, line);
            if next_token.is_none() {
                panic!("({line}) expected return type");
            }
            match next_token.unwrap().value {
                TokenType::Int | TokenType::Float | TokenType::Bool | TokenType::String |
                TokenType::Identifier(_) => {
                    ret_type_value = next_token.unwrap().value.clone();
                },
                _ => panic!("({line}) expected return type")
            }
            next_token = next(iter, line);
            if next_token.is_none_or(|x| x.value != TokenType::LeftCurly) {
                panic!("({line}) expected function body");
            }
        },
        TokenType::LeftCurly => {
            // do not panic
        },
        _ => panic!("({line}) expected function body or '->'")
    }
    let ret_type = Tree::leaf(ret_type_value, *line);
    new_fn.params.push(ret_type);

    // body
    new_fn.params.push(get_body(iter, line));
    new_fn
}

/// grabs lines of code until an unmatched '}' is found
fn get_body(iter: &mut std::iter::Peekable<core::slice::Iter<Token>>, line: &mut usize) -> Tree {
    let mut body = Tree{
        line: *line,
        value: TreeType::Body,
        params: Vec::new(),
    };
    loop {
        let next_token = peek(iter, line);
        if next_token.is_none() {
            panic!("({line}) expected '}}'");
        }
        match next_token.unwrap().value {
            TokenType::RightCurly => break,
            _ => body.params.push(get_tree(iter, line))
        }
    }
    next(iter, line); // remove '}' from the iter
    body
}

/// returns the next token from iter while also updating line number as necessary
fn next<'a>(iter: &'a mut std::iter::Peekable<core::slice::Iter<Token>>, line: &mut usize) -> Option<&'a Token> {
    match iter.next() {
        Some(next_token) => {
            *line = next_token.line;
            // println!("token: {next_token:?}");
            Some(next_token)
        },
        None => None
    }
}

/// returns the next token from iter while peeking while also updating line number as necessary
fn peek<'a>(iter: &'a mut std::iter::Peekable<core::slice::Iter<Token>>, line: &mut usize) -> Option<&'a Token> {
    match iter.peek() {
        Some(next_token) => {
            *line = next_token.line;
            Some(next_token)
        },
        None => None
    }
}

pub struct Tree {
    line: usize, // the line on which this tree is found
    value: TreeType,
    params: Vec<Tree>,
}
impl Tree {
    fn leaf(token_type: TokenType, line: usize) -> Self {
        Tree{
            line,
            value: TreeType::Leaf(token_type),
            params: Vec::new()
        }
    }
    fn token_to_leaf(token: &Token) -> Self {
        Tree{
            line: token.line,
            value: TreeType::Leaf(token.value.clone()),
            params: Vec::new()
        }
    }
}
impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_helper(0))
    }
}
impl Tree {
    fn fmt_helper(&self, indent: usize) -> String {
        let mut output = String::new();
        for _ in 0..indent {
            output.push_str("  ");
        }
        match &self.value {
            TreeType::Leaf(value) => {
                match value {
                    TokenType::IntLit(int) => output.push_str(&format!("'{int}'")),
                    TokenType::FloatLit(float) => output.push_str(&format!("'{float}'")),
                    TokenType::BoolLit(bool) => output.push_str(&format!("'{bool}'")),
                    TokenType::StringLit(string) => output.push_str(&format!("'{string}'")),
                    TokenType::Identifier(id) => output.push_str(&format!("'{id}'")),
                    _ => output.push_str(&format!("'{value:?}'"))
                };
                output.push_str(&format!(" ({})", &self.line));
                output.push('\n');
            },
            _ => {
                output.push_str(&format!("{:?} ({})", &self.value, &self.line));
                output.push('\n');
                for param in &self.params {
                    output.push_str(&param.fmt_helper(indent+2));
                }
            }
        }
        output
    }
}

#[derive(Debug, Clone)]
pub enum TreeType {
    File, // wraps the entire input file into a single expr
    Body, // contains 0 or more lines of code
    Leaf(TokenType), // any type of leaf, TODO: come up with a better name (terminator? (er?))
    Assign, Expression,
    Enum, Struct, // replace these with generic TypeDeclare?
    Function, Parameters, Parameter, Return,
    If, While,
}
impl PartialEq for TreeType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
