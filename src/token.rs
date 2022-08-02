
use crate::token::token_type::TokenType;

pub mod token_type;



#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
}

impl Token {
    pub fn new (token_type: TokenType) -> Token {
        Token {
            token_type
        }
    }
}