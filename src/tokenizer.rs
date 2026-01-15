use core::panic;
use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Var(String), // any valid identifier
    Lambda,      // '\'
    Dot,         // '.'
    LBrace,      // '{'
    RBrace,      // '}'
    Bra,         // '<'
    Delim,       // '|'
    Ket,         // '>'
}
type PIter<'a> = Peekable<Chars<'a>>;
fn ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
fn ident_body(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
// extract an identifier token from the input
fn consume_identifier(iter: &mut PIter, chr1: char) -> Token {
    let mut varname = String::new();
    varname.push(chr1); // already consumed
    while let Some(chr) = iter.next_if(|&c| ident_body(c)) {
        varname.push(chr);
    }
    Token::Var(varname)
}
// extract 1 exact token from the input (ignore whitespaces)
// returns None if EOF (ignoring whitespaces)
// panic when unknown char encountered OR invalid identifier OR something went wrong with next_if
fn consume_token(iter: &mut PIter) -> Option<Token> {
    // loop until non-whitespace or EOF
    while iter.next_if(|c| c.is_whitespace()).is_some() {}
    // now iter.next is either None/EOF or a non-WS char
    match iter.next().unwrap_or_default() {
        // trivial tokens
        '\\' => Some(Token::Lambda),
        '.' => Some(Token::Dot),
        '{' => Some(Token::LBrace),
        '}' => Some(Token::RBrace),
        '<' => Some(Token::Bra),
        '|' => Some(Token::Delim),
        '>' => Some(Token::Ket),
        // identifier
        chr if ident_start(chr) => Some(consume_identifier(iter, chr)),
        // EOF, reserve for later use
        '\0' => None,
        // unknown char otherwise
        chr => panic!("Unknown character encountered during tokenization: {}", chr),
    }
}
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut iter = input.chars().peekable();
    let mut tokens = Vec::new();
    // consume token with extracted func
    while let Some(token) = consume_token(&mut iter) {
        tokens.push(token);
    }
    tokens
}
