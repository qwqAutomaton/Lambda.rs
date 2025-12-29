/*
Syntax:
TERM = VAR | LAMBDA | APPLICATION
VAR = [a-zA-Z_][a-zA-Z0-9_]* -- normal identifier rules
LAMBDA = '\\' VAR '.' '{' TERM '}' -- \x.{x+1} for example
APPLICATION = '<' TERM '|' TERM '>' -- something like Dirac, <\x.{x+1}|y>
*/

mod tokenizer;
mod parser;
mod pretty_printer;

use crate::pretty_printer::PrettyPrinter;

fn main() {
    // S-combinator
    let input = r"\x.{\y.{\z.{<<x|z>|<y|z>>}}}";
    let tokens = tokenizer::tokenize(input);
    let mut parser = parser::Parser::new(&tokens);
    println!("Tokens: {:?}", tokens);
    let term = parser.parse();
    let mut printer = PrettyPrinter::new();
    println!("{}", printer.format(&term));
    // should be:
    // (λy => {(λx => {$0})((λt => {$0})($0))})(λinput => {$0})
}
