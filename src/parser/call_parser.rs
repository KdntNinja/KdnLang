use crate::error_handling::errors::KdnLangError;
use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::Result;

pub fn parse_function_call(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<(), miette::Error> {
    if let Some(identifier) = token_iter.next() {
        let function_name = identifier.lexeme.to_string();

        if let Some(open_paren) = token_iter.next() {
            if open_paren.token == Token::LeftParen {
                let mut args = Vec::new();

                // Parse arguments until we reach the closing parenthesis
                loop {
                    if let Some(t) = token_iter.peek() {
                        if t.token == Token::RightParen {
                            token_iter.next(); // Consume the closing parenthesis
                            break;
                        }
                    } else {
                        return Err(KdnLangError::parser_error(
                            "".to_string(),
                            "unknown",
                            (0, 0),
                            "Unexpected end of input while parsing function call",
                            format!(
                                "Function call to '{}' is missing a closing parenthesis ')'",
                                function_name
                            ),
                        )
                        .into());
                    }

                    // Parse the argument
                    let mut arg_depth = 0;
                    while let Some(t) = token_iter.peek() {
                        if t.token == Token::LeftParen {
                            arg_depth += 1;
                        } else if t.token == Token::RightParen {
                            if arg_depth == 0 {
                                break;
                            }
                            arg_depth -= 1;
                        } else if t.token == Token::Comma && arg_depth == 0 {
                            break;
                        }

                        // Add argument token
                        match t.token {
                            Token::StringLiteral => {
                                args.push(ASTNode::StringLiteral(t.lexeme.to_string()));
                            }
                            Token::Identifier => {
                                args.push(ASTNode::Identifier(t.lexeme.to_string()));
                            }
                            _ => {}
                        }

                        token_iter.next();
                    }

                    // Check for comma after argument
                    if let Some(t) = token_iter.peek() {
                        if t.token == Token::Comma {
                            token_iter.next(); // Consume the comma
                        }
                    }
                }

                // Create function call node
                let function_call = ASTNode::FunctionCall {
                    name: function_name,
                    args,
                };

                if let Some(scope) = scope_stack.last_mut() {
                    scope.push(function_call);
                }

                // Require a semicolon after function call
                if let Some(token) = token_iter.peek() {
                    if token.token == Token::Semicolon {
                        token_iter.next(); // Consume semicolon
                    } else {
                        // Error for missing semicolon
                        return Err(KdnLangError::parser_error(
                            "".to_string(),
                            "unknown",
                            (0, 0),
                            "Missing semicolon after function call",
                            "KdnLang requires semicolons at the end of statements, like: function_call();",
                        )
                        .into());
                    }
                } else {
                    // End of input without semicolon
                    return Err(KdnLangError::parser_error(
                        "".to_string(),
                        "unknown",
                        (0, 0),
                        "Missing semicolon at end of function call",
                        "KdnLang requires semicolons at the end of statements, like: function_call();",
                    )
                    .into());
                }

                return Ok(());
            }
        }
    }

    Err(KdnLangError::parser_error(
        "".to_string(),
        "unknown",
        (0, 0),
        "Failed to parse function call",
        "Function calls should be in the format: function_name(arg1, arg2, ...)",
    )
    .into())
}

pub fn parse_print(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<(), miette::Error> {
    if let Some(open_paren) = token_iter.next() {
        if open_paren.token == Token::LeftParen {
            let mut args = Vec::new();

            // Parse first argument
            if let Some(arg) = token_iter.next() {
                if arg.token == Token::StringLiteral {
                    args.push(ASTNode::StringLiteral(arg.lexeme.to_string()));
                } else if arg.token == Token::Identifier {
                    args.push(ASTNode::Identifier(arg.lexeme.to_string()));
                }

                // Check for more arguments separated by commas
                while let Some(token) = token_iter.peek() {
                    if token.token == Token::Comma {
                        token_iter.next(); // Consume the comma without using the variable

                        // Parse next argument
                        if let Some(next_arg) = token_iter.next() {
                            if next_arg.token == Token::StringLiteral {
                                args.push(ASTNode::StringLiteral(next_arg.lexeme.to_string()));
                            } else if next_arg.token == Token::Identifier {
                                args.push(ASTNode::Identifier(next_arg.lexeme.to_string()));
                            }
                        }
                    } else {
                        break;
                    }
                }
            }

            // Check for closing parenthesis
            if let Some(close_paren) = token_iter.next() {
                if close_paren.token != Token::RightParen {
                    return Err(KdnLangError::parser_error(
                        "".to_string(),
                        "unknown",
                        (0, 0),
                        "Expected ')' after print arguments",
                        "Print statement should be in the format: print(arg1, arg2, ...)",
                    )
                    .into());
                }
            }

            // Create print node
            let print_node = ASTNode::FunctionCall {
                name: "print".to_string(),
                args,
            };

            if let Some(scope) = scope_stack.last_mut() {
                scope.push(print_node);
            }

            // Require semicolon
            if let Some(token) = token_iter.peek() {
                if token.token == Token::Semicolon {
                    token_iter.next(); // Consume semicolon
                } else {
                    // Error for missing semicolon
                    return Err(KdnLangError::parser_error(
                        "".to_string(),
                        "unknown",
                        (0, 0),
                        "Missing semicolon after print statement",
                        "KdnLang requires semicolons at the end of statements, like: print(\"Hello\");",
                    )
                    .into());
                }
            } else {
                // End of input without semicolon
                return Err(KdnLangError::parser_error(
                    "".to_string(),
                    "unknown",
                    (0, 0),
                    "Missing semicolon at end of print statement",
                    "KdnLang requires semicolons at the end of statements, like: print(\"Hello\");",
                )
                .into());
            }

            return Ok(());
        }
    }

    Err(KdnLangError::parser_error(
        "".to_string(),
        "unknown",
        (0, 0),
        "Failed to parse print statement",
        "Print statement should be in the format: print(\"Hello\") or print(variable)",
    )
    .into())
}
