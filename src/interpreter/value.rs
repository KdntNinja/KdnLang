use crate::parser::ASTNode;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    #[allow(dead_code)] // Added to suppress warning
    Boolean(bool),
    #[allow(dead_code)] // Added to suppress warning
    Function(Vec<String>, Box<ASTNode>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Function(_, _) => write!(f, "<function>"),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    #[allow(dead_code)] // Added to suppress warning
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Function(_, _) => "function",
            Value::Null => "null",
        }
    }

    #[allow(dead_code)] // Added to suppress warning
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Function(_, _) => true,
        }
    }
}
