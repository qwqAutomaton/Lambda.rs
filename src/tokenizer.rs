use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Var(String),
    Lambda, // '\'
    Dot,    // '.'
    LBrace, // '{'
    RBrace, // '}'
    Bra,    // '<'
    Delim,  // '|'
    Ket,    // '>'
}
type PIter<'a> = Peekable<Chars<'a>>;
fn ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
fn ident_body(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
fn tok_var(iter: &mut PIter) -> Token {
    let mut name = String::new();
    while let Some(&c) = iter.peek() {
        if ident_body(c) {
            name.push(c);
            iter.next();
        } else {
            break;
        }
    }
    Token::Var(name)
}
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut iter = input.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(&c) = iter.peek() {
        match c {
            '\\' => {
                tokens.push(Token::Lambda);
                iter.next();
            }
            '.' => {
                tokens.push(Token::Dot);
                iter.next();
            }
            '{' => {
                tokens.push(Token::LBrace);
                iter.next();
            }
            '}' => {
                tokens.push(Token::RBrace);
                iter.next();
            }
            '<' => {
                tokens.push(Token::Bra);
                iter.next();
            }
            '|' => {
                tokens.push(Token::Delim);
                iter.next();
            }
            '>' => {
                tokens.push(Token::Ket);
                iter.next();
            }
            c if ident_start(c) => tokens.push(tok_var(&mut iter)),
            c if c.is_whitespace() => {
                iter.next();
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }
    tokens
}
