use crate::lexer::tokens::TokenWithSpan;
use crate::lexer::Token;
use crate::parser::ast::ASTNode;
use crate::parser::call_parser::{parse_function_call, parse_print};
use crate::parser::conditional_parser::parse_if_statement;
use crate::parser::function_parser::parse_function;
use crate::parser::variable_parser::parse_variable;
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

    let src_content: String = if !tokens.is_empty() {
        tokens
            .iter()
            .map(|token: &TokenWithSpan<'_>| token.lexeme)
            .collect::<String>()
    } else {
        String::new()
    };

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
            Token::Identifier if token.lexeme == "print" => {
                token_iter.next();
                parse_print(&mut token_iter, &mut scope_stack)?;
            }
            Token::StringLiteral => {
                let string_node = ASTNode::StringLiteral(token.lexeme.to_string());
                if let Some(scope) = scope_stack.last_mut() {
                    scope.push(string_node);
                }
                token_iter.next();
            }
            Token::Identifier => {
                parse_function_call(&mut token_iter, &mut scope_stack)?;
            }
            _ => {
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
