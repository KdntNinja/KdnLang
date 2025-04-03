use crate::error_handling::errors::KdnLangError;
use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::Result;

pub fn parse_function(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<(), miette::Error> {
    // Parse function name
    if let Some(func_name_token) = token_iter.next() {
        if func_name_token.token == Token::Identifier {
            // Parse parameter list
            if let Some(open_paren) = token_iter.next() {
                if open_paren.token == Token::LeftParen {
                    // Create a vector to store parameters with their types
                    let mut parameters = Vec::new();

                    // Parse parameters according to the grammar's param_list rule
                    while let Some(param_token) = token_iter.peek() {
                        if param_token.token == Token::RightParen {
                            token_iter.next(); // Consume closing paren
                            break;
                        }

                        // Parse parameter name
                        if param_token.token == Token::Identifier {
                            let param_name = param_token.lexeme.to_string();
                            token_iter.next(); // Consume parameter name

                            // Check for type annotation
                            if let Some(colon_token) = token_iter.peek() {
                                if colon_token.token == Token::Colon {
                                    token_iter.next(); // Consume colon

                                    // Get type name
                                    if let Some(type_token) = token_iter.next() {
                                        if type_token.token == Token::Identifier {
                                            let param_type = type_token.lexeme.to_string();
                                            parameters.push((param_name, param_type));
                                        }
                                    }
                                }
                            }

                            // Check for comma
                            if let Some(comma_token) = token_iter.peek() {
                                if comma_token.token == Token::Comma {
                                    token_iter.next(); // Consume comma
                                }
                            }
                        } else {
                            // Unexpected token, skip it
                            token_iter.next();
                        }
                    }

                    // Check for optional return type
                    let mut return_type = "void".to_string();
                    if let Some(token) = token_iter.peek() {
                        if token.token == Token::Colon {
                            token_iter.next(); // Consume colon

                            if let Some(type_token) = token_iter.next() {
                                if type_token.token == Token::Identifier {
                                    return_type = type_token.lexeme.to_string();
                                }
                            }
                        }
                    }

                    // Parse function body
                    if let Some(open_brace) = token_iter.next() {
                        if open_brace.token == Token::LeftBrace && open_brace.lexeme == "{" {
                            // Create a new scope for the function body
                            scope_stack.push(Vec::new());

                            // Parse statements inside function body
                            while let Some(body_token) = token_iter.peek() {
                                if body_token.token == Token::RightBrace && body_token.lexeme == "}"
                                {
                                    token_iter.next(); // Consume closing brace
                                    break;
                                }
                                token_iter.next(); // Skip body tokens
                            }

                            // Pop the function body scope
                            if let Some(function_body) = scope_stack.pop() {
                                // Create a function node
                                let function_node = ASTNode::Function {
                                    name: func_name_token.lexeme.to_string(),
                                    parameters,
                                    return_type,
                                    body: function_body,
                                };

                                // Add the function to the current scope
                                if let Some(current_scope) = scope_stack.last_mut() {
                                    current_scope.push(function_node);
                                }

                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    }

    Err(KdnLangError::parser_error(
        "".to_string(),
        "unknown",
        (0, 0),
        "Failed to parse function",
        "Functions should be defined like: fn name(params) { body }",
    )
    .into())
}
