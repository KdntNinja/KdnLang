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

    #[allow(dead_code)]
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
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

    #[allow(dead_code)]
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value)
        } else {
            Err(format!("Undefined variable '{}'", name))
        }
    }

    pub fn get_builtin_function(&self, name: &str) -> Option<stdlib::BuiltinFunction> {
        let builtins: &HashMap<String, stdlib::BuiltinFunction> = stdlib::get_builtins();
        builtins.get(name).copied()
    }
}
