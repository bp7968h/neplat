use std::rc::Rc;

use crate::{
    lexer::{Literal, TokenType},
    parser::{
        expr::Expr,
        stmt::Stmt,
        visitor::{ExprVisitor, StmtVisitor},
    },
};

use super::{callable::NepLatFunc, environment::Environment, interpret_error::InterpretError};

pub struct Interpreter {
    errors: Vec<InterpretError>,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            errors: Vec::new(),
            environment: Environment::new(),
        }
    }

    pub fn get_env(&self) -> Environment {
        self.environment.clone()
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), &Vec<InterpretError>> {
        for stmt in statements {
            self.execute(stmt);
        }

        if !self.errors.is_empty() {
            Err(&self.errors)
        } else {
            Ok(())
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    pub fn execute_block(&mut self, statements: Vec<Box<Stmt>>, new_env: Environment) {
        let _ = std::mem::replace(&mut self.environment, new_env);
        for stmt in statements {
            self.execute(&stmt);
        }

        if let Some(enclosed) = self.environment.enclosing.clone() {
            self.environment = *enclosed;
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Option<Literal> {
        expr.accept(self)
    }

    pub fn get_variable(&self, name: &str) -> Option<&Literal> {
        match self.environment.get(name) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    pub fn get_errors(&self) -> &[InterpretError] {
        &self.errors
    }

    fn report_error(&mut self, error: InterpretError) {
        self.errors.push(error);
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::BooleanLiteral(b) => *b,
            Literal::NullLiteral => false,
            _ => true,
        }
    }
}

impl ExprVisitor<Option<Literal>> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Literal(value) = expr {
            Some(value.clone())
        } else {
            None
        }
    }

    fn visit_grouping_expression(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Grouping(inner) = expr {
            self.evaluate(inner)
        } else {
            None
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Unary(operator, operand) = expr {
            let operand_value = self.evaluate(operand)?;

            match (operator.token_type(), operand_value) {
                (TokenType::MINUS, Literal::NumberLiteral(value)) => {
                    Some(Literal::NumberLiteral(-value))
                }
                (TokenType::BANG, Literal::BooleanLiteral(value)) => {
                    Some(Literal::BooleanLiteral(!value))
                }
                _ => {
                    self.report_error(InterpretError::TypeMismatch(
                        "Invalid operand for unary operator between".to_string(),
                    ));
                    None
                }
            }
        } else {
            None
        }
    }

    fn visit_binary_expression(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Binary(left, operator, right) = expr {
            let left_value = self.evaluate(left)?;
            let right_value = self.evaluate(right)?;

            match operator.token_type() {
                // Handle +
                TokenType::PLUS => match (left_value, right_value) {
                    // Handle numeric addition
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::NumberLiteral(left_num + right_num))
                    }
                    // Handle string concatenation with +
                    (Literal::StringLiteral(left_str), Literal::StringLiteral(right_str)) => {
                        Some(Literal::StringLiteral(left_str + &right_str))
                    }
                    // Handle string + number or number + string
                    (Literal::StringLiteral(left_str), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::StringLiteral(left_str + &right_num.to_string()))
                    }
                    (Literal::NumberLiteral(left_num), Literal::StringLiteral(right_str)) => {
                        Some(Literal::StringLiteral(left_num.to_string() + &right_str))
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in addition".to_string(),
                        ));
                        None
                    }
                },

                // Handle -
                TokenType::MINUS => match (left_value, right_value) {
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::NumberLiteral(left_num - right_num))
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in subtraction".to_string(),
                        ));
                        None
                    }
                },

                // Handle *
                TokenType::STAR => match (left_value, right_value) {
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::NumberLiteral(left_num * right_num))
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in multiplication".to_string(),
                        ));
                        None
                    }
                },

                // Handle /
                TokenType::SLASH => match (left_value, right_value) {
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        if right_num == 0.0 {
                            // Handle division by zero case
                            self.report_error(InterpretError::DivisionByZero);
                            None
                        } else {
                            Some(Literal::NumberLiteral(left_num / right_num))
                        }
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in division".to_string(),
                        ));
                        None
                    }
                },

                // Handle >
                TokenType::GREATER => match (left_value, right_value) {
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::BooleanLiteral(left_num > right_num))
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in comparison".to_string(),
                        ));
                        None
                    }
                },

                // Handle >=
                TokenType::GREATEREQUAL => match (left_value, right_value) {
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::BooleanLiteral(left_num >= right_num))
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in comparison".to_string(),
                        ));
                        None
                    }
                },

                // Handle <
                TokenType::LESS => match (left_value, right_value) {
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::BooleanLiteral(left_num < right_num))
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in comparison".to_string(),
                        ));
                        None
                    }
                },

                // Handle <=
                TokenType::LESSEQUAL => match (left_value, right_value) {
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::BooleanLiteral(left_num <= right_num))
                    }
                    _ => {
                        self.report_error(InterpretError::TypeMismatch(
                            "Type mismatch in comparison".to_string(),
                        ));
                        None
                    }
                },

                // Handle ==
                TokenType::EQUALEQUAL => match (left_value, right_value) {
                    // Case: Number equality check
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::BooleanLiteral(left_num == right_num))
                    }

                    // Case: String equality check
                    (Literal::StringLiteral(left_str), Literal::StringLiteral(right_str)) => {
                        Some(Literal::BooleanLiteral(left_str == right_str))
                    }

                    // Case: Both are null (null equality)
                    (Literal::NullLiteral, Literal::NullLiteral) => {
                        Some(Literal::BooleanLiteral(true))
                    }

                    //Case: Both are boolean
                    (Literal::BooleanLiteral(left), Literal::BooleanLiteral(right)) => {
                        Some(Literal::BooleanLiteral(left == right))
                    }

                    // For type mismatches or unsupported types
                    _ => Some(Literal::BooleanLiteral(false)),
                },

                // Handle !=
                TokenType::BANGEQUAL => match (left_value, right_value) {
                    // Case: Number equality check
                    (Literal::NumberLiteral(left_num), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::BooleanLiteral(left_num != right_num))
                    }

                    // Case: String equality check
                    (Literal::StringLiteral(left_str), Literal::StringLiteral(right_str)) => {
                        Some(Literal::BooleanLiteral(left_str != right_str))
                    }

                    // Case: Both are null (null equality)
                    (Literal::NullLiteral, Literal::NullLiteral) => {
                        Some(Literal::BooleanLiteral(false))
                    }

                    //Case: Both are boolean
                    (Literal::BooleanLiteral(left), Literal::BooleanLiteral(right)) => {
                        Some(Literal::BooleanLiteral(left != right))
                    }

                    // For type mismatches or unsupported types
                    _ => Some(Literal::BooleanLiteral(true)),
                },

                _ => None,
            }
        } else {
            None
        }
    }

    fn visit_assign_expression(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Assign(token, value_expr) = expr {
            let value = self.evaluate(value_expr)?;

            match self.environment.assign(token, value.clone()) {
                Ok(_) => {
                    return Some(value);
                },
                Err(error) => {
                    self.report_error(error);
                    return None;
                }
            }
        } else {
            None
        }
    }

    fn vist_variable_expr(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Variable(token) = expr {
            let token_name = token.lexeme();
            match self.environment.get(token_name) {
                Ok(value) => {
                    match value {
                        &Literal::NullLiteral => {
                            self.report_error(InterpretError::UnassignmedVariable(format!(
                                "Variable {} is not assigned",
                                token_name
                            )));
                            return None
                        }
                        _ =>  {
                            return Some(value.clone())
                        },
                    }
                    // return Some(value.clone())
                }
                Err(e) => {
                    self.report_error(e);
                    return None;
                }
            }
        }
        None
    }

    fn visit_logical_expression(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Logical(left_expr, operator, right_expr) = expr {
            let left_value = self.evaluate(left_expr)?;

            match operator.token_type() {
                TokenType::OR => {
                    if self.is_truthy(&left_value) {
                        return Some(left_value);
                    }
                },
                TokenType::AND => {
                    if !self.is_truthy(&left_value) {
                        return Some(left_value);
                    }
                }
                _ => {}
            }
            let right_value = self.evaluate(right_expr)?;
            return Some(right_value);
        }

        None
    }

    fn visit_call_expression(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Call(callee, _paren, args) = expr {
            let caller = self.evaluate(callee)?;

            let mut func_args: Vec<Literal> = Vec::new();
            for argument in args {
                if let Some(arg) = self.evaluate(argument) {
                    func_args.push(arg);
                } else {
                    return None;
                }
            }

            if let Some(callable) = caller.as_callable() {
                // Verify argument count matches the arity of the function
                if func_args.len() != callable.arity() {
                    self.report_error(InterpretError::ArgumentMismatch(
                        format!(
                            "Expected {} arguments but got {}.",
                            callable.arity(),
                            func_args.len()
                        ),
                    ));
                    return None;
                }

                return callable.call(self, func_args)
            } else {
                self.report_error(InterpretError::TypeMismatch(
                    "Can only call functions and classes.".to_string(),
                ));
                return None;
            }
        }

        None
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Block(stmt_list) = stmt {
            let new_env = Environment::new_with_env(Box::new(self.environment.clone()));
            self.execute_block(stmt_list.clone(), new_env);
        }
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Expression(expr) = stmt {
            self.evaluate(expr);
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Print(expr) = stmt {
            if let Some(value) = self.evaluate(expr) {
                println!("{}", value);
            }
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Var(token, initializer) = stmt {
            let value = if let Some(expr) = initializer {
                self.evaluate(expr)
            } else {
                Some(Literal::NullLiteral)
            };

            if let Some(val) = value {
                self.environment.define(token.lexeme(), val);
            }
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::If(condition, then_branch, else_branch) = stmt {
            if let Some(Literal::BooleanLiteral(true)) = self.evaluate(condition) {
                self.execute(then_branch);
            } else if let Some(else_branch) = else_branch {
                self.execute(else_branch);
            }
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::While(condition, body) = stmt {
                while let Some(cond_value) = self.evaluate(condition) {
                    if !self.is_truthy(&cond_value) {
                        break;
                    }

                    self.execute(body);
                }
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Function(name, params, body) = stmt {
            let function = NepLatFunc::new(name.clone(), params.clone(), body.clone());
            let function_literal = Literal::Callable(Rc::new(function));

            self.environment.define(name.lexeme(), function_literal);
        }
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Return(_keyword, _value) = stmt {
            todo!()
        }
    }
}
