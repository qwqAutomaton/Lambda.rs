use std::iter::Peekable;

use crate::tokenizer::Token;

#[derive(PartialEq)]
pub enum Term {
    Variable(i32), // negative for free variable
    Lambda(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
}

pub struct Parser<'a> {
    iter: Peekable<std::slice::Iter<'a, Token>>,
    env: Vec<String>,
    freevar: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter().peekable(),
            env: Vec::new(),
            freevar: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> (Term, Vec<String>) {
        (self.parse_term(), self.freevar.clone())
    }

    fn expect_token(&mut self, expected: &Token, msg: &str) {
        if self.iter.next() != Some(expected) {
            panic!("{}", msg);
        }
    }

    fn expect_ident(&mut self) -> String {
        if let Some(Token::Var(name)) = self.iter.next() {
            name.clone()
        } else {
            panic!("Expected identifier");
        }
    }

    fn parse_term(&mut self) -> Term {
        match self.iter.peek() {
            Some(Token::Var(_)) => self.parse_var(),
            Some(Token::Lambda) => self.parse_lambda(),
            Some(Token::Bra) => self.parse_application(),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_var(&mut self) -> Term {
        let ident = self.expect_ident();
        if let Some(idx) = self.env.iter().rposition(|name| name == &ident) {
            let depth = self.env.len() - idx;
            Term::Variable(depth as i32)
        } else {
            self.freevar.push(ident.clone());
            Term::Variable(-(self.freevar.len() as i32))
        }
    }

    fn parse_lambda(&mut self) -> Term {
        self.iter.next();
        let param = self.expect_ident();
        self.expect_token(&Token::Dot, "Expected '.' after variable in lambda");
        self.expect_token(&Token::LBrace, "Expected '{' after '.' in lambda");
        self.env.push(param.clone());
        let body = self.parse_term();
        self.expect_token(&Token::RBrace, "Expected '}' after lambda body");
        self.env.pop();
        Term::Lambda(param, Box::new(body))
    }

    fn parse_application(&mut self) -> Term {
        self.iter.next();
        let lhs = self.parse_term();
        if let Some(Token::Delim) = self.iter.next() {
        } else {
            panic!("Expected delimiter '|' in application");
        };
        let rhs = self.parse_term();
        if let Some(Token::Ket) = self.iter.next() {
        } else {
            panic!("Expected '>' after application");
        };
        Term::Application(Box::new(lhs), Box::new(rhs))
    }
}
