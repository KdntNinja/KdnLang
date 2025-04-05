use std::collections::HashMap;
use miette::{NamedSource, Report};

use crate::error::{KdnLangError, Result};
use crate::parser::{parse_program, AstNode};

/// Helper function to interpret code with a shared variable context
pub fn interpret_with_context(input: &str, variables: &mut HashMap<String, f64>) -> Result<()> {
    // Parse the input code
    let ast = parse_program(input)?;
    
    // Evaluate the parsed AST with the provided context
    match ast {
        AstNode::Program(statements) => {
            for statement in statements {
                evaluate_statement(&statement, variables)?;
            }
        },
        _ => return Err(Report::new(KdnLangError {
            src: NamedSource::new("context", input.to_string()),
            span: (0, 1).into(),
            help: Some("Expected a program".to_string()),
        })),
    }
    
    Ok(())
}

/// Evaluate a statement node with the given context
fn evaluate_statement(node: &AstNode, variables: &mut HashMap<String, f64>) -> Result<Option<f64>> {
    match node {
        AstNode::LetStatement { name, value, .. } => {
            let value_result = evaluate_expression(value, variables)?;
            variables.insert(name.clone(), value_result);
            Ok(None)
        },
        AstNode::Assignment { name, value } => {
            let value_result = evaluate_expression(value, variables)?;
            
            // Check if variable exists before assignment
            if !variables.contains_key(name) {
                return Err(Report::new(KdnLangError {
                    src: NamedSource::new("context", name.clone()),
                    span: (0, name.len()).into(),
                    help: Some(format!("Variable '{}' not declared", name)),
                }));
            }
            
            variables.insert(name.clone(), value_result);
            Ok(None)
        },
        AstNode::PrintStatement { expression } => {
            let value = evaluate_expression(expression, variables)?;
            println!("{}", value);
            Ok(None)
        },
        AstNode::ForLoop { variable, range_start, range_end, body } => {
            let start_value = evaluate_expression(range_start, variables)? as i64;
            let end_value = evaluate_expression(range_end, variables)? as i64;
            
            for i in start_value..end_value {
                // Create a new scope for loop variables
                let mut loop_variables = variables.clone();
                loop_variables.insert(variable.clone(), i as f64);
                
                // Execute the loop body
                for statement in body {
                    evaluate_statement(statement, &mut loop_variables)?;
                }
                
                // Copy modified variables back to parent scope (except loop variable)
                for (key, value) in loop_variables.iter() {
                    if key != variable {
                        variables.insert(key.clone(), *value);
                    }
                }
            }
            
            Ok(None)
        },
        _ => Err(Report::new(KdnLangError {
            src: NamedSource::new("context", "".to_string()),
            span: (0, 1).into(),
            help: Some("Invalid statement type".to_string()),
        })),
    }
}

/// Evaluate an expression in the AST
fn evaluate_expression(node: &AstNode, variables: &HashMap<String, f64>) -> Result<f64> {
    match node {
        AstNode::BinaryOp { op, left, right } => {
            let left_value = evaluate_expression(left, variables)?;
            let right_value = evaluate_expression(right, variables)?;
            
            let result = match op.as_str() {
                "+" => left_value + right_value,
                "-" => left_value - right_value,
                "*" => left_value * right_value,
                "/" => {
                    if right_value == 0.0 {
                        return Err(Report::new(KdnLangError {
                            src: NamedSource::new("context", "".to_string()),
                            span: (0, 1).into(),
                            help: Some("Division by zero".to_string()),
                        }));
                    }
                    left_value / right_value
                },
                _ => {
                    return Err(Report::new(KdnLangError {
                        src: NamedSource::new("context", op.clone()),
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
                    src: NamedSource::new("context", name.clone()),
                    span: (0, name.len()).into(),
                    help: Some(format!("Undefined variable: {}", name)),
                }))
            }
        },
        _ => Err(Report::new(KdnLangError {
            src: NamedSource::new("context", "".to_string()),
            span: (0, 1).into(),
            help: Some("Invalid expression".to_string()),
        })),
    }
}