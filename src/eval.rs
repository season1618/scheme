use std::fmt;
use crate::parser::Expr;

use Expr::*;
use Data::*;

#[derive(Debug, Clone)]
pub enum Data {
    Num(u32),
    Bool(bool),
    Str(String),
}

#[derive(Debug)]
pub struct Env {
    env: Vec<Vec<(String, Data)>>
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

    fn add(&mut self, ident: String, data: Data) {
        self.env.last_mut().unwrap().push((ident, data));
    }

    fn find(&self, expected: &String) -> Option<Data> {
        for frame in self.env.iter().rev() {
            for (ident, data) in frame.iter().rev() {
                if ident == expected {
                    return Some(data.clone());
                }
            }
        }
        None
    }
}

pub fn eval(expr: Expr, env: &mut Env) -> Result<Data, String> {
    let res = match expr {
        Let { binds, expr } => {
            env.push_frame();
            for (ident, expr) in binds {
                let data = eval(expr, env)?;
                env.add(ident, data);
            }
            let data = eval(*expr, env)?;
            env.pop_frame();
            data
        },
        Var(ident) => {
            match env.find(&ident) {
                Some(data) => data,
                None => return Err(format!("{:?} is undefined", ident)),
            }
        },
        Expr::Num(val) => Data::Num(val),
        Expr::Bool(val) => Data::Bool(val),
        Expr::Str(val) => Data::Str(val),
    };
    Ok(res)
}
