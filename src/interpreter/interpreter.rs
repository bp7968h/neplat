use crate::{
    lexer::{Literal, TokenType},
    parser::{expr::Expr, stmt::Stmt, visitor::{ExprVisitor, StmtVisitor}},
};

use super::{environment::Environment, interpret_error::InterpretError};

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

    fn evaluate(&mut self, expr: &Expr) -> Option<Literal> {
        expr.accept(self)
    }

    pub fn get_variable(&self, name: &str) -> Option<&Literal> {
        self.environment.get(name)
    }

    pub fn get_errors(&self) -> &[InterpretError] {
        &self.errors
    }

    fn report_error(&mut self, error: InterpretError) {
        self.errors.push(error);
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
                    },
                    // Handle string concatenation with +
                    (Literal::StringLiteral(left_str), Literal::StringLiteral(right_str)) => {
                        Some(Literal::StringLiteral(left_str + &right_str))
                    },
                    // Handle string + number or number + string
                    (Literal::StringLiteral(left_str), Literal::NumberLiteral(right_num)) => {
                        Some(Literal::StringLiteral(left_str + &right_num.to_string()))
                    },
                    (Literal::NumberLiteral(left_num), Literal::StringLiteral(right_str)) => {
                        Some(Literal::StringLiteral(left_num.to_string() + &right_str))
                    },
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

    fn vist_variable_expr(&mut self, expr: &Expr) -> Option<Literal> {
        if let Expr::Variable(token) = expr {
            if let Some(value) = self.environment.get(token.lexeme()) {
                return Some(value.clone());
            } else {
                // Handle error for undefined variable
                self.report_error(InterpretError::UndefinedVariable(token.lexeme().to_string()));
                return None;
            }
        }

        None
    }
}

impl StmtVisitor<()> for Interpreter {
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
}