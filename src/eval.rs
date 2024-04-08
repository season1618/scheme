use crate::parser::Expr;

use Expr::*;
use Data::*;

#[derive(Debug)]
pub enum Data {
    Num(u32),
    Bool(bool),
    Str(String),
}

pub fn eval(expr: Expr) -> Result<Data, String> {
    let res = match expr {
        Expr::Num(val) => Data::Num(val),
        Expr::Bool(val) => Data::Bool(val),
        Expr::Str(val) => Data::Str(val),
    };
    Ok(res)
}
