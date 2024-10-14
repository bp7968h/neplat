use std::collections::HashMap;

use crate::lexer::Literal;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            values: HashMap::new(),
         }
    }

    pub fn get(&self, name: &str) -> Option<&Literal> {
        self.values.get(name)
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }
}

