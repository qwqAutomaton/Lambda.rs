/*
Syntax:
TERM = VAR | LAMBDA | APPLICATION
VAR = [a-zA-Z_][a-zA-Z0-9_]* -- normal identifier rules
LAMBDA = '\' VAR '.' '{' TERM '}' -- \x.{x+1} for example
APPLICATION = '<' TERM '|' TERM '>' -- something like Dirac, <\x.{x+1}|y>
*/

use core::panic;
use std::{iter::Peekable};
#[derive(PartialEq)]
enum Term {
    Variable(Option<usize>),   // store de Bruijn internally; None = free var
    Lambda(String, Box<Term>), // param for pretty-printer (debug)
    Application(Box<Term>, Box<Term>),
}

// pretty printer wrapper. print with named vars (not indices)
fn pretty_print(term: &Term) -> String {
    fn print_term(term: &Term, env: &mut Vec<String>) -> String {
        match term {
            Term::Variable(idx) => print_var(idx, env),
            Term::Lambda(lmd, body) => print_lambda(lmd, body, env),
            Term::Application(lhs, rhs) => print_application(lhs, rhs, env),
        }
    }
    fn print_var(idx: &Option<usize>, env: &Vec<String>) -> String {
        if let Some(i) = idx {
            env[env.len() - i].clone()
        } else {
            "[Free]".to_string()
        }
    }
    fn print_lambda(lmd: &String, body: &Term, env: &mut Vec<String>) -> String {
        env.push(lmd.clone());
        let body_str = print_term(body, env);
        env.pop();
        format!("λ{} => ({})", lmd, body_str)
    }
    fn print_application(lhs: &Term, rhs: &Term, env: &mut Vec<String>) -> String {
        let lhs_str = print_term(lhs, env);
        let rhs_str = print_term(rhs, env);
        // add parentheses to rhs if missing
        if rhs_str.starts_with('(') && rhs_str.ends_with(')') {
            format!("{}{}", lhs_str, rhs_str)
        } else {
            format!("{}({})", lhs_str, rhs_str)
        }
    }
    let mut env = vec![];
    print_term(term, &mut env)
}

#[derive(PartialEq, Debug)]
enum Token {
    Var(String),
    Lambda, // '\'
    Dot,    // '.'
    LBrace, // '{'
    RBrace, // '}'
    Bra,    // '<'
    Delim,  // '|'
    Ket,    // '>'
}

fn tokenize(input: &str) -> Vec<Token> {
    fn ident_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
    fn ident_body(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    let mut iter = input.chars().peekable();
    let mut tok = Vec::new();
    while let Some(&c) = iter.peek() {
        match c {
            '\\' => {
                tok.push(Token::Lambda);
                iter.next();
            }
            '.' => {
                tok.push(Token::Dot);
                iter.next();
            }
            '{' => {
                tok.push(Token::LBrace);
                iter.next();
            }
            '}' => {
                tok.push(Token::RBrace);
                iter.next();
            }
            '<' => {
                tok.push(Token::Bra);
                iter.next();
            }
            '|' => {
                tok.push(Token::Delim);
                iter.next();
            }
            '>' => {
                tok.push(Token::Ket);
                iter.next();
            }
            c if ident_start(c) => {
                let mut var = String::new();
                while let Some(&c) = iter.peek() {
                    if ident_body(c) {
                        var.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                tok.push(Token::Var(var));
            }
            c if c.is_whitespace() => {
                iter.next(); // skip whitespace
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }
    tok
}

fn parse(tokens: &[Token]) -> Term {
    use std::slice::Iter;
    type PeekIter<'a> = Peekable<Iter<'a, Token>>;
    fn expect_token(iter: &mut PeekIter, expected: &Token, msg: &str) {
        if iter.next() != Some(expected) {
            panic!("{}", msg);
        }
    }
    fn expect_ident(iter: &mut PeekIter) -> String {
        if let Some(Token::Var(name)) = iter.next() {
            name.clone()
        } else {
            panic!("Expected identifier");
        }
    }
    fn parse_term(iter: &mut PeekIter, env: &mut Vec<String>) -> Term {
        match iter.peek() {
            Some(Token::Var(_)) => parse_var(iter, env),
            Some(Token::Lambda) => parse_lambda(iter, env),
            Some(Token::Bra) => parse_application(iter, env),
            _ => panic!("Unexpected token"),
        }
    }
    fn parse_var(iter: &mut PeekIter, env: &mut Vec<String>) -> Term {
        let ident = expect_ident(iter);
        // get de bruijn
        // de bruijn index is the distance to the its lambda
        if let Some(idx) = env.iter().rposition(|x| *x == ident) {
            let de_bruijn_index = env.len() - idx; // backwards. index starts from 1
            Term::Variable(Some(de_bruijn_index))
        } else {
            Term::Variable(None) // free variable
        }
    }
    fn parse_lambda(iter: &mut PeekIter, env: &mut Vec<String>) -> Term {
        iter.next(); // consume '\'
        // expect variable
        let param = expect_ident(iter);
        // expect '.'
        expect_token(iter, &Token::Dot, "Expected '.' after variable in lambda");
        // expect '{'
        expect_token(iter, &Token::LBrace, "Expected '{' after '.' in lambda");
        env.push(param.clone());
        // expect term as body
        let body = parse_term(iter, env);
        // expect '}'
        expect_token(iter, &Token::RBrace, "Expected '}' after lambda body");
        env.pop();
        Term::Lambda(param, Box::new(body))
    }
    fn parse_application(iter: &mut PeekIter, env: &mut Vec<String>) -> Term {
        iter.next(); // consume '<'
        let lhs = parse_term(iter, env);
        // expect '|'
        if let Some(Token::Delim) = iter.next() {
        } else {
            panic!("Expected delimiter '|' in application");
        };
        let rhs = parse_term(iter, env);
        // expect '>'
        if let Some(Token::Ket) = iter.next() {
        } else {
            panic!("Expected '>' after application");
        };
        Term::Application(Box::new(lhs), Box::new(rhs))
    }
    let mut iter = tokens.iter().peekable();
    let mut env = Vec::new();
    parse_term(&mut iter, &mut env)
}


fn main() {
    // S-combinator
    let input = r"\x.{\y.{\z.{<<x|z>|<y|z>>}}}";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);
    let term = parse(&tokens);
    println!("{}", pretty_print(&term));
    // should be:
    // (λy => {(λx => {$0})((λt => {$0})($0))})(λinput => {$0})
}
