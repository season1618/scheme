use crate::data::{Expr, OprKind, Value, Proc, Env};

use Expr::*;
use OprKind::*;
use Value::*;

pub fn eval(expr: Expr, env: &mut Env) -> Result<Value, String> {
    let res = match expr {
        Apply { proc, args } => {
            let Proc(proc) = eval(*proc.clone(), env)? else {
                return Err(format!("{:?} is not procedure", proc));
            };
            let args = args.into_iter().map(|arg| eval(arg, env)).collect::<Result<_, _>>()?;

            match proc {
                Proc::Opr(opr) => eval_opr(opr, args)?,
                Proc::Lambda { env, params, expr } => {
                    let env = &mut env.push_frame();
                    for (param, arg) in params.into_iter().zip(args.into_iter()) {
                        env.add(param, arg);
                    }
                    let value = eval(expr, env)?;

                    value
                },
            }
        },
        Lambda { params, expr } => {
            Proc(Proc::Lambda { env: env.push_frame(), params, expr: *expr })
        },
        Let { binds, expr } => {
            let binds = binds.into_iter().map(|(ident, expr)| (ident, eval(expr, env))).collect::<Vec<_>>();

            let env = &mut env.push_frame();
            for (ident, value) in binds {
                env.add(ident, value?);
            }
            let value = eval(*expr, env)?;

            value
        },
        LetStar { binds, expr } => {
            let env = &mut env.push_frame();
            for (ident, expr) in binds {
                let value = eval(expr, env)?;
                env.add(ident, value);
            }
            let value = eval(*expr, env)?;

            value
        },
        LetRec { binds, expr } => {
            let env = &mut env.push_frame();
            for (ident, _) in &binds {
                env.add(ident.clone(), Value::Nil);
            }
            for (ident, expr) in binds {
                let value = eval(expr, env)?;
                env.set(ident, value);
            }
            let value = eval(*expr, env)?;

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
