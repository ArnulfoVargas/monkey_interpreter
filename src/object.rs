use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Default, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Error(String),

    #[default]
    Null,
}

impl Object {
    pub fn object_type(&self) -> String {
        match self {
            Self::Integer(_) => String::from("int"),
            Self::Boolean(_) => String::from("bool"),
            Self::Error(_) => String::from("t_error"),
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
            Self::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Option<Object> {
        return match self.store.get(name.as_str()) {
            Some(v) => Some(v.clone()),
            None => None,
        };
    }

    pub fn set(&mut self, name: String, value: Object) -> Option<Object> {
        self.store.insert(name.clone(), value);
        return self.get(name);
    }
}
