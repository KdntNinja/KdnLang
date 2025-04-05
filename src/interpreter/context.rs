use std::collections::HashMap;
use logos::Logos;
use miette::{NamedSource, Report};

use crate::error::{KdnLangError, Result};
use crate::lexer::Token;
use crate::parser::evaluate_expression_with_vars;

/// Helper function to interpret code with a shared variable context
pub fn interpret_with_context(input: &str, variables: &mut HashMap<String, f64>) -> Result<()> {
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
            Ok(_) => {},
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