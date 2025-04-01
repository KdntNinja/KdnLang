use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use miette::{Diagnostic, Result};
use pest::error::InputLocation;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct KdnLangParser;

#[derive(Debug, Diagnostic, Error)]
#[error("Parse error")]
#[diagnostic(code(kdnlang::parser::error), help("Check the syntax of your input."))]
pub struct ParseError {
    #[source_code]
    pub src: String,

    #[label("Error occurred here")]
    pub span: (usize, usize),
}

pub fn convert_location_to_span(location: InputLocation) -> (usize, usize) {
    match location {
        InputLocation::Pos(pos) => (pos, pos + 1), // Single position
        InputLocation::Span((start, end)) => (start, end), // Span range
    }
}

pub fn parse_program(tokens: &[TokenWithSpan<'_>]) -> Result<ASTNode, ParseError> {
    let mut scope_stack: Vec<Vec<ASTNode>> = vec![Vec::new()];
    let mut token_iter = tokens.iter().peekable();

    while let Some(token) = token_iter.next() {
        match token.token {
            Token::Keyword if token.lexeme == "let" => {
                let variable = token_iter.next().unwrap().lexeme.to_string();
                token_iter.next(); // Skip `:`
                let type_annotation = token_iter.next().unwrap().lexeme.to_string();
                token_iter.next(); // Skip `=`
                let value = token_iter.next().unwrap().lexeme.to_string();

                let assignment = ASTNode::Assignment {
                    variable,
                    type_annotation,
                    value: Box::new(ASTNode::StringLiteral(value)),
                };

                if let Some(current_scope) = scope_stack.last_mut() {
                    current_scope.push(assignment);
                }
            }
            Token::Bracket if token.lexeme == "{" => {
                scope_stack.push(Vec::new());
            }
            Token::Bracket if token.lexeme == "}" => {
                if let Some(completed_scope) = scope_stack.pop() {
                    let block_node = ASTNode::Block(completed_scope);
                    if let Some(current_scope) = scope_stack.last_mut() {
                        current_scope.push(block_node);
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
