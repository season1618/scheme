use std::fmt;
use crate::parser::Expr;

use Expr::*;
use Data::*;

#[derive(Debug, Clone)]
pub enum Data {
    Proc { params: Vec<String>, expr: Expr },
    Num(u32),
    Bool(bool),
    Str(String),
    Nil,
}

#[derive(Debug)]
pub struct Env {
    env: Vec<Vec<(String, Data)>>
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Proc { params, expr } => write!(f, "{:?} -> {:?}: procedure", params, expr),
            Data::Num(val) => write!(f, "{}: number", val),
            Data::Bool(val) => write!(f, "{}: bool", if *val { "#t" } else { "#f" }),
            Data::Str(val) => write!(f, "{}: string", val),
            Data::Nil => write!(f, "()"),
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

    fn find(&self, expected: &String) -> Result<Data, String> {
        for frame in self.env.iter().rev() {
            for (ident, data) in frame.iter().rev() {
                if ident == expected {
                    return Ok(data.clone());
                }
            }
        }
        Err(format!("{:?} is undefined", expected))
    }

    fn set(&mut self, expected: String, new_data: Data) -> Result<Data, String> {
        for frame in self.env.iter_mut().rev() {
            for (ident, data) in frame.iter_mut().rev() {
                if *ident == expected {
                    *data = new_data.clone();
                    return Ok(new_data);
                }
            }
        }
        Err(format!("{:?} is undefined", expected))
    }
}

pub fn eval(expr: Expr, env: &mut Env) -> Result<Data, String> {
    let res = match expr {
        Apply { proc, args } => {
            let Proc { params, expr } = eval(*proc.clone(), env)? else {
                return Err(format!("{:?} is not procedure", proc));
            };
            let args = args.into_iter().map(|arg| eval(arg, env)).collect::<Vec<_>>();

            env.push_frame();
            for (param, arg) in params.into_iter().zip(args.into_iter()) {
                env.add(param, arg?);
            }
            let data = eval(expr, env)?;
            env.pop_frame();
            data
        },
        Lambda { params, expr } => {
            Proc { params, expr: *expr }
        },
        Let { binds, expr } => {
            let binds = binds.into_iter().map(|(ident, expr)| (ident, eval(expr, env))).collect::<Vec<_>>();

            env.push_frame();
            for (ident, data) in binds {
                env.add(ident, data?);
            }
            let data = eval(*expr, env)?;
            env.pop_frame();

            data
        },
        LetStar { binds, expr } => {
            env.push_frame();
            for (ident, expr) in binds {
                let data = eval(expr, env)?;
                env.add(ident, data);
            }
            let data = eval(*expr, env)?;
            env.pop_frame();

            data
        },
        Set { ident, expr } => {
            let data = eval(*expr, env)?;
            env.set(ident, data)?
        },
        Var(ident) => env.find(&ident)?,
        Expr::Num(val) => Data::Num(val),
        Expr::Bool(val) => Data::Bool(val),
        Expr::Str(val) => Data::Str(val),
        Expr::Nil => Data::Nil,
    };
    Ok(res)
}
