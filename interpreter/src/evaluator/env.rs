use std::collections::HashMap;
use crate::core::value::Value;

// TODO: Scopes!

pub struct Environment {
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        } 

        Err(format!("Use of undefined variable '{}'.", name))
    }
}