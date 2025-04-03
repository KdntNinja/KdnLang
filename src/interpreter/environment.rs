use crate::interpreter::Value;
use crate::stdlib;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let local_value: Option<&Value> = self.variables.get(name);
        match local_value {
            Some(value) => Some(value.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    parent.get(name)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_builtin_function(&self, name: &str) -> Option<stdlib::BuiltinFunction> {
        let builtins: &HashMap<String, stdlib::BuiltinFunction> = stdlib::get_builtins();
        builtins.get(name).copied()
    }

    // New method to get all variables in the current environment
    pub fn get_all_vars(&self) -> HashMap<String, Value> {
        self.variables.clone()
    }
}
