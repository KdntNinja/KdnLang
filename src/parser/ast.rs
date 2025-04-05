use std::collections::HashMap;
use pest::iterators::{Pair, Pairs};
use miette::{NamedSource, Report};
use pest::Parser;

use crate::error::{KdnLangError, Result};
use crate::parser::KdnParser;

/// Types of AST nodes in our language
#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    LetStatement {
        name: String,
        type_annotation: Option<String>,
        value: Box<AstNode>,
    },
    Assignment {
        name: String,
        value: Box<AstNode>,
    },
    PrintStatement {
        expression: Box<AstNode>,
    },
    ForLoop {
        variable: String,
        range_start: Box<AstNode>,
        range_end: Box<AstNode>,
        body: Vec<AstNode>,
    },
    BinaryOp {
        op: String,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Number(f64),
    Identifier(String),
}

/// Parse a KdnLang program string into an AST
pub fn parse_program(input: &str) -> Result<AstNode> {
    // Parse the input using pest
    let pairs = KdnParser::parse(Rule::program, input)
        .map_err(|e| Report::new(KdnLangError {
            src: NamedSource::new("input", input.to_string()),
            span: (0, input.len().min(10)).into(),
            help: Some(format!("Parser error: {}", e)),
        }))?;
    
    // Convert parse result to AST
    let ast = build_ast_from_pairs(pairs);
    
    Ok(ast)
}

/// Build an AST from Pest parser output pairs
fn build_ast_from_pairs(pairs: Pairs<Rule>) -> AstNode {
    let mut statements = Vec::new();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::directive => {
                let directive = parse_directive(pair);
                statements.push(directive);
            },
            Rule::EOI => {}, // End of input, ignore
            _ => {}, // Ignore other rules for now
        }
    }
    
    AstNode::Program(statements)
}

/// Parse a directive rule into an AST node
fn parse_directive(pair: Pair<Rule>) -> AstNode {
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::let_statement => parse_let_statement(inner),
        Rule::print_statement => parse_print_statement(inner),
        Rule::for_loop => parse_for_loop(inner),
        _ => unreachable!("Unknown directive type: {:?}", inner.as_rule()),
    }
}

/// Parse a let statement into an AST node
fn parse_let_statement(pair: Pair<Rule>) -> AstNode {
    let mut pairs = pair.into_inner();
    
    let identifier = pairs.next().unwrap();
    let name = identifier.as_str().to_string();
    
    let mut type_annotation = None;
    let mut next_pair = pairs.next().unwrap();
    
    // Check if we have a type annotation
    if next_pair.as_rule() == Rule::type_annotation {
        type_annotation = Some(next_pair.as_str().to_string());
        next_pair = pairs.next().unwrap(); // Get the expression
    }
    
    let expr_pair = next_pair;
    let expr = parse_expression(expr_pair);
    
    AstNode::LetStatement {
        name,
        type_annotation,
        value: Box::new(expr),
    }
}

/// Parse a print statement into an AST node
fn parse_print_statement(pair: Pair<Rule>) -> AstNode {
    let expr_pair = pair.into_inner().next().unwrap();
    let expr = parse_expression(expr_pair);
    
    AstNode::PrintStatement {
        expression: Box::new(expr),
    }
}

/// Parse a for loop into an AST node
fn parse_for_loop(pair: Pair<Rule>) -> AstNode {
    let mut pairs = pair.into_inner();
    
    let identifier = pairs.next().unwrap();
    let variable = identifier.as_str().to_string();
    
    let range_pair = pairs.next().unwrap();
    let mut range_pairs = range_pair.into_inner();
    
    let range_start_pair = range_pairs.next().unwrap();
    let range_end_pair = range_pairs.next().unwrap();
    
    let range_start = parse_expression(range_start_pair);
    let range_end = parse_expression(range_end_pair);
    
    let block_pair = pairs.next().unwrap();
    let mut body = Vec::new();
    
    for directive_pair in block_pair.into_inner() {
        if directive_pair.as_rule() == Rule::directive {
            body.push(parse_directive(directive_pair));
        }
    }
    
    AstNode::ForLoop {
        variable,
        range_start: Box::new(range_start),
        range_end: Box::new(range_end),
        body,
    }
}

/// Parse an expression into an AST node
fn parse_expression(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => parse_binary_expression(pair, true),
        _ => unreachable!("Expected expression, got {:?}", pair.as_rule()),
    }
}

/// Parse a binary expression (for addition/subtraction)
fn parse_binary_expression(pair: Pair<Rule>, is_addition: bool) -> AstNode {
    let mut pairs = pair.into_inner();
    
    // First parse the left term
    let mut node = if is_addition {
        // For addition/subtraction expressions, first element is a term
        let term_pair = pairs.next().unwrap();
        parse_binary_expression(term_pair, false)
    } else {
        // For multiplication/division expressions, first element is a factor
        let factor_pair = pairs.next().unwrap();
        parse_factor(factor_pair)
    };
    
    // Then handle any operators that follow
    while let Some(op_pair) = pairs.next() {
        let op = op_pair.as_str().to_string();
        
        let right = if is_addition {
            // For addition/subtraction, right operand is a term
            let term_pair = pairs.next().unwrap();
            parse_binary_expression(term_pair, false)
        } else {
            // For multiplication/division, right operand is a factor
            let factor_pair = pairs.next().unwrap();
            parse_factor(factor_pair)
        };
        
        node = AstNode::BinaryOp {
            op,
            left: Box::new(node),
            right: Box::new(right),
        };
    }
    
    node
}

/// Parse a factor into an AST node
fn parse_factor(pair: Pair<Rule>) -> AstNode {
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::number => {
            let value = inner.as_str().parse::<f64>().unwrap();
            AstNode::Number(value)
        },
        Rule::identifier => {
            let name = inner.as_str().to_string();
            AstNode::Identifier(name)
        },
        Rule::expr => parse_expression(inner),
        _ => unreachable!("Unknown factor rule: {:?}", inner.as_rule()),
    }
}

// Define the Rule enum to match our Pest grammar
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    program,
    directive,
    let_statement,
    type_annotation,
    print_statement,
    for_loop,
    range,
    block,
    expr,
    term,
    factor,
    identifier,
    number,
    WHITESPACE,
    EOI,
}