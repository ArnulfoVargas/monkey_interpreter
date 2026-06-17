use std::{collections::HashMap, fmt::Display};

use crate::ast::{BlockStatement, Identifier, Node};

#[derive(Debug, Default, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Error(String),
    Function(Function),

    #[default]
    Null,
}

impl Object {
    pub fn object_type(&self) -> String {
        match self {
            Self::Integer(_) => String::from("int"),
            Self::Boolean(_) => String::from("bool"),
            Self::Error(_) => String::from("t_error"),
            Self::Function(_) => String::from("t_func"),
            Self::ReturnValue(res) => format!("t_return_of<{}>", res.object_type()),

            Self::Null => String::from("t_null"),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Error(b) => write!(f, "{}", b),
            Self::ReturnValue(b) => write!(f, "{}", *b),
            Self::Function(func) => {
                let mut out = String::from("");
                let mut params = vec![];

                for f in &func.parameters {
                    params.push(f.print_string());
                }

                out.push_str("fn");
                out.push_str("(");
                out.push_str(params.join(", ").as_str());
                out.push_str(") {");
                out.push_str(func.body.print_string().as_str());
                out.push_str("\n}");

                write!(f, "{}", out)
            }
            Self::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed_environment(outer: Box<Environment>) -> Environment {
        Environment {
            outer: Some(outer),
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Option<Object> {
        return match self.store.get(name.as_str()) {
            Some(v) => Some(v.clone()),
            None => match &self.outer {
                Some(env) => env.get(name),
                None => None,
            },
        };
    }

    pub fn set(&mut self, name: String, value: Object) -> Option<Object> {
        self.store.insert(name.clone(), value);
        return self.get(name);
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}
