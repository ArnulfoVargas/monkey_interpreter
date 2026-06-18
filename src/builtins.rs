use crate::object::Object;

pub struct Builtins;

impl Builtins {
    pub fn all_builtins(&self) -> Vec<(String, Object)> {
        vec![
            (String::from("len"), Object::BuiltinFunction(b_len)),
            (String::from("first"), Object::BuiltinFunction(b_first)),
            (String::from("last"), Object::BuiltinFunction(b_last)),
            (String::from("push"), Object::BuiltinFunction(b_push)),
            (
                String::from("removeAt"),
                Object::BuiltinFunction(b_remove_at),
            ),
        ]
    }
}

fn b_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got = {}, want = 1",
            args.len(),
        ));
    }

    return match &args[0] {
        Object::String(str) => Object::Integer(str.len() as i64),
        Object::Array(arr) => Object::Integer(arr.len() as i64),
        other => Object::Error(format!(
            "argument to 'len' not supported. got = {}",
            other.object_type()
        )),
    };
}

fn b_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got = {}, want = 1",
            args.len(),
        ));
    }

    return match &args[0] {
        Object::String(str) => {
            let mut out = String::from("");

            if str.len() > 0 {
                let ch = str.chars().nth(0);
                if let Some(c) = ch {
                    out.push(c);

                    return Object::String(out);
                }
            }
            return Object::Error(String::from("cannot get first of empty string"));
        }
        Object::Array(arr) => {
            if arr.len() > 0 {
                return arr[0].clone();
            }
            Object::Error(String::from("cannot get first of empty array"))
        }
        other => Object::Error(format!(
            "argument to 'first' not supported. got = {}",
            other.object_type()
        )),
    };
}

fn b_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got = {}, want = 1",
            args.len(),
        ));
    }

    return match &args[0] {
        Object::String(str) => {
            let mut out = String::from("");

            if str.len() > 0 {
                let ch = str.chars().last();
                if let Some(c) = ch {
                    out.push(c);

                    return Object::String(out);
                }
            }
            return Object::Error(String::from("cannot get last of empty string"));
        }
        Object::Array(arr) => {
            if arr.len() > 0 {
                if let Some(v) = arr.last() {
                    return v.clone();
                }
            }
            Object::Error(String::from("cannot get last of empty array"))
        }
        other => Object::Error(format!(
            "argument to 'last' not supported. got = {}",
            other.object_type()
        )),
    };
}

fn b_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. got = {}, want = 2",
            args.len(),
        ));
    }

    return match &args[0] {
        Object::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Object::Array(new_arr)
        }
        other => Object::Error(format!(
            "argument to 'last' not supported. got = {}",
            other.object_type()
        )),
    };
}

fn b_remove_at(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. got = {}, want = 2",
            args.len(),
        ));
    }

    let (arr, index) = (&args[0], &args[1]);

    return match (arr, index) {
        (Object::Array(arr), Object::Integer(idx)) => {
            let mut new_arr = arr.clone();

            if *idx < 0 || *idx >= (arr.len() as i64) {
                return Object::Error(String::from("index out of bounds"));
            }

            new_arr.remove(*idx as usize);
            Object::Array(new_arr)
        }
        (arr, idx) => Object::Error(format!(
            "argument to 'removeAt' not supported by {} with index of {}. expected array with int index",
            arr.object_type(),
            idx.object_type()
        )),
    };
}
