use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use crate::parser::call_parser::{parse_function_call, parse_print};
use crate::parser::conditional_parser::parse_if_statement;
use crate::parser::function_parser::parse_function;
use crate::parser::variable_parser::parse_variable;
use miette::Result;
use pest::error::InputLocation;
use pest_derive::Parser; // Use pest_derive for the Parser derive macro

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
#[allow(dead_code)]
pub struct KdnLangParser;

// The Rule enum is automatically made public and available

#[allow(dead_code)]
pub fn convert_location_to_span(location: InputLocation) -> miette::SourceSpan {
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

    let src_content: String = if !tokens.is_empty() {
        tokens
            .iter()
            .map(|token: &TokenWithSpan<'_>| token.lexeme)
            .collect::<String>()
    } else {
        String::new()
    };

    // Using the pest grammar rule mapping to process tokens
    while let Some(token) = token_iter.peek() {
        match token.token {
            Token::Keyword if token.lexeme == "fn" => {
                token_iter.next();
                parse_function(&mut token_iter, &mut scope_stack)?;
            }
            Token::Keyword if token.lexeme == "let" => {
                token_iter.next();
                parse_variable(&mut token_iter, &mut scope_stack, &src_content, filename)?;
            }
            Token::Keyword if token.lexeme == "if" => {
                token_iter.next();
                parse_if_statement(&mut token_iter, &mut scope_stack)?;
            }
            Token::Keyword if token.lexeme == "while" => {
                // Would implement parse_while_statement here
                token_iter.next();
            }
            Token::Keyword if token.lexeme == "struct" => {
                // Would implement parse_struct_definition here
                token_iter.next();
            }
            Token::Keyword if token.lexeme == "match" => {
                // Would implement parse_match_statement here
                token_iter.next();
            }
            Token::Keyword if token.lexeme == "return" => {
                // Would implement parse_return_statement here
                token_iter.next();
            }
            Token::Identifier if token.lexeme == "print" => {
                token_iter.next();
                parse_print(&mut token_iter, &mut scope_stack)?;
            }
            Token::Identifier => {
                // First check if it's a function call, otherwise handle as an identifier
                let next_token = token_iter.clone().nth(1);
                if let Some(nt) = next_token {
                    if nt.token == Token::LeftParen {
                        parse_function_call(&mut token_iter, &mut scope_stack)?;
                    } else if nt.token == Token::Operator && nt.lexeme == "=" {
                        // Would implement parse_assignment here
                        token_iter.next();
                    } else {
                        // Unhandled identifier usage
                        token_iter.next();
                    }
                } else {
                    token_iter.next();
                }
            }
            Token::StringLiteral | Token::Number | Token::BoolLiteral => {
                // Handle literals - for now just add literals as nodes
                match token.token {
                    Token::StringLiteral => {
                        let string_node = ASTNode::StringLiteral(token.lexeme.to_string());
                        if let Some(scope) = scope_stack.last_mut() {
                            scope.push(string_node);
                        }
                    }
                    Token::Number => {
                        let num_node = ASTNode::Number(token.lexeme.to_string());
                        if let Some(scope) = scope_stack.last_mut() {
                            scope.push(num_node);
                        }
                    }
                    Token::BoolLiteral => {
                        let bool_value = token.lexeme == "true";
                        let bool_node = ASTNode::BooleanLiteral(bool_value);
                        if let Some(scope) = scope_stack.last_mut() {
                            scope.push(bool_node);
                        }
                    }
                    _ => {}
                }
                token_iter.next();
            }
            _ => {
                // Skip unhandled tokens
                token_iter.next();
            }
        }
    }

    if let Some(global_scope) = scope_stack.pop() {
        Ok(ASTNode::Block(global_scope))
    } else {
        Ok(ASTNode::Block(Vec::new()))
    }
}
