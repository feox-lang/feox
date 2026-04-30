use std::{cell::RefCell, io::{self, Read}, rc::Rc};

use crate::eval::{EvalError, EvalResult, Value};

pub fn len(args: Vec<Value>) -> EvalResult {
    if args.len() != 1 {
        return Err(EvalError::WrongNumberOfArgs(format!(
            "expected 1 got {}",
            args.len()
        )));
    }

    match &args[0] {
        Value::Array(a) => Ok(Value::Number(a.borrow().len() as i64)),
        _ => Err(EvalError::TypeError("argument of len has to be an array")),
    }
}

pub fn input(_: Vec<Value>) -> EvalResult {
    let lock = io::stdin().lock();

    let mut res: Vec<Value> = Vec::new();
    let mut space = false;

    for byte in lock.bytes() {
        let b = byte.unwrap();

        if b.is_ascii_whitespace() {
            if space {
                break;
            }
        } else {
            space = true;
            res.push(Value::Char(b as char));
        }
    }
    Ok(Value::Array(Rc::new(RefCell::new(res))))
}

pub fn print(args: Vec<Value>) -> EvalResult {
    if args.len() != 1 {
        return Err(EvalError::WrongNumberOfArgs(format!(
            "expected 1 got {}",
            args.len()
        )));
    }

    let obj = &args[0];
    match obj {
        Value::Number(n) => {
            print!("{}", n);
            Ok(Value::Nil)
        }
        Value::Array(a) => {
            let is_string = a.borrow().iter().all(|v| matches!(v, Value::Char(_)));

            if is_string {
                for v in a.borrow().iter() {
                    if let Value::Char(c) = v {
                        print!("{}", c);
                    }
                }
                Ok(Value::Nil)
            } else {
                for (i, v) in a.borrow().iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", v);
                }
                Ok(Value::Nil)
            }
        }
        Value::Nil => {
            print!("nil");
            Ok(Value::Nil)
        }
        Value::Char(c) => {
            print!("{}", c);
            Ok(Value::Nil)
        }
        _ => Err(EvalError::TypeError("can't print that type")),
    }
}

pub fn push(mut args: Vec<Value>) -> EvalResult {
    if args.len() != 2 {
        return Err(EvalError::WrongNumberOfArgs(format!(
            "expected 1 got {}",
            args.len()
        )));
    }
    let obj = args.remove(0);
    let val = args.remove(0);
    let obj = match obj {
        Value::Array(a) => a,
        _ => return Err(EvalError::TypeError("can only push to arrays")),
    };

    obj.borrow_mut().push(val);
    Ok(Value::Nil)
}
