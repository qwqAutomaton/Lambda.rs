/*
Syntax:
TERM = VAR | LAMBDA | APPLICATION
VAR = [a-zA-Z_][a-zA-Z0-9_]* -- normal identifier rules
LAMBDA = '\' VAR '.' '{' TERM '}' -- \x.{x+1} for example
APPLICATION = '<' TERM '|' TERM '>' -- something like Dirac, <\x.{x+1}|y>
*/

use core::panic;
use std::iter::Peekable;

enum Term {
    Variable(String),
    Lambda(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Variable(name) => write!(f, "\'{}\'", name),
            Term::Lambda(param, body) => write!(f, "({}) => ({:?})", param, body),
            Term::Application(lhs, rhs) => write!(f, "({:?})({:?})", lhs, rhs),
        }
    }
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
    fn parse_term(iter: &mut PeekIter) -> Term {
        match iter.peek() {
            Some(Token::Var(_)) => parse_var(iter),
            Some(Token::Lambda) => parse_lambda(iter),
            Some(Token::Bra) => parse_application(iter),
            _ => panic!("Unexpected token"),
        }
    }
    fn parse_var(iter: &mut PeekIter) -> Term {
        Term::Variable(expect_ident(iter))
    }
    fn parse_lambda(iter: &mut PeekIter) -> Term {
        iter.next(); // consume '\'
        // expect variable
        let param = expect_ident(iter);
        // expect '.'
        expect_token(iter, &Token::Dot, "Expected '.' after variable in lambda");
        // expect '{'
        expect_token(iter, &Token::LBrace, "Expected '{' after '.' in lambda");
        // expect term as body
        let body = parse_term(iter);
        // expect '}'
        expect_token(iter, &Token::RBrace, "Expected '}' after lambda body");
        Term::Lambda(param, Box::new(body))
    }
    fn parse_application(iter: &mut PeekIter) -> Term {
        iter.next(); // consume '<'
        let lhs = parse_term(iter);
        // expect '|'
        if let Some(Token::Delim) = iter.next() {
        } else {
            panic!("Expected delimiter '|' in application");
        };
        let rhs = parse_term(iter);
        // expect '>'
        if let Some(Token::Ket) = iter.next() {
        } else {
            panic!("Expected '>' after application");
        };
        Term::Application(Box::new(lhs), Box::new(rhs))
    }
    let mut iter = tokens.iter().peekable();
    parse_term(&mut iter)
}

// tests
#[cfg(test)]
mod tests {
    // test tokenizer
    use super::*;
    #[test]
    fn test_tokenize() {
        let input = r"< \x . {x} | < \t . {t} | y > >";
        let tokens = tokenize(input);
        let expected = vec![
            Token::Bra,
            Token::Lambda,
            Token::Var("x".to_string()),
            Token::Dot,
            Token::LBrace,
            Token::Var("x".to_string()),
            Token::RBrace,
            Token::Delim,
            Token::Bra,
            Token::Lambda,
            Token::Var("t".to_string()),
            Token::Dot,
            Token::LBrace,
            Token::Var("t".to_string()),
            Token::RBrace,
            Token::Delim,
            Token::Var("y".to_string()),
            Token::Ket,
            Token::Ket,
        ];
        assert_eq!(tokens, expected);
    }
}

fn main() {
    let input = r"< \x . {x} | < \t . {t} | y > >";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);
    let term = parse(&tokens);
    println!("Parsed Term: {:?}", term);
}

/*
Term *parse(const token *input, int len)
{
    if (input[0] is var) return parsevar(input, len);
    if (input[0] is lambda) return parselambda(input, len);
    if (input[0] is application) return parseapplication(input, len);
    error("Unexpected token");
}
*/
