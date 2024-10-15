use std::collections::HashMap;

use crate::lexer::{Literal, Token};

use super::interpret_error::InterpretError;

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

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), InterpretError> {
        let var_name = name.lexeme().to_string();

        if self.values.contains_key(&var_name) {
            self.values.insert(var_name, value);

            return Ok(());
        }

        Err(InterpretError::UndefinedVariable(format!("{}", var_name)))

    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }
}

