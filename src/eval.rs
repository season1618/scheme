use std::fmt;
use crate::parser::Expr;

use Expr::*;
use Data::*;

#[derive(Debug)]
pub enum Data {
    Num(u32),
    Bool(bool),
    Str(String),
}

#[derive(Debug)]
pub struct Env {
    env: Vec<Vec<(String, Expr)>>
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Num(val) => write!(f, "{}: number", val),
            Data::Bool(val) => write!(f, "{}: bool", if *val { "#t" } else { "#f" }),
            Data::Str(val) => write!(f, "{}: string", val),
        }
    }
}

impl Env {
    pub fn new() -> Self {
        Env { env: Vec::new() }
    }

    fn push_frame(&mut self) {
        self.env.push(Vec::new());
    }

    fn pop_frame(&mut self) {
        self.env.pop();
    }

    fn add(&mut self, ident: String, expr: Expr) {
        self.env.last_mut().unwrap().push((ident, expr));
    }
}

pub fn eval(expr: Expr, env: &mut Env) -> Result<Data, String> {
    let res = match expr {
        Let { binds, expr } => {
            env.push_frame();
            for (ident, expr) in binds {
                env.add(ident, expr);
            }
            let data = eval(*expr, env)?;
            env.pop_frame();
            data
        },
        Expr::Num(val) => Data::Num(val),
        Expr::Bool(val) => Data::Bool(val),
        Expr::Str(val) => Data::Str(val),
    };
    Ok(res)
}
