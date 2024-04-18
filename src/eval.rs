use crate::data::{TopLevel, Defn, Expr, OprKind, Value, Proc, Env};

use Expr::*;
use OprKind::*;
use Value::*;

pub fn run(nodes: Vec<TopLevel>) {
    let mut env = Env::new();
    for node in nodes {
        match node {
            TopLevel::Defn(defn) => {
                match bind(defn, &mut env) {
                    Ok(()) => {},
                    Err(err) => { eprintln!("{}", err); return; },
                }
            },
            TopLevel::Expr(expr) => {
                match eval(expr, &mut env) {
                    Ok(val) => println!("{}", val),
                    Err(err) => { eprintln!("{}", err); return; },
                }
            },
        }
    }
}

fn bind(defn: Defn, env: &mut Env) -> Result<(), String> {
    let Defn { ident, expr } = defn;
    let value = eval(expr, env)?;
    env.add(ident, value);
    Ok(())
}

fn eval(expr: Expr, env: &mut Env) -> Result<Value, String> {
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
        Quote(s_expr) => *s_expr,
        If { cond, expr1, expr2 } => {
            if let Value::Bool(cond) = eval(*cond.clone(), env)? {
                if cond { eval(*expr1, env)? } else { eval(*expr2, env)? }
            } else {
                return Err(format!("{:?} is not condition", *cond));
            }
        },
        And { args } => {
            for arg in args {
                match eval(arg, env)? {
                    Value::Bool(true) => {},
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    _ => return Err(String::from("not boolean")),
                }
            }
            Value::Bool(true)
        },
        Or { args } => {
            for arg in args {
                match eval(arg, env)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {},
                    _ => return Err(String::from("not boolean")),
                }
            }
            Value::Bool(false)
        },
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
        Cons => {
            if args.len() == 2 {
                Ok(Pair { car: Box::new(args[0].clone()), cdr: Box::new(args[1].clone()) })
            } else {
                Err(String::from("the number of arguments is not 2"))
            }
        },
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
