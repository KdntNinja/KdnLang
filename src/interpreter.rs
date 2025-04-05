// src/interpreter.rs - Code execution for KdnLang
use logos::Logos;
use miette::{NamedSource, Report};
use std::collections::HashMap;

use crate::error::{KdnLangError, Result};
use crate::lexer::Token;
use crate::parser::evaluate_expression_with_vars;

/// Main interpret function that processes KdnLang code
pub fn interpret(input: &str) -> Result<()> {
    let mut lexer = Token::lexer(input);
    let mut variables: HashMap<String, f64> = HashMap::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(Token::Let) => {
                if let Some(Ok(Token::Identifier)) = lexer.next() {
                    let var_name = lexer.slice().to_string();
                    if let Some(Ok(Token::Equals)) = lexer.next() {
                        let mut expr = String::new();
                        while let Some(inner_result) = lexer.next() {
                            match inner_result {
                                Ok(Token::Semicolon) => break,
                                Ok(_) => expr.push_str(lexer.slice()),
                                Err(_) => {
                                    return Err(Report::new(KdnLangError {
                                        src: NamedSource::new("input", input.to_string()),
                                        span: (lexer.span().start, lexer.span().end).into(),
                                        help: Some(
                                            "Unexpected token while parsing expression".to_string(),
                                        ),
                                    }));
                                }
                            }
                        }
                        let value = evaluate_expression_with_vars(&expr, &variables)?;
                        variables.insert(var_name, value);
                    } else {
                        return Err(Report::new(KdnLangError {
                            src: NamedSource::new("input", input.to_string()),
                            span: (lexer.span().start, lexer.span().end).into(),
                            help: Some("Expected '=' after variable name".to_string()),
                        }));
                    }
                }
            }
            Ok(Token::Identifier) => {
                let var_name = lexer.slice().to_string();
                if let Some(Ok(Token::Equals)) = lexer.next() {
                    let mut expr = String::new();
                    while let Some(inner_result) = lexer.next() {
                        match inner_result {
                            Ok(Token::Semicolon) => break,
                            Ok(_) => expr.push_str(lexer.slice()),
                            Err(_) => {
                                return Err(Report::new(KdnLangError {
                                    src: NamedSource::new("input", input.to_string()),
                                    span: (lexer.span().start, lexer.span().end).into(),
                                    help: Some(
                                        "Unexpected token while parsing expression".to_string(),
                                    ),
                                }));
                            }
                        }
                    }
                    let value = evaluate_expression_with_vars(&expr, &variables)?;

                    // Check if variable exists before assignment
                    if !variables.contains_key(&var_name) {
                        return Err(Report::new(KdnLangError {
                            src: NamedSource::new("input", input.to_string()),
                            span: (lexer.span().start, lexer.span().end).into(),
                            help: Some(format!("Variable '{}' not declared", var_name)),
                        }));
                    }

                    variables.insert(var_name, value);
                } else {
                    return Err(Report::new(KdnLangError {
                        src: NamedSource::new("input", input.to_string()),
                        span: (lexer.span().start, lexer.span().end).into(),
                        help: Some("Expected '=' after variable name".to_string()),
                    }));
                }
            }
            Ok(Token::Print) => {
                if let Some(Ok(Token::LParen)) = lexer.next() {
                    let mut expr = String::new();
                    while let Some(inner_result) = lexer.next() {
                        match inner_result {
                            Ok(Token::RParen) => break,
                            Ok(_) => expr.push_str(lexer.slice()),
                            Err(_) => {
                                return Err(Report::new(KdnLangError {
                                    src: NamedSource::new("input", input.to_string()),
                                    span: (lexer.span().start, lexer.span().end).into(),
                                    help: Some(
                                        "Unexpected token while parsing expression".to_string(),
                                    ),
                                }));
                            }
                        }
                    }
                    let value = evaluate_expression_with_vars(&expr, &variables)?;
                    println!("{}", value);
                } else {
                    return Err(Report::new(KdnLangError {
                        src: NamedSource::new("input", input.to_string()),
                        span: (lexer.span().start, lexer.span().end).into(),
                        help: Some("Expected '(' after 'print'".to_string()),
                    }));
                }
            }
            Ok(Token::For) => {
                if let Some(Ok(Token::Identifier)) = lexer.next() {
                    let loop_var = lexer.slice().to_string();
                    if let Some(Ok(Token::In)) = lexer.next() {
                        let mut range_start = String::new();
                        while let Some(inner_result) = lexer.next() {
                            match inner_result {
                                Ok(Token::Range) => break,
                                Ok(_) => range_start.push_str(lexer.slice()),
                                Err(_) => {
                                    return Err(Report::new(KdnLangError {
                                        src: NamedSource::new("input", input.to_string()),
                                        span: (lexer.span().start, lexer.span().end).into(),
                                        help: Some(
                                            "Unexpected token while parsing range start"
                                                .to_string(),
                                        ),
                                    }));
                                }
                            }
                        }

                        let mut range_end = String::new();
                        while let Some(inner_result) = lexer.next() {
                            match inner_result {
                                Ok(Token::LBrace) => break,
                                Ok(_) => range_end.push_str(lexer.slice()),
                                Err(_) => {
                                    return Err(Report::new(KdnLangError {
                                        src: NamedSource::new("input", input.to_string()),
                                        span: (lexer.span().start, lexer.span().end).into(),
                                        help: Some(
                                            "Unexpected token while parsing range end".to_string(),
                                        ),
                                    }));
                                }
                            }
                        }

                        let start = evaluate_expression_with_vars(&range_start, &variables)? as i64;
                        let end = evaluate_expression_with_vars(&range_end, &variables)? as i64;

                        // Read the entire block content
                        let mut block_content = String::new();
                        let mut brace_level = 1;

                        while let Some(inner_result) = lexer.next() {
                            match inner_result {
                                Ok(Token::LBrace) => {
                                    brace_level += 1;
                                    block_content.push_str(lexer.slice());
                                }
                                Ok(Token::RBrace) => {
                                    brace_level -= 1;
                                    if brace_level == 0 {
                                        break;
                                    }
                                    block_content.push_str(lexer.slice());
                                }
                                Ok(_) => block_content.push_str(lexer.slice()),
                                Err(_) => {
                                    return Err(Report::new(KdnLangError {
                                        src: NamedSource::new("input", input.to_string()),
                                        span: (lexer.span().start, lexer.span().end).into(),
                                        help: Some("Unexpected token in loop body".to_string()),
                                    }));
                                }
                            }
                        }

                        // Execute the loop
                        for i in start..end {
                            // Update the loop variable for each iteration
                            let mut loop_variables = variables.clone();
                            loop_variables.insert(loop_var.clone(), i as f64);

                            // Execute the block with the current context
                            interpret_with_context(&block_content, &mut loop_variables)?;

                            // Update any variables that might have changed in the loop
                            for (key, value) in loop_variables.iter() {
                                if key != &loop_var {
                                    variables.insert(key.clone(), *value);
                                }
                            }
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(_) => {
                return Err(Report::new(KdnLangError {
                    src: NamedSource::new("input", input.to_string()),
                    span: (lexer.span().start, lexer.span().end).into(),
                    help: Some("Unexpected token".to_string()),
                }));
            }
        }
    }
    Ok(())
}

/// Helper function to interpret code with a shared variable context
fn interpret_with_context(input: &str, variables: &mut HashMap<String, f64>) -> Result<()> {
    let mut lexer = Token::lexer(input);

    while let Some(result) = lexer.next() {
        match result {
            Ok(Token::Let) => {
                if let Some(Ok(Token::Identifier)) = lexer.next() {
                    let var_name = lexer.slice().to_string();
                    if let Some(Ok(Token::Equals)) = lexer.next() {
                        let mut expr = String::new();
                        while let Some(inner_result) = lexer.next() {
                            match inner_result {
                                Ok(Token::Semicolon) => break,
                                Ok(_) => expr.push_str(lexer.slice()),
                                Err(_) => {
                                    return Err(Report::new(KdnLangError {
                                        src: NamedSource::new("input", input.to_string()),
                                        span: (lexer.span().start, lexer.span().end).into(),
                                        help: Some(
                                            "Unexpected token while parsing expression".to_string(),
                                        ),
                                    }));
                                }
                            }
                        }
                        let value = evaluate_expression_with_vars(&expr, &variables)?;
                        variables.insert(var_name, value);
                    } else {
                        return Err(Report::new(KdnLangError {
                            src: NamedSource::new("input", input.to_string()),
                            span: (lexer.span().start, lexer.span().end).into(),
                            help: Some("Expected '=' after variable name".to_string()),
                        }));
                    }
                }
            }
            Ok(Token::Identifier) => {
                let var_name = lexer.slice().to_string();
                if let Some(Ok(Token::Equals)) = lexer.next() {
                    let mut expr = String::new();
                    while let Some(inner_result) = lexer.next() {
                        match inner_result {
                            Ok(Token::Semicolon) => break,
                            Ok(_) => expr.push_str(lexer.slice()),
                            Err(_) => {
                                return Err(Report::new(KdnLangError {
                                    src: NamedSource::new("input", input.to_string()),
                                    span: (lexer.span().start, lexer.span().end).into(),
                                    help: Some(
                                        "Unexpected token while parsing expression".to_string(),
                                    ),
                                }));
                            }
                        }
                    }
                    let value = evaluate_expression_with_vars(&expr, &variables)?;

                    // Check if variable exists before assignment
                    if !variables.contains_key(&var_name) {
                        return Err(Report::new(KdnLangError {
                            src: NamedSource::new("input", input.to_string()),
                            span: (lexer.span().start, lexer.span().end).into(),
                            help: Some(format!("Variable '{}' not declared", var_name)),
                        }));
                    }

                    variables.insert(var_name, value);
                } else {
                    return Err(Report::new(KdnLangError {
                        src: NamedSource::new("input", input.to_string()),
                        span: (lexer.span().start, lexer.span().end).into(),
                        help: Some("Expected '=' after variable name".to_string()),
                    }));
                }
            }
            Ok(Token::Print) => {
                if let Some(Ok(Token::LParen)) = lexer.next() {
                    let mut expr = String::new();
                    while let Some(inner_result) = lexer.next() {
                        match inner_result {
                            Ok(Token::RParen) => break,
                            Ok(_) => expr.push_str(lexer.slice()),
                            Err(_) => {
                                return Err(Report::new(KdnLangError {
                                    src: NamedSource::new("input", input.to_string()),
                                    span: (lexer.span().start, lexer.span().end).into(),
                                    help: Some(
                                        "Unexpected token while parsing expression".to_string(),
                                    ),
                                }));
                            }
                        }
                    }
                    let value = evaluate_expression_with_vars(&expr, &variables)?;
                    println!("{}", value);
                } else {
                    return Err(Report::new(KdnLangError {
                        src: NamedSource::new("input", input.to_string()),
                        span: (lexer.span().start, lexer.span().end).into(),
                        help: Some("Expected '(' after 'print'".to_string()),
                    }));
                }
            }
            Ok(Token::For) => {
                // Nested loops - simplified for this version
                // Can be expanded in the future for more complex cases
            }
            Ok(_) => {}
            Err(_) => {
                return Err(Report::new(KdnLangError {
                    src: NamedSource::new("input", input.to_string()),
                    span: (lexer.span().start, lexer.span().end).into(),
                    help: Some("Unexpected token".to_string()),
                }));
            }
        }
    }
    Ok(())
}
