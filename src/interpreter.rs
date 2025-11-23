use crate::error::*;
use crate::expr::*;
use crate::object::*;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        // as_res: Option<T> -> Option<&T>
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        Ok(Object::Nil)
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match right {
                Object::Num(n) => return Ok(Object::Num(-n)),
                _ => return Ok(Object::Nil),
            },

            TokenType::Bang => {
                if self.is_truthy(&right) {
                    Ok(Object::False)
                } else {
                    Ok(Object::True)
                }
            }
            _ => Err(LoxError::error(
                0,
                "Unreachable according to author.".to_string(),
            )),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        // uncommented code does this
        // match object {
        //     Object::False | Object::Nil => false,
        //     _ => true,
        // }
        !matches!(object, Object::Nil | Object::False)
    }
}
