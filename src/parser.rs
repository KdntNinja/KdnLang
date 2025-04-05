use crate::error::{KdnLangError, Result};
use miette::{NamedSource, Report};
use std::collections::HashMap;

/// Evaluates a mathematical expression with variables
pub fn evaluate_expression_with_vars(expr: &str, variables: &HashMap<String, f64>) -> Result<f64> {
    // Simple tokenizer for basic expressions
    let mut result = 0.0;
    let mut current_number = String::new();
    let mut current_op = '+';
    let mut chars = expr.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' | '.' => {
                current_number.push(c);
            }
            '+' | '-' | '*' | '/' => {
                if !current_number.is_empty() {
                    let num = current_number.parse::<f64>().unwrap_or(0.0);
                    current_number.clear();

                    match current_op {
                        '+' => result += num,
                        '-' => result -= num,
                        '*' => result *= num,
                        '/' => result /= num,
                        _ => {}
                    }
                }
                current_op = c;
            }
            ' ' | '\t' => {} // Skip whitespace
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut var_name = String::new();
                var_name.push(c);

                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if let Some(val) = variables.get(&var_name) {
                    match current_op {
                        '+' => result += val,
                        '-' => result -= val,
                        '*' => result *= val,
                        '/' => result /= val,
                        _ => {}
                    }
                } else {
                    return Err(Report::new(KdnLangError {
                        src: NamedSource::new("expression", expr.to_string()),
                        span: (
                            expr.find(&var_name).unwrap_or(0),
                            expr.find(&var_name).unwrap_or(0) + var_name.len(),
                        )
                            .into(),
                        help: Some(format!("Undefined variable: {}", var_name)),
                    }));
                }
            }
            _ => {
                return Err(Report::new(KdnLangError {
                    src: NamedSource::new("expression", expr.to_string()),
                    span: (expr.find(c).unwrap_or(0), expr.find(c).unwrap_or(0) + 1).into(),
                    help: Some(format!("Unexpected character in expression: {}", c)),
                }));
            }
        }
    }

    // Process the last number if any
    if !current_number.is_empty() {
        let num = current_number.parse::<f64>().unwrap_or(0.0);
        match current_op {
            '+' => result += num,
            '-' => result -= num,
            '*' => result *= num,
            '/' => result /= num,
            _ => {}
        }
    }

    Ok(result)
}
