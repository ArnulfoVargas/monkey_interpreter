use std::{
    collections::HashMap,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    ast::{BlockStatement, Identifier, Node},
    builtins::Builtins,
};

pub type BuiltinFunction = fn(Vec<Object>) -> Object;

#[derive(Debug, Default, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Error(String),
    Function(Function),
    String(String),
    BuiltinFunction(BuiltinFunction),
    Array(Vec<Object>),
    Hash(HashStruct),

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
            Self::String(_) => String::from("str"),
            Self::BuiltinFunction(_) => String::from("t_func"),
            Self::ReturnValue(res) => format!("t_return_of<{}>", res.object_type()),
            Self::Array(_) => String::from("t_array"),
            Self::Hash(_) => String::from("t_hash"),

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
            Self::String(b) => write!(f, "{}", *b),
            Self::BuiltinFunction(_) => write!(f, "builtin_t_func"),
            Self::Function(func) => {
                let mut out = String::from("");
                let mut params = vec![];

                for f in &func.parameters {
                    params.push(f.print_string());
                }

                out.push_str("fn");
                out.push_str("(");
                out.push_str(params.join(", ").as_str());
                out.push_str(") {\n");
                out.push_str(func.body.print_string().as_str());
                out.push_str("\n}");

                write!(f, "{}", out)
            }
            Self::Array(arr) => {
                let mut out = String::from("");
                let mut elements = vec![];

                for el in arr {
                    elements.push(format!("{}", el));
                }

                out.push('[');
                out.push_str(elements.join(", ").as_str());
                out.push(']');

                write!(f, "{}", out)
            }
            Self::Hash(hash) => {
                let mut out = String::from("");
                let mut pairs = vec![];

                for (_, pair) in &hash.pairs {
                    pairs.push(format!(
                        "{} : {}",
                        pair.key.to_string().as_str(),
                        pair.value.to_string().as_str()
                    ));
                }

                out.push('{');
                out.push_str(pairs.join(", ").as_str());
                out.push('}');

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
        let mut env_map = HashMap::new();
        Self::init_builtins(&mut env_map);

        Environment {
            store: env_map,
            outer: None,
        }
    }

    pub fn new_enclosed_environment(outer: Box<Environment>) -> Environment {
        let mut env_map = HashMap::new();
        Self::init_builtins(&mut env_map);

        Environment {
            outer: Some(outer),
            store: env_map,
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

    fn init_builtins(map: &mut HashMap<String, Object>) {
        let builtin_funcs = Builtins.all_builtins();

        for (name, object) in builtin_funcs {
            map.insert(name, object);
        }
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

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct HashKey {
    pub object_type: String,
    pub value: i64,
}

impl Hash for HashKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.object_type.hash(state);
        self.value.hash(state);
    }
}

pub trait Hashable {
    fn hash_key(&self) -> Result<HashKey, String>;
}

impl Hashable for Object {
    fn hash_key(&self) -> Result<HashKey, String> {
        match &self {
            Object::Boolean(val) => {
                let value = if *val { 1 } else { 0 };
                Ok(HashKey {
                    object_type: self.object_type(),
                    value: value,
                })
            }
            Object::Integer(val) => Ok(HashKey {
                object_type: self.object_type(),
                value: *val,
            }),
            Object::String(string) => {
                let mut hasher = DefaultHasher::new();
                string.hash(&mut hasher);

                Ok(HashKey {
                    object_type: self.object_type(),
                    value: hasher.finish() as i64,
                })
            }
            other => Err(String::from(format!(
                "cannot hash type {}",
                other.object_type().as_str(),
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HashMapObject {
    pub key: Object,
    pub value: Object,
}

#[derive(Clone, Debug)]
pub struct HashStruct {
    pub pairs: HashMap<HashKey, HashMapObject>,
}

#[cfg(test)]
mod test {
    use super::{Hashable, Object};

    #[test]
    fn test_string_hash_key() {
        let hello1 = Object::String("Hello World".to_string());
        let hello2 = Object::String("Hello World".to_string());
        let some_other = Object::String("Some Other".to_string());

        assert_eq!(
            hello1.hash_key(),
            hello2.hash_key(),
            "strings with same content have different hash keys"
        );
        assert_ne!(
            hello1.hash_key(),
            some_other.hash_key(),
            "strings with different content have same hash keys"
        );
    }
}
