use std::collections::HashMap;

use crate::lexer::{Literal, Token};
use super::interpret_error::InterpretError;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            values: HashMap::new(),
            enclosing: None,
         }
    }

    pub fn new_with_env(enclosing: Box<Environment>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn get(&self, name: &str) -> Result<&Literal, InterpretError> {
        if let Some(value) = self.values.get(name) {
            return Ok(value);
        } else if let Some(ref enclosing_env) = self.enclosing {
            return enclosing_env.get(name);
        }

        Err(InterpretError::UndefinedVariable(format!(
            "Undefined variable '{}'", name
        )))
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), InterpretError> {
        let var_name = name.lexeme().to_string();

        if self.values.contains_key(&var_name) {
            self.values.insert(var_name, value);
            return Ok(());
        } else if let Some(ref mut enclosing_env) = self.enclosing {
            return enclosing_env.assign(name, value);
        }

        Err(InterpretError::UndefinedVariable(format!(
            "Undefined variable '{}'", var_name
        )))
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }
}

