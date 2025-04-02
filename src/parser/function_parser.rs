use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::Result;

pub fn parse_function(
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>>,
    scope_stack: &mut Vec<Vec<ASTNode>>,
) -> Result<()> {
    if let Some(identifier_token) = token_iter.next() {
        if identifier_token.token == Token::Identifier {
            let function_name = identifier_token.lexeme.to_string();

            // Ensure the next token is an opening parenthesis
            if let Some(open_paren) = token_iter.next() {
                if open_paren.token == Token::Bracket && open_paren.lexeme == "(" {
                    // Skip parameters parsing for now
                    while let Some(param_token) = token_iter.next() {
                        if param_token.token == Token::Bracket && param_token.lexeme == ")" {
                            break;
                        }
                    }

                    // Ensure the next token is an opening brace
                    if let Some(open_brace) = token_iter.next() {
                        if open_brace.token == Token::Bracket && open_brace.lexeme == "{" {
                            scope_stack.push(Vec::new());

                            // Parse the function body
                            while let Some(body_token) = token_iter.next() {
                                if body_token.token == Token::Bracket && body_token.lexeme == "}" {
                                    break;
                                }
                                // Handle function body tokens (e.g., statements)
                            }

                            if let Some(body_scope) = scope_stack.pop() {
                                let function_node = ASTNode::Function {
                                    name: function_name,
                                    body: body_scope,
                                };

                                if let Some(current_scope) = scope_stack.last_mut() {
                                    current_scope.push(function_node);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
