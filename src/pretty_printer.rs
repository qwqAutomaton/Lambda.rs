use crate::parser::Term;

pub struct PrettyPrinter {
    env: Vec<String>,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self { env: Vec::new() }
    }

    pub fn format(&mut self, term: &Term) -> String {
        self.env.clear();
        self.print_term(term)
    }

    fn print_term(&mut self, term: &Term) -> String {
        match term {
            Term::Variable(index) => self.print_var(index),
            Term::Lambda(param, body) => self.print_lambda(param, body),
            Term::Application(lhs, rhs) => self.print_application(lhs, rhs),
        }
    }

    fn print_var(&self, index: &Option<usize>) -> String {
        if let Some(depth) = index {
            let pos = self.env.len() - depth;
            self.env[pos].clone()
        } else {
            "[Free]".to_string()
        }
    }

    fn print_lambda(&mut self, param: &String, body: &Term) -> String {
        self.env.push(param.clone());
        let body_str = self.print_term(body);
        self.env.pop();
        format!("λ{} => ({})", param, body_str)
    }

    fn print_application(&mut self, lhs: &Term, rhs: &Term) -> String {
        let lhs_str = self.print_term(lhs);
        let rhs_str = self.print_term(rhs);
        if rhs_str.starts_with('(') && rhs_str.ends_with(')') {
            format!("{}{}", lhs_str, rhs_str)
        } else {
            format!("{}({})", lhs_str, rhs_str)
        }
    }
}
