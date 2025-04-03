use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::{Diagnostic, NamedSource, Result, SourceSpan};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Conditional parsing error: {message}")]
#[diagnostic(code(kdnlang::parser::conditional::error))]
pub struct ConditionalParseError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,

    pub message: String,
}

pub fn parse_if_statement(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<(), miette::Error> {
    // Parse condition (inside parentheses)
    if let Some(token) = token_iter.next() {
        if token.token != Token::LeftParen {
            return Err(ConditionalParseError {
                src: NamedSource::new("unknown", String::new()),
                span: (token.span.0, token.span.1 - token.span.0).into(),
                message: "Expected '(' after 'if'".to_string(),
            }
            .into());
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
            Token::StringLiteral => {
                condition_parts.push(ASTNode::StringLiteral(token.lexeme.to_string()));
            }
            Token::NumberLiteral => {
                condition_parts.push(ASTNode::Number(token.lexeme.to_string()));
            }
            Token::Equal => {
                // For now, we'll handle basic equality comparison
                if let Some(next_token) = token_iter.next() {
                    if next_token.token == Token::Equal {
                        condition_parts.push(ASTNode::Operator("==".to_string()));
                    }
                }
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
    } else {
        Box::new(ASTNode::Identifier("true".to_string())) // Fallback
    };

    // Parse 'then' branch (statements inside braces)
    let mut then_branch = Vec::new();

    // Check for opening brace
    if let Some(token) = token_iter.next() {
        if token.token != Token::LeftBrace {
            return Err(ConditionalParseError {
                src: NamedSource::new("unknown", String::new()),
                span: (token.span.0, token.span.1 - token.span.0).into(),
                message: "Expected '{' after condition".to_string(),
            }
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

    // Check for 'else' branch
    let mut else_branch = None;

    if let Some(token) = token_iter.peek() {
        if token.token == Token::Keyword && token.lexeme == "else" {
            token_iter.next(); // Consume 'else'

            // Check for opening brace
            if let Some(token) = token_iter.next() {
                if token.token != Token::LeftBrace {
                    return Err(ConditionalParseError {
                        src: NamedSource::new("unknown", String::new()),
                        span: (token.span.0, token.span.1 - token.span.0).into(),
                        message: "Expected '{' after 'else'".to_string(),
                    }
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
