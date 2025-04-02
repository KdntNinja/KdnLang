use crate::error_handling::errors::ParseErrorWithDetails;
use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::Result;

pub fn parse_variable(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
    src_content: &str,
    filename: &str,
) -> Result<()> {
    // The 'let' keyword has already been consumed by the caller
    if let Some(name_token) = token_iter.next() {
        if name_token.token == Token::Identifier {
            let variable_name = name_token.lexeme.to_string();

            // Check for type annotation
            let mut type_annotation = String::new();
            if let Some(colon_token) = token_iter.peek() {
                if colon_token.token == Token::Colon {
                    token_iter.next(); // Consume the colon
                    if let Some(type_token) = token_iter.next() {
                        if type_token.token == Token::Identifier
                            || type_token.token == Token::TypeKeyword
                        {
                            type_annotation = type_token.lexeme.to_string();
                        }
                    }
                }
            }

            // Check for equals sign
            if let Some(equals_token) = token_iter.peek() {
                if equals_token.token == Token::Punctuation && equals_token.lexeme == "=" {
                    token_iter.next(); // Consume the equals sign

                    // Check if we have a function call
                    if let Some(fn_token) = token_iter.peek().cloned() {
                        if fn_token.token == Token::Identifier {
                            let fn_name = fn_token.lexeme.to_string();
                            token_iter.next(); // Consume the function name

                            // Parse function call with arguments
                            if let Some(open_paren) = token_iter.peek() {
                                if open_paren.token == Token::Bracket && open_paren.lexeme == "(" {
                                    token_iter.next(); // Consume the open parenthesis

                                    let mut args = Vec::new();

                                    // Parse arguments
                                    loop {
                                        if let Some(arg_token) = token_iter.peek().cloned() {
                                            if arg_token.token == Token::Bracket
                                                && arg_token.lexeme == ")"
                                            {
                                                break;
                                            }

                                            match arg_token.token {
                                                Token::StringLiteral => {
                                                    args.push(ASTNode::StringLiteral(
                                                        arg_token.lexeme.to_string(),
                                                    ));
                                                    token_iter.next(); // Consume the argument
                                                }
                                                Token::Number => {
                                                    args.push(ASTNode::Number(
                                                        arg_token.lexeme.to_string(),
                                                    ));
                                                    token_iter.next(); // Consume the argument
                                                }
                                                Token::Identifier => {
                                                    args.push(ASTNode::Identifier(
                                                        arg_token.lexeme.to_string(),
                                                    ));
                                                    token_iter.next(); // Consume the argument
                                                }
                                                _ => break,
                                            }

                                            // Handle comma between arguments
                                            if let Some(comma) = token_iter.peek() {
                                                if comma.token == Token::Punctuation
                                                    && comma.lexeme == ","
                                                {
                                                    token_iter.next(); // Consume the comma
                                                }
                                            } else {
                                                break;
                                            }
                                        } else {
                                            break;
                                        }
                                    }

                                    // Consume closing parenthesis
                                    if let Some(close_paren) = token_iter.next() {
                                        if close_paren.token == Token::Bracket
                                            && close_paren.lexeme == ")"
                                        {
                                            // Ensure the statement ends with a semicolon
                                            if let Some(semicolon_token) = token_iter.next() {
                                                if semicolon_token.token == Token::Semicolon {
                                                    let function_call = ASTNode::FunctionCall {
                                                        name: fn_name,
                                                        args,
                                                    };

                                                    let assignment_node = ASTNode::Assignment {
                                                        variable: variable_name,
                                                        type_annotation,
                                                        value: Box::new(function_call),
                                                    };

                                                    if let Some(current_scope) =
                                                        scope_stack.last_mut()
                                                    {
                                                        current_scope.push(assignment_node);
                                                    }

                                                    return Ok(());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Handle simple value expressions (non-function calls)
                    if let Some(value_token) = token_iter.next() {
                        let value_node = match value_token.token {
                            Token::StringLiteral => {
                                ASTNode::StringLiteral(value_token.lexeme.to_string())
                            }
                            Token::Number => ASTNode::Number(value_token.lexeme.to_string()),
                            Token::Identifier => {
                                ASTNode::Identifier(value_token.lexeme.to_string())
                            }
                            _ => {
                                return Err(ParseErrorWithDetails::missing_semicolon(
                                    src_content.to_string(),
                                    filename,
                                    (
                                        value_token.span.start,
                                        value_token.span.end - value_token.span.start,
                                    )
                                        .into(),
                                    "Invalid value expression",
                                )
                                .into());
                            }
                        };

                        // Ensure the statement ends with a semicolon
                        if let Some(semicolon_token) = token_iter.next() {
                            if semicolon_token.token == Token::Semicolon {
                                let assignment_node = ASTNode::Assignment {
                                    variable: variable_name,
                                    type_annotation,
                                    value: Box::new(value_node),
                                };

                                if let Some(current_scope) = scope_stack.last_mut() {
                                    current_scope.push(assignment_node);
                                }
                                return Ok(());
                            } else {
                                return Err(ParseErrorWithDetails::missing_semicolon(
                                    src_content.to_string(),
                                    filename,
                                    (
                                        semicolon_token.span.start,
                                        semicolon_token.span.end - semicolon_token.span.start,
                                    )
                                        .into(),
                                    "Missing semicolon",
                                )
                                .into());
                            }
                        }
                    }
                }
            }
        }
    }

    Err(ParseErrorWithDetails {
        src: miette::NamedSource::new(filename, src_content.to_string()),
        span: (0, 1).into(),
        message: "Invalid variable declaration".to_string(),
        help_text: "Variables must be declared with 'let NAME: TYPE = VALUE;' syntax".to_string(),
    }
    .into())
}
