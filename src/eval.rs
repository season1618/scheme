use std::fmt;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};
use crate::parser::{Expr, OprKind};

use Expr::*;
use OprKind::*;
use Value::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Proc(Proc),
    Num(f32),
    Bool(bool),
    Str(String),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Proc {
    Lambda { params: Vec<String>, expr: Expr },
    Opr(OprKind),
}

#[derive(Debug)]
pub struct Env {
    env: Vec<Vec<(String, Value)>>
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Num(lhs), Value::Num(rhs)) => f32::partial_cmp(lhs, rhs),
            (Value::Bool(lhs), Value::Bool(rhs)) => bool::partial_cmp(lhs, rhs),
            (Value::Str(lhs), Value::Str(rhs)) => String::partial_cmp(lhs, rhs),
            _ => None,
        }
    }
}

impl Add for Value {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(lhs), Value::Num(rhs)) => Ok(Value::Num(lhs + rhs)),
            (Value::Num(_), rhs) => Err(format!("{:?} is not a number", rhs)),
            lhs => Err(format!("{:?} is not a number", lhs)),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(lhs), Value::Num(rhs)) => Ok(Value::Num(lhs - rhs)),
            (Value::Num(_), rhs) => Err(format!("{:?} is not a number", rhs)),
            lhs => Err(format!("{:?} is not a number", lhs)),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(lhs), Value::Num(rhs)) => Ok(Value::Num(lhs * rhs)),
            (Value::Num(_), rhs) => Err(format!("{:?} is not a number", rhs)),
            lhs => Err(format!("{:?} is not a number", lhs)),
        }
    }
}

impl Div for Value {
    type Output = Result<Self, String>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Num(lhs), Value::Num(rhs)) => Ok(Value::Num(lhs / rhs)),
            (Value::Num(_), rhs) => Err(format!("{:?} is not a number", rhs)),
            lhs => Err(format!("{:?} is not a number", lhs)),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Proc (Proc::Lambda { params, expr }) => write!(f, "{:?} -> {:?}: procedure", params, expr),
            Proc(Proc::Opr(opr)) => write!(f, "{:?}: procedure", opr),
            Value::Num(val) => write!(f, "{}: number", val),
            Value::Bool(val) => write!(f, "{}: bool", if *val { "#t" } else { "#f" }),
            Value::Str(val) => write!(f, "{}: string", val),
            Value::Nil => write!(f, "()"),
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

    fn add(&mut self, ident: String, value: Value) {
        self.env.last_mut().unwrap().push((ident, value));
    }

    fn find(&self, expected: &String) -> Result<Value, String> {
        for frame in self.env.iter().rev() {
            for (ident, value) in frame.iter().rev() {
                if ident == expected {
                    return Ok(value.clone());
                }
            }
        }
        Err(format!("{:?} is undefined", expected))
    }

    fn set(&mut self, expected: String, new_value: Value) -> Result<Value, String> {
        for frame in self.env.iter_mut().rev() {
            for (ident, value) in frame.iter_mut().rev() {
                if *ident == expected {
                    *value = new_value.clone();
                    return Ok(new_value);
                }
            }
        }
        Err(format!("{:?} is undefined", expected))
    }
}

pub fn eval(expr: Expr, env: &mut Env) -> Result<Value, String> {
    let res = match expr {
        Apply { proc, args } => {
            let Proc(proc) = eval(*proc.clone(), env)? else {
                return Err(format!("{:?} is not procedure", proc));
            };
            let args = args.into_iter().map(|arg| eval(arg, env)).collect::<Result<_, _>>()?;

            match proc {
                Proc::Opr(opr) => eval_opr(opr, args)?,
                Proc::Lambda { params, expr } => {
                    env.push_frame();
                    for (param, arg) in params.into_iter().zip(args.into_iter()) {
                        env.add(param, arg);
                    }
                    let value = eval(expr, env)?;
                    env.pop_frame();
                    value
                },
            }
        },
        Lambda { params, expr } => {
            Proc(Proc::Lambda { params, expr: *expr })
        },
        Let { binds, expr } => {
            let binds = binds.into_iter().map(|(ident, expr)| (ident, eval(expr, env))).collect::<Vec<_>>();

            env.push_frame();
            for (ident, value) in binds {
                env.add(ident, value?);
            }
            let value = eval(*expr, env)?;
            env.pop_frame();

            value
        },
        LetStar { binds, expr } => {
            env.push_frame();
            for (ident, expr) in binds {
                let value = eval(expr, env)?;
                env.add(ident, value);
            }
            let value = eval(*expr, env)?;
            env.pop_frame();

            value
        },
        LetRec { binds, expr } => {
            env.push_frame();
            for (ident, _) in &binds {
                env.add(ident.clone(), Value::Nil);
            }
            for (ident, expr) in binds {
                let value = eval(expr, env)?;
                env.set(ident, value);
            }
            let value = eval(*expr, env)?;
            env.pop_frame();

            value
        },
        Set { ident, expr } => {
            let value = eval(*expr, env)?;
            env.set(ident, value)?
        },
        Var(ident) => env.find(&ident)?,
        Expr::Opr(opr) => Proc(Proc::Opr(opr)),
        Expr::Num(val) => Value::Num(val),
        Expr::Bool(val) => Value::Bool(val),
        Expr::Str(val) => Value::Str(val),
        Expr::Nil => Value::Nil,
    };
    Ok(res)
}

fn eval_opr(opr: OprKind, args: Vec<Value>) -> Result<Value, String> {
    match opr {
        Eq => Ok(Value::Bool(args.windows(2).all(|p| p[0] == p[1]))),
        Lt => Ok(Value::Bool(args.windows(2).all(|p| p[0] <  p[1]))),
        Le => Ok(Value::Bool(args.windows(2).all(|p| p[0] <= p[1]))),
        Gt => Ok(Value::Bool(args.windows(2).all(|p| p[0] >  p[1]))),
        Ge => Ok(Value::Bool(args.windows(2).all(|p| p[0] >= p[1]))),
        Add => args.into_iter().fold(Ok(Value::Num(0.0)), |sum, val| sum.and_then(|sum| sum + val)),
        Sub => {
            let (minuend, subtrahends) = {
                let mut args = args.into_iter();
                (args.next().ok_or(String::from("'-' requires at least 1 argument")), args)
            };
            subtrahends.fold(minuend, |sum, val| sum.and_then(|sum| sum - val))
        },
        Mul => args.into_iter().fold(Ok(Value::Num(1.0)), |prod, val| prod.and_then(|prod| prod * val)),
        Div => {
            let (dividend, divisors) = {
                let mut args = args.into_iter();
                (args.next().ok_or(String::from("'-' requires at least 1 argument")), args)
            };
            divisors.fold(dividend, |prod, val| prod.and_then(|prod| prod / val))
        },
    }
}
