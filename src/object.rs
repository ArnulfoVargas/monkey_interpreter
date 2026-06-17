use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),

    #[default]
    Null,
}

impl Object {
    pub fn object_type(&self) -> String {
        match self {
            Self::Integer(_) => String::from("int"),
            Self::Boolean(_) => String::from("bool"),
            Self::ReturnValue(res) => format!("RETURN_VALUE<{}>", res.object_type()),
            Self::Null => String::from("t_null"),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::ReturnValue(b) => write!(f, "{}", *b),
            Self::Null => write!(f, "null"),
        }
    }
}

