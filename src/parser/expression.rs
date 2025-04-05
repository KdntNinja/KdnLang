// src/parser/expression.rs - Expression evaluation
use std::collections::HashMap;
use miette::{NamedSource, Report};
use crate::error::{KdnLangError, Result};
use crate::parser::AstNode;

/// Evaluate an AST node expression
pub fn evaluate_ast_expression(node: &AstNode, variables: &HashMap<String, f64>) -> Result<f64> {
    match node {
        AstNode::BinaryOp { op, left, right } => {
            let left_value = evaluate_ast_expression(left, variables)?;
            let right_value = evaluate_ast_expression(right, variables)?;
            
            let result = match op.as_str() {
                "+" => left_value + right_value,
                "-" => left_value - right_value,
                "*" => left_value * right_value,
                "/" => {
                    if right_value == 0.0 {
                        return Err(Report::new(KdnLangError {
                            src: NamedSource::new("expression", format!("{} / {}", left_value, right_value)),
                            span: (0, 1).into(),
                            help: Some("Division by zero".to_string()),
                        }));
                    }
                    left_value / right_value
                },
                _ => {
                    return Err(Report::new(KdnLangError {
                        src: NamedSource::new("expression", op.clone()),
                        span: (0, op.len()).into(),
                        help: Some(format!("Unknown operator: {}", op)),
                    }));
                },
            };
            
            Ok(result)
        },
        AstNode::Number(value) => Ok(*value),
        AstNode::Identifier(name) => {
            if let Some(value) = variables.get(name) {
                Ok(*value)
            } else {
                Err(Report::new(KdnLangError {
                    src: NamedSource::new("expression", name.clone()),
                    span: (0, name.len()).into(),
                    help: Some(format!("Undefined variable: {}", name)),
                }))
            }
        },
        _ => Err(Report::new(KdnLangError {
            src: NamedSource::new("expression", "".to_string()),
            span: (0, 1).into(),
            help: Some("Invalid expression node type".to_string()),
        })),
    }
}