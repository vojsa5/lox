
use std::collections::{LinkedList};
use std::str;
use std::str::FromStr;

use crate::token::Token;
use crate::token_type::{*};
use crate::token_type::TokenType::{*};

pub struct Scanner {
    code: String,
    start: usize,
    curr: usize,
    line: i32,
    tokens: LinkedList<Token>,
}


impl Scanner {
    pub fn new(code: String) -> Scanner {
        Scanner {
            code,
            start: 0,
            curr: 0,
            line: 0,
            tokens: LinkedList::new()
        }
    }

    pub fn scan_file(&mut self)-> &LinkedList<Token> {
        while !self.at_end() {
            self.start = self.curr;
            self.scan_token();
        }
        self.create_token(Eof);
        &self.tokens
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0'
        }
        self.code.chars().nth(self.curr).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.curr + 1 >= self.code.len() {
            return '\0'
        }
        self.code.chars().nth(self.curr + 1).unwrap_or('\0')
    }

    fn advance(&mut self){
        self.curr += 1;
    }

    fn at_end(&self) -> bool {
        self.code.len() <= self.curr
    }

    fn create_token(&mut self, tok: TokenType){
        self.tokens.push_back(Token::new(tok, self.code.get(self.start..self.curr).unwrap().to_string(), self.line));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.at_end() {
            !todo!() //TODO error unterminated str
        }

        self.advance();
        let str_val = (&self.code[self.start + 1..self.curr - 1]).to_string();
        self.create_token(StringT { str: str_val })
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance()
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance()
            }
        }
        let num_val = f64::from_str(&self.code[self.start..self.curr]).unwrap();
        self.create_token(Number { num: num_val})
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance()
        }
        let iden_val = self.code.get(self.start..self.curr).expect("");
        let keyword_opt = Scanner::get_keyword(iden_val);
        self.create_token(
            match keyword_opt {
                    Some(keyword) => keyword,
                    None => Identifier {iden: iden_val.to_string()}
            }
        )
    }
    
    fn get_keyword(keyword: &str) -> Option<TokenType> {
        match keyword {
            "and"       =>      Some(And),
            "class"     =>      Some(Class),
            "else"      =>      Some(Else),
            "false"     =>      Some(False),
            "for"       =>      Some(For),
            "fun"       =>      Some(Fun),
            "if"        =>      Some(If),
            "nil"       =>      Some(Nil),
            "or"        =>      Some(Or),
            "print"     =>      Some(Print),
            "return"    =>      Some(Return),
            "super"     =>      Some(Super),
            "this"      =>      Some(This),
            "true"      =>      Some(True),
            "var"       =>      Some(Var),
            "while"     =>      Some(While),
            _           =>      None
        }
    }

    fn scan_token(&mut self) {
        let ch = self.peek();
        self.advance();
            match ch {
                '(' => self.create_token(LeftParen),
                ')' => self.create_token(RightParen),
                '{' => self.create_token(LeftBrace),
                '}' => self.create_token(RightBrace),
                ',' => self.create_token(Comma),
                '.' => self.create_token(Dot),
                '-' => self.create_token(Minus),
                '+' => self.create_token(Plus),
                ';' => self.create_token(Semicolon),
                '*' => self.create_token(Star),
                '!' => self.create_token(if self.peek_next() == '=' { BangEqual } else { Bang }),
                '=' => self.create_token(if self.peek_next() == '=' { EqualEqual } else { Equal }),
                '<' => self.create_token(if self.peek_next() == '=' { LessEqual } else { Less }),
                '>' => self.create_token(if self.peek_next() == '=' { GreaterEqual } else { Greater }),
                '/' =>
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.at_end() {
                            self.advance();
                        }
                    }
                    else {
                        self.create_token(Slash)
                    },
                ' ' => {}
                '\t' => {}
                '\r' => {}
                '\n' => self.line += 1,
                '"' => self.string(),
                _ if ch.is_ascii_digit() => self.number(),
                _ if ch.is_ascii_alphanumeric() || ch == '_' => self.identifier(),
                _ => !todo!() //TODO error Unexpected character
            }

    }
}
