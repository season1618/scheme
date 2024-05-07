use crate::data::{TopLevel, Defn, Expr, Value, Proc, Env};

use std::rc::Rc;
use std::cell::RefCell;

use Expr::*;
use Value::*;

pub fn exec(nodes: Vec<TopLevel>) -> Result<(), String> {
    let mut env = Env::new();
    for node in nodes {
        exec_line(node, &mut env)?;
    }
    Ok(())
}

pub fn exec_line(node: TopLevel, env: &mut Env) -> Result<(), String> {
    match node {
        TopLevel::Defn(defn) => bind(defn, env)?,
        TopLevel::Expr(expr) => println!("{}", eval(expr, env)?),
    }
    Ok(())
}

fn bind(defn: Defn, env: &mut Env) -> Result<(), String> {
    let Defn { ident, expr } = defn;
    let value = eval(expr, env)?;
    env.add(ident, value);
    Ok(())
}

fn eval(expr: Expr, env: &mut Env) -> Result<Value, String> {
    match expr {
        Let { binds, expr } => {
            let binds = binds.into_iter().map(|(ident, expr)| (ident, eval(expr, env))).collect::<Vec<_>>();
            let env = &mut env.push_frame();
            for (ident, value) in binds {
                env.add(ident, value?);
            }
            eval(*expr, env)
        },
        LetStar { binds, expr } => {
            let env = &mut env.push_frame();
            for (ident, expr) in binds {
                let value = eval(expr, env)?;
                env.add(ident, value);
            }
            eval(*expr, env)
        },
        LetRec { binds, expr } => {
            let env = &mut env.push_frame();
            for (ident, _) in &binds {
                env.add(ident.clone(), Value::Nil);
            }
            for (ident, expr) in binds {
                let value = eval(expr, env)?;
                env.set(ident, value)?;
            }
            eval(*expr, env)
        },
        Set { ident, expr } => {
            let value = eval(*expr, env)?;
            env.set(ident, value)
        },
        Var(ident) => env.find(&ident),
        Quote(s_expr) => Ok(*s_expr),
        Begin(exprs) => {
            let mut value = Value::Nil;
            for expr in exprs {
                value = eval(expr, env)?;
            }
            Ok(value)
        },
        If { cond, expr1, expr2 } => {
            if let Value::Bool(cond) = eval(*cond.clone(), env)? {
                if cond { eval(*expr1, env) } else { eval(*expr2, env) }
            } else {
                Err(format!("{:?} is not condition", *cond))
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
            Ok(Value::Bool(true))
        },
        Or { args } => {
            for arg in args {
                match eval(arg, env)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {},
                    _ => return Err(String::from("not boolean")),
                }
            }
            Ok(Value::Bool(false))
        },
        Apply { proc, args } => {
            let Proc(proc) = eval(*proc.clone(), env)? else {
                return Err(format!("{:?} is not procedure", proc));
            };
            let args = args.into_iter().map(|arg| eval(arg, env)).collect::<Result<_, _>>()?;

            match proc {
                Proc::Opr(opr) => eval_opr(opr, args),
                Proc::Lambda { env, params, expr } => {
                    let env = &mut env.push_frame();
                    for (param, arg) in params.into_iter().zip(args.into_iter()) {
                        env.add(param, arg);
                    }
                    eval(expr, env)
                },
            }
        },
        Lambda { params, expr } => Ok(Proc(Proc::Lambda { env: env.push_frame(), params, expr: *expr })),
        Expr::Opr(opr) => Ok(Proc(Proc::Opr(opr))),
        Expr::Num(val) => Ok(Value::Num(val)),
        Expr::Bool(val) => Ok(Value::Bool(val)),
        Expr::Str(val) => Ok(Value::Str(Rc::new(val))),
        Expr::Nil => Ok(Value::Nil),
    }
}

fn eval_opr(operator: &'static str, args: Vec<Value>) -> Result<Value, String> {
    match (operator, args.len()) {
        ("cons", 2) => Ok(Pair(Rc::new(RefCell::new((args[0].clone(), args[1].clone()))))),
        ("car" , 1) => {
            if let Pair(pair) = &args[0] {
                Ok((pair.borrow().0).clone())
            } else {
                Err(format!("{:?} is not pair", args[0]))
            }
        },
        ("cdr" , 1) => {
            if let Pair(pair) = &args[0] {
                Ok((pair.borrow().1).clone())
            } else {
                Err(format!("{:?} is not pair", args[0]))
            }
        },
        ("set-car!", 2) => {
            if let Pair(pair) = &args[0] {
                pair.borrow_mut().0 = args[1].clone();
                Ok(Pair(Rc::clone(pair)))
            } else {
                Err(format!("{:?} is not pair", args[0]))
            }
        },
        ("set-cdr!", 2) => {
            if let Pair(pair) = &args[0] {
                pair.borrow_mut().1 = args[1].clone();
                Ok(Pair(Rc::clone(pair)))
            } else {
                Err(format!("{:?} is not pair", args[0]))
            }
        },
        ("not", 1) => {
            if let Value::Bool(val) = args[0] {
                Ok(Value::Bool(!val))
            } else {
                Err(format!("'{:?}' is not boolean", args[0]))
            }
        },
        ("eq?"   , 2) => Ok(Value::Bool(Value::eq(&args[0], &args[1]))),
        ("neq?"  , 2) => Ok(Value::Bool(!Value::eq(&args[0], &args[1]))),
        ("equal?", 2) => Ok(Value::Bool(Value::equal(&args[0], &args[1]))),
        ("list?", 1) => Ok(Value::Bool(args[0].is_list())),
        (ident, 1) if ["pair?", "procedure?", "symbol?", "number?", "boolean?", "string?", "null?"].contains(&ident) => {
            match (operator, &args[0]) {
                ("pair?"     , Pair(_)       ) |
                ("procedure?", Proc(_)       ) |
                ("symbol?"   , Symbol(_)     ) |
                ("number?"   , Value::Num(_) ) |
                ("boolean?"  , Value::Bool(_)) |
                ("string?"   , Value::Str(_) ) |
                ("null?"     , Value::Nil    ) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            }
        },
        ("length", 1) => args[0].length().map(|val| Value::Num(val as f32)),
        ("last"  , 1) => args[0].last(),
        ("memq"  , 2) => Ok(Value::memq(&args[0], &args[1])),
        ("append", 2) => Value::append(&args[0], &args[1]),
        ("=" , _) => Ok(Value::Bool(args.windows(2).all(|p| p[0] == p[1]))),
        ("<" , _) => Ok(Value::Bool(args.windows(2).all(|p| p[0] <  p[1]))),
        ("<=", _) => Ok(Value::Bool(args.windows(2).all(|p| p[0] <= p[1]))),
        (">" , _) => Ok(Value::Bool(args.windows(2).all(|p| p[0] >  p[1]))),
        (">=", _) => Ok(Value::Bool(args.windows(2).all(|p| p[0] >= p[1]))),
        ("+" , _) => args.into_iter().fold(Ok(Value::Num(0.0)), |sum, val| sum.and_then(|sum| sum + val)),
        ("-" , _) => {
            let (minuend, subtrahends) = {
                let mut args = args.into_iter();
                (args.next().ok_or(String::from("'-' requires at least 1 argument")), args)
            };
            subtrahends.fold(minuend, |sum, val| sum.and_then(|sum| sum - val))
        },
        ("*" , _) => args.into_iter().fold(Ok(Value::Num(1.0)), |prod, val| prod.and_then(|prod| prod * val)),
        ("/" , _) => {
            let (dividend, divisors) = {
                let mut args = args.into_iter();
                (args.next().ok_or(String::from("'-' requires at least 1 argument")), args)
            };
            divisors.fold(dividend, |prod, val| prod.and_then(|prod| prod / val))
        },
        (_, n) => Err(format!("the number of argments is not {n}")),
    }
}
