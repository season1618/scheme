use std::fmt;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};
use std::rc::Rc;
use std::cell::RefCell;

use Value::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    OpenParen,
    CloseParen,
    SingleQuote,
    Keyword(&'static str),
    Operator(&'static str),
    Ident(&'a str),
    Num(f32),
    Bool(bool),
    Str(String),
}

#[derive(Debug)]
pub enum TopLevel {
    Defn(Defn),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Body {
    pub defns: Vec<Defn>,
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Defn {
    pub ident: String,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Apply { proc: Box<Expr>, args: Vec<Expr> },
    Lambda { params: Vec<String>, body: Body },
    Let { binds: Vec<(String, Expr)>, body: Body },
    LetStar { binds: Vec<(String, Expr)>, body: Body },
    LetRec { binds: Vec<(String, Expr)>, body: Body },
    Set { ident: String, expr: Box<Expr> },
    Var(String),
    Quote(Box<Value>),
    Begin(Vec<Expr>),
    If { cond: Box<Expr>, expr1: Box<Expr>, expr2: Box<Expr> },
    Cond { cond_then: Vec<(Expr, Expr)> },
    And { args: Vec<Expr> },
    Or { args: Vec<Expr> },
    Do { binds: Vec<(String, Expr, Expr)>, test: Box<Expr>, exprs: Vec<Expr>, body: Box<Expr> },
    Opr(&'static str),
    Num(f32),
    Bool(bool),
    Str(String),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Value {
    Pair(Rc<RefCell<(Value, Value)>>),
    Proc(Proc),
    Symbol(String),
    Num(f32),
    Bool(bool),
    Str(Rc<String>),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Proc {
    Lambda { env: Env, params: Vec<String>, body: Body },
    Opr(&'static str),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pair(pair) => write!(f, "({} . {})", pair.borrow().0, pair.borrow().1),
            Proc(Proc::Lambda { env, params, body }) => {
                write!(f, "env\n")?;
                write!(f, "{}", env)?;
                write!(f, "{:?} -> {:?}", params, body)
            },
            Proc(Proc::Opr(opr)) => write!(f, "{:?}", opr),
            Symbol(symbol) => symbol.fmt(f),
            Value::Num(val) => val.fmt(f),
            Value::Bool(val) => if *val { "#t" } else { "#f" }.fmt(f),
            Value::Str(val) => write!(f, "\"{}\"", val),
            Value::Nil => "()".fmt(f),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        Value::equal(self, other)
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

impl Value {
    pub fn eq(lhs: &Value, rhs: &Value) -> bool {
        match (lhs, rhs) {
            (Pair(lhs), Pair(rhs)) => Rc::ptr_eq(lhs, rhs),
            (Proc(_), Proc(_)) => false,
            (Symbol(lhs)     , Symbol(rhs)     ) => lhs == rhs,
            (Value::Num(lhs) , Value::Num(rhs) ) => lhs == rhs,
            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::Str(lhs) , Value::Str(rhs) ) => Rc::ptr_eq(lhs, rhs),
            (Value::Nil      , Value::Nil      ) => true,
            _ => false,
        }
    }

    pub fn equal(lhs: &Value, rhs: &Value) -> bool {
        match (lhs, rhs) {
            (Pair(lhs), Pair(rhs)) => Self::equal(&lhs.borrow().0, &rhs.borrow().0) && Self::equal(&lhs.borrow().1, &rhs.borrow().1),
            (Proc(_), Proc(_)) => false,
            (Symbol(lhs)     , Symbol(rhs)     ) => lhs == rhs,
            (Value::Num(lhs) , Value::Num(rhs) ) => lhs == rhs,
            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::Str(lhs) , Value::Str(rhs) ) => lhs == rhs,
            (Value::Nil      , Value::Nil      ) => true,
            _ => false,
        }
    }

    pub fn list(values: Vec<Value>) -> Value {
        let mut list = Value::Nil;
        for value in values.into_iter().rev() {
            list = Pair(Rc::new(RefCell::new((value, list))));
        }
        list
    }

    pub fn is_list(&self) -> bool {
        match self {
            Pair(pair) => pair.borrow().1.is_list(),
            Value::Nil => true,
            _ => false,
        }
    }
    
    pub fn length(&self) -> Result<u32, String> {
        match self {
            Pair(pair) => Ok(1 + pair.borrow().1.length()?),
            Value::Nil => Ok(0),
            _ => Err(String::from("not list")),
        }
    }

    pub fn last(&self) -> Result<Value, String> {
        match self {
            Pair(pair) => {
                if let Pair(_) = pair.borrow().1 {
                    pair.borrow().1.last()
                } else {
                    Ok(pair.borrow().0.clone())
                }
            },
            _ => Err(String::from("not pair")),
        }
    }
    
    pub fn memq(first: &Value, list: &Value) -> Value {
        match list {
            Pair(pair) => {
                if *first == pair.borrow().0 {
                    Pair(Rc::clone(pair))
                } else {
                    Self::memq(first, &pair.borrow().1)
                }
            },
            _ => Value::Bool(false),
        }
    }
    
    pub fn append(list1: &Value, list2: &Value) -> Result<Value, String> {
        match list1 {
            Pair(pair) => Ok(Pair(Rc::new(RefCell::new((pair.borrow().0.clone(), Self::append(&pair.borrow().1, list2)?))))),
            Value::Nil => Ok(list2.clone()),
            _ => Err(String::from("not list")),
        }
    }

    pub fn string_append(values: Vec<Value>) -> Result<Value, String> {
        let mut sum = String::new();
        for value in values {
            if let Value::Str(string) = value {
                sum.push_str(&string);
            } else {
                return Err(String::from("not string"));
            }
        }
        Ok(Value::Str(Rc::new(sum)))
    }

    pub fn symbol_to_string(&self) -> Result<Value, String> {
        if let Symbol(symbol) = self {
            Ok(Value::Str(Rc::new(symbol.clone())))
        } else {
            Err(String::from("not symbol"))
        }
    }

    pub fn string_to_symbol(&self) -> Result<Value, String> {
        if let Value::Str(string) = self {
            Ok(Symbol((**string).clone()))
        } else {
            Err(String::from("not string"))
        }
    }

    pub fn number_to_string(&self) -> Result<Value, String> {
        if let Value::Num(val) = self {
            Ok(Value::Str(Rc::new(val.to_string())))
        } else {
            Err(String::from("not number"))
        }
    }

    pub fn string_to_number(&self) -> Result<Value, String> {
        if let Value::Str(string) = self {
            match string.parse() {
                Ok(val) => Ok(Value::Num(val)),
                Err(_) => Err(String::from("can not convert to number"))
            }
        } else {
            Err(String::from("not string"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Env(Rc<RefCell<(Vec<(String, Value)>, Option<Env>)>>);

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parent = &self.0.borrow().1;
        if let Some(parent) = parent {
            parent.fmt(f)?;
        }
        let frame = &self.0.borrow().0;
        for (ident, value) in frame {
            let disp = if let Proc(_) = value { Symbol(String::from("procedure")) } else { value.clone() };
            write!(f, "{}: {}, ", ident, disp)?;
        }
        write!(f, "\n")
    }
}

impl Env {
    pub fn new() -> Self {
        Env(Rc::new(RefCell::new(
            (Vec::new(), None)
        )))
    }

    pub fn push_frame(&self) -> Self {
        Env(Rc::new(RefCell::new(
            (Vec::new(), Some(Env(Rc::clone(&self.0))))
        )))
    }

    pub fn add(&self, ident: &String, value: Value) {
        self.0.borrow_mut().0.push((ident.clone(), value));
    }

    pub fn find(&self, expected: &String) -> Result<Value, String> {
        for (ident, value) in self.0.borrow().0.iter().rev() {
            if ident == expected {
                return Ok(value.clone());
            }
        }
        if let Some(parent) = &self.0.borrow().1 {
            parent.find(expected)
        } else {
            Err(format!("{:?} is undefined", expected))
        }
    }

    pub fn set(&mut self, expected: &String, new_value: Value) -> Result<Value, String> {
        for (ident, value) in self.0.borrow_mut().0.iter_mut().rev() {
            if ident == expected {
                *value = new_value.clone();
                return Ok(new_value);
            }
        }
        if let Some(parent) = &mut self.0.borrow_mut().1 {
            parent.set(expected, new_value)
        } else {
            Err(format!("{:?} is undefined", expected))
        }
    }
}
