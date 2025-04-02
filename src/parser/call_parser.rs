use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::Result;

pub fn parse_function_call(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<()> {
    if let Some(function_name_token) = token_iter.next() {
        if function_name_token.token == Token::Identifier {
            let function_name: String = function_name_token.lexeme.to_string();

            // Ensure the next token is an opening parenthesis
            if let Some(open_paren) = token_iter.next() {
                if open_paren.token == Token::Bracket && open_paren.lexeme == "(" {
                    // Parse arguments
                    let mut args: Vec<ASTNode> = Vec::new();

                    while token_iter.peek().map_or(false, |t: &&TokenWithSpan<'_>| {
                        t.token != Token::Bracket || t.lexeme != ")"
                    }) {
                        if let Some(arg_token) = token_iter.next() {
                            if arg_token.token == Token::StringLiteral {
                                args.push(ASTNode::StringLiteral(arg_token.lexeme.to_string()));
                            } else if arg_token.token == Token::Number {
                                args.push(ASTNode::Number(arg_token.lexeme.to_string()));
                            } else if arg_token.token == Token::Identifier {
                                args.push(ASTNode::Identifier(arg_token.lexeme.to_string()));
                            }
                        }

                        if token_iter.peek().map_or(false, |t: &&TokenWithSpan<'_>| {
                            t.token == Token::Punctuation && t.lexeme == ","
                        }) {
                            token_iter.next();
                        }
                    }

                    token_iter.next(); // Consume the closing parenthesis

                    // Ensure the statement ends with a semicolon
                    if let Some(semicolon_token) = token_iter.next() {
                        if semicolon_token.token == Token::Semicolon {
                            let call_node: ASTNode = ASTNode::FunctionCall {
                                name: function_name,
                                args,
                            };

                            if let Some(current_scope) = scope_stack.last_mut() {
                                current_scope.push(call_node);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn parse_print(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<()> {
    let mut args: Vec<ASTNode> = Vec::new();

    if let Some(open_paren) = token_iter.next() {
        if open_paren.token == Token::Bracket && open_paren.lexeme == "(" {
            while let Some(arg_token) = token_iter.peek() {
                match arg_token.token {
                    Token::StringLiteral => {
                        args.push(ASTNode::StringLiteral(arg_token.lexeme.to_string()));
                        token_iter.next();
                    }
                    Token::Identifier => {
                        args.push(ASTNode::Identifier(arg_token.lexeme.to_string()));
                        token_iter.next();
                    }
                    _ => break,
                }

                if let Some(comma) = token_iter.peek() {
                    if comma.token == Token::Punctuation && comma.lexeme == "," {
                        token_iter.next();
                    }
                }
            }

            if let Some(close_paren) = token_iter.next() {
                if close_paren.token != Token::Bracket || close_paren.lexeme != ")" {
                    return Err(miette::miette!(
                        "Expected closing parenthesis in print statement"
                    ));
                }
            }

            if let Some(scope) = scope_stack.last_mut() {
                scope.push(ASTNode::FunctionCall {
                    name: "print".to_string(),
                    args,
                });
            }
        }
    }

    Ok(())
}
