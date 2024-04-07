use crate::parser::Expr;

use Expr::*;

pub fn eval(expr: Expr) -> Result<u32, String> {
    let res = match expr {
        Num(val) => val,
    };
    Ok(res)
}
