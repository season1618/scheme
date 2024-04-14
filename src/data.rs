use std::fmt;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};

use Value::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Keyword(String),
    Ident(String),
    Num(f32),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Apply { proc: Box<Expr>, args: Vec<Expr> },
    Lambda { params: Vec<String>, expr: Box<Expr> },
    Let { binds: Vec<(String, Expr)>, expr: Box<Expr> },
    LetStar { binds: Vec<(String, Expr)>, expr: Box<Expr> },
    LetRec { binds: Vec<(String, Expr)>, expr: Box<Expr> },
    Set { ident: String, expr: Box<Expr> },
    Var(String),
    Opr(OprKind),
    Num(f32),
    Bool(bool),
    Str(String),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OprKind {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
}

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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Proc(Proc::Lambda { params, expr }) => write!(f, "{:?} -> {:?}: procedure", params, expr),
            Proc(Proc::Opr(opr)) => write!(f, "{:?}: procedure", opr),
            Value::Num(val) => write!(f, "{}: number", val),
            Value::Bool(val) => write!(f, "{}: bool", if *val { "#t" } else { "#f" }),
            Value::Str(val) => write!(f, "{}: string", val),
            Value::Nil => write!(f, "()"),
        }
    }
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
