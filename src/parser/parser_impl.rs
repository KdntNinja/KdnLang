use crate::error_handling::errors::ParseErrorWithDetails;
use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::{Diagnostic, NamedSource, Result, SourceSpan};
use pest::error::InputLocation;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
#[allow(dead_code)]
pub struct KdnLangParser;

#[derive(Debug, Diagnostic, Error)]
#[error("Parse error")]
#[diagnostic(code(kdnlang::parser::error), help("Check the syntax of your input."))]
pub struct ParseError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,
}

#[allow(dead_code)]
pub fn convert_location_to_span(location: InputLocation) -> SourceSpan {
    match location {
        InputLocation::Pos(pos) => (pos, 1).into(), // Single position
        InputLocation::Span((start, end)) => (start, end - start).into(), // Span range
    }
}

pub fn parse_program(
    tokens: &[TokenWithSpan<'_>],
    filename: &str,
) -> Result<ASTNode, miette::Error> {
    let mut scope_stack: Vec<Vec<ASTNode>> = vec![Vec::new()];
    let mut token_iter: std::iter::Peekable<std::slice::Iter<'_, TokenWithSpan<'_>>> =
        tokens.iter().peekable();

    // Combine all tokens into a string for error reporting
    let src_content: String = if !tokens.is_empty() {
        // Concatenate all token lexemes into a single string
        tokens
            .iter()
            .map(|token: &TokenWithSpan<'_>| token.lexeme)
            .collect::<String>()
    } else {
        String::new()
    };

    while let Some(token) = token_iter.next() {
        match token.token {
            Token::Keyword if token.lexeme == "let" => {
                if let Some(next_token) = token_iter.next() {
                    if next_token.token == Token::Identifier {
                        let variable: String = next_token.lexeme.to_string();

                        // Try to get the type annotation
                        if let Some(colon_token) = token_iter.next() {
                            if colon_token.token == Token::Colon {
                                if let Some(type_token) = token_iter.next() {
                                    let type_annotation: String = type_token.lexeme.to_string();

                                    // Get the equals sign
                                    if let Some(equals_token) = token_iter.next() {
                                        if equals_token.token == Token::Punctuation
                                            && equals_token.lexeme == "="
                                        {
                                            if let Some(value_token) = token_iter.next() {
                                                let value: String = value_token.lexeme.to_string();

                                                let assignment: ASTNode = ASTNode::Assignment {
                                                    variable,
                                                    type_annotation,
                                                    value: Box::new(ASTNode::StringLiteral(value)),
                                                };

                                                if let Some(current_scope) = scope_stack.last_mut()
                                                {
                                                    current_scope.push(assignment);
                                                }
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // If we get here, there was an error in the let statement syntax
                return Err(ParseError {
                    src: NamedSource::new(filename, src_content.clone()),
                    span: (token.span.start, token.span.end - token.span.start).into(),
                }
                .into());
            }
            Token::Bracket if token.lexeme == "{" => {
                scope_stack.push(Vec::new());
            }
            Token::Bracket if token.lexeme == "}" => {
                if let Some(completed_scope) = scope_stack.pop() {
                    let block_node: ASTNode = ASTNode::Block(completed_scope);
                    if let Some(current_scope) = scope_stack.last_mut() {
                        current_scope.push(block_node);
                    }
                }
            }
            Token::Identifier => {
                // Check if this is a function call
                if token_iter.peek().map_or(false, |t: &&TokenWithSpan<'_>| {
                    t.token == Token::Bracket && t.lexeme == "("
                }) {
                    let function_name: String = token.lexeme.to_string();
                    token_iter.next(); // Consume the opening bracket

                    // Parse arguments
                    let mut args: Vec<ASTNode> = Vec::new();

                    // While not at the closing bracket
                    while token_iter.peek().map_or(false, |t: &&TokenWithSpan<'_>| {
                        t.token != Token::Bracket || t.lexeme != ")"
                    }) {
                        // Parse argument expression
                        if let Some(arg_token) = token_iter.next() {
                            if arg_token.token == Token::StringLiteral {
                                args.push(ASTNode::StringLiteral(arg_token.lexeme.to_string()));
                            } else if arg_token.token == Token::Number {
                                args.push(ASTNode::Number(arg_token.lexeme.to_string()));
                            } else if arg_token.token == Token::Identifier {
                                args.push(ASTNode::Identifier(arg_token.lexeme.to_string()));
                            }
                        }

                        // Skip comma if present
                        if token_iter.peek().map_or(false, |t: &&TokenWithSpan<'_>| {
                            t.token == Token::Punctuation && t.lexeme == ","
                        }) {
                            token_iter.next();
                        }
                    }

                    // Consume the closing bracket
                    token_iter.next();

                    // Ensure function calls in statement position are followed by semicolons
                    if token_iter
                        .peek()
                        .map_or(false, |t: &&TokenWithSpan<'_>| t.token == Token::Semicolon)
                    {
                        token_iter.next(); // Consume the semicolon
                    } else {
                        // Missing semicolon error with clear indication and location
                        let call_end_pos: usize = if let Some(last_token) = token_iter.peek() {
                            last_token.span.start
                        } else {
                            src_content.len()
                        };

                        // Extract the function call for context
                        let call_context: &str = if call_end_pos > token.span.start
                            && call_end_pos <= src_content.len()
                        {
                            &src_content[token.span.start..call_end_pos]
                        } else {
                            function_name.as_str()
                        };

                        // Use the detailed error for missing semicolons with the actual filename
                        return Err(ParseErrorWithDetails::missing_semicolon(
                            src_content.clone(),
                            filename,
                            (call_end_pos, 0).into(),
                            call_context,
                        )
                        .into());
                    }

                    // Create function call node
                    let call_node: ASTNode = ASTNode::FunctionCall {
                        name: function_name,
                        args,
                    };

                    if let Some(current_scope) = scope_stack.last_mut() {
                        current_scope.push(call_node);
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(global_scope) = scope_stack.pop() {
        Ok(ASTNode::Block(global_scope))
    } else {
        Ok(ASTNode::Block(Vec::new()))
    }
}
