use crate::parser::Term;

const MAXLEN: usize = 10;

pub struct PrettyPrinter {
    env: Vec<String>,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self { env: Vec::new() }
    }

    pub fn format(&mut self, term: &Term, free: &[String]) -> String {
        self.env.clear();
        self.print_term(term, free)
    }

    fn print_term(&mut self, term: &Term, free: &[String]) -> String {
        match term {
            Term::Variable(index) => self.print_var(*index, free),
            Term::Lambda(param, body) => self.print_lambda(param, body, free),
            Term::Application(lhs, rhs) => self.print_application(lhs, rhs, free),
        }
    }

    fn print_var(&self, index: i32, free: &[String]) -> String {
        if index < 0 {
            let freepos = -(index + 1) as usize;
            format!("${}", free[freepos])
        } else {
            let bindpos = self.env.len() - (index as usize);
            self.env[bindpos].clone()
        }
    }

    fn print_lambda(&mut self, param: &String, body: &Term, free: &[String]) -> String {
        self.env.push(param.clone());
        let body_str = self.print_term(body, free);
        self.env.pop();
        let fmtbody = if body_str.len() > MAXLEN {
            Self::addparen(&body_str)
        } else {
            body_str
        };
        format!("λ{}. {}", param, fmtbody)
    }

    fn addparen(s: &String) -> String {
        if s.starts_with('(') && s.ends_with(')') {
            s.clone()
        } else {
            format!("({})", s)
        }
    }

    fn print_application(&mut self, lhs: &Term, rhs: &Term, free: &[String]) -> String {
        let lhs_str = self.print_term(lhs, free);
        let rhs_str = self.print_term(rhs, free);
        // add parentheses for lhs if len > MAXLEN
        let fmtlhs = if lhs_str.len() > MAXLEN {
            Self::addparen(&lhs_str)
        } else {
            lhs_str
        };
        format!("{}{}", fmtlhs, Self::addparen(&rhs_str))
    }
}
