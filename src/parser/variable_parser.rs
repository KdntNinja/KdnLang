use crate::error_handling::errors::KdnLangError;
use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::Result;

pub fn parse_variable(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
    source: &str,
    filename: &str,
) -> Result<(), miette::Error> {
    if let Some(var_name_token) = token_iter.next() {
        if var_name_token.token == Token::Identifier {
            // Parse type annotation
            let mut type_annotation = "any".to_string(); // Default type
            let mut type_span = None; // Store the span for the type annotation

            if let Some(type_token) = token_iter.peek() {
                // Check if token is a colon followed by a type name
                if type_token.token == Token::Colon {
                    let colon_span = type_token.span.clone();
                    token_iter.next(); // Consume the colon
                    if let Some(actual_type_token) = token_iter.next() {
                        if actual_type_token.token == Token::Identifier {
                            // Store the type name directly from our grammar's type_name rule
                            type_annotation = match actual_type_token.lexeme {
                                "int" | "float" | "str" | "bool" | "void" => {
                                    actual_type_token.lexeme.to_string()
                                }
                                _ => actual_type_token.lexeme.to_string(), // Custom type or identifier
                            };
                            // Save the span from the colon to the end of the type
                            type_span = Some((
                                colon_span.start,
                                actual_type_token.span.end - colon_span.start,
                            ));
                        }
                    }
                }
            }

            // Parse equals sign and value
            if let Some(equals_token) = token_iter.next() {
                if equals_token.token == Token::Operator && equals_token.lexeme == "=" {
                    let mut value: ASTNode = ASTNode::Void;

                    // Parse value (could be identifier, literal, or function call)
                    if let Some(value_token) = token_iter.peek() {
                        match value_token.token {
                            Token::StringLiteral => {
                                value = ASTNode::StringLiteral(value_token.lexeme.to_string());
                                token_iter.next();
                            }
                            Token::Number => {
                                value = ASTNode::Number(value_token.lexeme.to_string());
                                token_iter.next();
                            }
                            Token::Identifier => {
                                let function_name = value_token.lexeme.to_string();
                                token_iter.next();

                                // Check if this is a function call
                                if let Some(open_paren) = token_iter.peek() {
                                    if open_paren.token == Token::LeftParen
                                        && open_paren.lexeme == "("
                                    {
                                        token_iter.next(); // Consume opening parenthesis

                                        let mut args = Vec::new();

                                        // Parse arguments
                                        while let Some(arg_token) = token_iter.peek() {
                                            if arg_token.token == Token::RightParen {
                                                token_iter.next(); // Consume closing parenthesis
                                                break;
                                            }

                                            // Parse argument value
                                            if arg_token.token == Token::StringLiteral {
                                                args.push(ASTNode::StringLiteral(
                                                    arg_token.lexeme.to_string(),
                                                ));
                                                token_iter.next();
                                            } else if arg_token.token == Token::Identifier {
                                                args.push(ASTNode::Identifier(
                                                    arg_token.lexeme.to_string(),
                                                ));
                                                token_iter.next();
                                            } else {
                                                // Skip other token types
                                                token_iter.next();
                                            }

                                            // Check for comma
                                            if let Some(comma) = token_iter.peek() {
                                                if comma.token == Token::Comma {
                                                    token_iter.next(); // Consume comma
                                                }
                                            }
                                        }

                                        // Create function call node
                                        value = ASTNode::FunctionCall {
                                            name: function_name,
                                            args,
                                        };
                                    } else {
                                        // If not a function call, just use the identifier
                                        value = ASTNode::Identifier(function_name);
                                    }
                                } else {
                                    // If no opening parenthesis, just use the identifier
                                    value = ASTNode::Identifier(function_name);
                                }
                            }
                            _ => {
                                token_iter.next(); // Skip other token types
                            }
                        }
                    }

                    // Check for required semicolon - make this a hard requirement
                    if let Some(semicolon) = token_iter.peek() {
                        if semicolon.token == Token::Semicolon {
                            token_iter.next(); // Consume semicolon
                        } else {
                            // Return an error if semicolon is missing
                            return Err(KdnLangError::missing_semicolon_error(
                                source.to_string(),
                                filename,
                                (semicolon.span.start, 1).into(),
                                &var_name_token.lexeme,
                            )
                            .into());
                        }
                    } else {
                        // End of input without semicolon
                        return Err(KdnLangError::parser_error(
                            source.to_string(),
                            filename,
                            (0, 0),
                            "Missing semicolon at end of statement",
                            "KdnLang requires semicolons at the end of statements, like Rust syntax.",
                        )
                        .into());
                    }

                    // Create variable assignment node
                    let var_node = ASTNode::Assignment {
                        variable: var_name_token.lexeme.to_string(),
                        type_annotation,
                        value: Box::new(value),
                        type_span, // Include the span of the type annotation
                    };

                    // Add to current scope
                    if let Some(scope) = scope_stack.last_mut() {
                        scope.push(var_node);
                    }

                    return Ok(());
                }
            }
        }
    }

    // Return a parsing error
    Err(KdnLangError::parser_error(
        source.to_string(),
        filename,
        (0, 0),
        "Failed to parse variable assignment",
        "Variable assignments should have format: 'let name: type = value;'",
    )
    .into())
}
