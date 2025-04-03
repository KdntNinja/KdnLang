use crate::error_handling::errors::KdnLangError;
use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::Result;

pub fn parse_if_statement(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<(), miette::Error> {
    // Parse condition (inside parentheses)
    if let Some(token) = token_iter.next() {
        if token.token != Token::LeftParen {
            return Err(KdnLangError::parser_error(
                "".to_string(), // Empty source since we don't have it here
                "unknown",
                (token.span.start, token.span.end - token.span.start),
                "Expected '(' after 'if'",
                "The if statement condition should be wrapped in parentheses, like: if (condition) { ... }",
            ).into());
        }
    }

    // Parse the condition expression
    let mut condition_parts = Vec::new();
    let mut depth = 1;

    while let Some(token) = token_iter.next() {
        match token.token {
            Token::LeftParen => depth += 1,
            Token::RightParen => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }

        // Collect the condition parts (simplified for now, just identifiers and operators)
        match token.token {
            Token::Identifier => {
                condition_parts.push(ASTNode::Identifier(token.lexeme.to_string()));
            }
            Token::BoolLiteral => {
                let value = token.lexeme == "true";
                condition_parts.push(ASTNode::BooleanLiteral(value));
            }
            Token::StringLiteral => {
                condition_parts.push(ASTNode::StringLiteral(token.lexeme.to_string()));
            }
            Token::Number => {
                condition_parts.push(ASTNode::Number(token.lexeme.to_string()));
            }
            Token::Operator if token.lexeme == "=" => {
                // For now, we'll handle basic equality comparison
                if let Some(next_token) = token_iter.next() {
                    if next_token.token == Token::Operator && next_token.lexeme == "=" {
                        condition_parts.push(ASTNode::Operator("==".to_string()));
                    }
                }
            }
            Token::Operator => {
                condition_parts.push(ASTNode::Operator(token.lexeme.to_string()));
            }
            _ => {}
        }
    }

    // Build the condition (simple for now, assuming binary expression)
    let condition = if condition_parts.len() >= 3 {
        let left = Box::new(condition_parts[0].clone());
        let operator = match &condition_parts[1] {
            ASTNode::Operator(op) => op.clone(),
            _ => "unknown".to_string(),
        };
        let right = Box::new(condition_parts[2].clone());

        Box::new(ASTNode::BinaryExpression {
            left,
            operator,
            right,
        })
    } else if condition_parts.len() == 1 {
        // Single condition (probably a boolean or variable)
        Box::new(condition_parts[0].clone())
    } else {
        // Default to true if no condition found (shouldn't happen in valid code)
        Box::new(ASTNode::BooleanLiteral(true))
    };

    // Parse 'then' branch (statements inside braces)
    let mut then_branch = Vec::new();

    // Check for opening brace
    if let Some(token) = token_iter.next() {
        if token.token != Token::LeftBrace {
            return Err(KdnLangError::parser_error(
                "".to_string(),
                "unknown",
                (token.span.start, token.span.end - token.span.start),
                "Expected '{' after condition",
                "The if statement body should be wrapped in braces, like: if (condition) { ... }",
            )
            .into());
        }
    }

    // Parse the statements inside the then branch
    scope_stack.push(Vec::new());

    // Skip content within then branch for now (simplified)
    let mut brace_count = 1;
    while let Some(token) = token_iter.peek() {
        match token.token {
            Token::LeftBrace => {
                brace_count += 1;
            }
            Token::RightBrace => {
                brace_count -= 1;
                if brace_count == 0 {
                    token_iter.next(); // Consume the right brace
                    break;
                }
            }
            _ => {}
        }

        then_branch = scope_stack.last_mut().unwrap().clone();
        token_iter.next();
    }

    if let Some(inner_scope) = scope_stack.pop() {
        then_branch = inner_scope;
    }

    // Check for 'else if' before checking for just 'else'
    let mut else_branch = None;

    if let Some(token) = token_iter.peek() {
        if token.token == Token::Keyword && token.lexeme == "else" {
            token_iter.next(); // Consume 'else'

            // Check if this is an "else if" statement
            if let Some(next_token) = token_iter.peek() {
                if next_token.token == Token::Keyword && next_token.lexeme == "if" {
                    // This is an "else if" block - would need to recursively handle this
                    // For simplicity, we'll just consume the tokens but in a real implementation
                    // would need to properly parse the nested if statement
                    token_iter.next(); // Consume 'if'
                } else {
                    // Regular 'else' block
                    // Check for opening brace
                    if let Some(token) = token_iter.next() {
                        if token.token != Token::LeftBrace {
                            return Err(KdnLangError::parser_error(
                                "".to_string(),
                                "unknown",
                                (token.span.start, token.span.end - token.span.start),
                                "Expected '{' after 'else'",
                                "The else statement body should be wrapped in braces, like: else { ... }",
                            )
                            .into());
                        }
                    }

                    // Parse the statements inside the else branch
                    scope_stack.push(Vec::new());

                    // Skip content within else branch for now (simplified)
                    let mut brace_count = 1;
                    while let Some(token) = token_iter.peek() {
                        match token.token {
                            Token::LeftBrace => {
                                brace_count += 1;
                            }
                            Token::RightBrace => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    token_iter.next(); // Consume the right brace
                                    break;
                                }
                            }
                            _ => {}
                        }

                        token_iter.next();
                    }

                    if let Some(inner_scope) = scope_stack.pop() {
                        else_branch = Some(inner_scope);
                    }
                }
            }
        }
    }

    // Create an if node and add it to the current scope
    let if_node = ASTNode::If {
        condition,
        then_branch,
        else_branch,
    };

    if let Some(scope) = scope_stack.last_mut() {
        scope.push(if_node);
    }

    Ok(())
}
