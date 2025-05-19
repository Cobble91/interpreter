use crate::lexer::{
    lex,
    tokens::{
        *,
        Token::*,
        Keyword::*,
        Literal::*,
        Operator::*,
        Separator::*,
        Identifier::*,
    }
};
use crate::parser::nodes::{
    *,
    Type::*,
};

pub fn parse(tokens: Vec<Token>) {
    let iter = tokens.iter();
    for token in iter {
        
    }
}

pub mod nodes {
    pub struct Node {
        node_type: Type,
    }
    pub enum Type {
        Block,
    }
}
