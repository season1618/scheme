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

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Num(val) => write!(f, "{}: number", val),
            Data::Bool(val) => write!(f, "{}: bool", if *val { "#t" } else { "#f" }),
            Data::Str(val) => write!(f, "{}: string", val),
        }
    }
}

pub fn eval(expr: Expr) -> Result<Data, String> {
    let res = match expr {
        Expr::Num(val) => Data::Num(val),
        Expr::Bool(val) => Data::Bool(val),
        Expr::Str(val) => Data::Str(val),
    };
    Ok(res)
}
