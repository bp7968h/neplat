use std::fmt;

use crate::{lexer::{Literal, Token}, parser::stmt::Stmt};

use super::{environment::Environment, Interpreter};

pub trait Callable: fmt::Debug {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Option<Literal>;
    fn arity(&self) -> usize;
}

#[derive(Debug)]
pub struct NepLatFunc {
    name: Token,
    params: Vec<Token>,
    body: Vec<Box<Stmt>>,
}

impl NepLatFunc {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Box<Stmt>>) -> Self {
        NepLatFunc { name, params, body }
    }
}

impl fmt::Display for NepLatFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme() )
    }
}

impl Callable for NepLatFunc {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Option<Literal> {
        // Create a new environment with the interpreter's current environment as its enclosing one
        let mut environment = Environment::new_with_env(Box::new(interpreter.get_env()));

        // Define each parameter in the new environment, binding them to the provided arguments
        for (i, param) in self.params.iter().enumerate() {
            if i < arguments.len() {
                environment.define(&param.lexeme(), arguments[i].clone());
            }
        }

        // Execute the function body in the new environment
        interpreter.execute_block(self.body.clone(), environment);
        None
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}