use crate::parser::ast::ASTNode;
use crate::parser::parser_impl::{convert_location_to_span, KdnLangParser, Rule};
use miette::{Diagnostic, NamedSource, Result, SourceSpan};
use pest::iterators::Pairs;
use pest::Parser; // Import the Parser trait
use thiserror::Error;

// Define a custom ParseError for expression parser
#[derive(Debug, Diagnostic, Error)]
#[error("Parse error")]
#[diagnostic(code(kdnlang::parser::expression::error))]
pub struct ParseError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,
}

#[allow(dead_code)]
pub fn parse_expression(input: &str) -> Result<ASTNode, miette::Error> {
    let input_owned: String = input.to_string();
    let pairs: Pairs<Rule> = KdnLangParser::parse(Rule::expr, &input_owned).map_err(|e| {
        let error = ParseError {
            src: NamedSource::new("input.kdn", input_owned.clone()),
            span: convert_location_to_span(e.location),
        };
        // Explicitly specify the target error type
        let err: miette::Error = error.into();
        err
    })?;

    let mut ast_nodes: Vec<ASTNode> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::string_lit => {
                let lexeme = pair.as_str();
                ast_nodes.push(ASTNode::StringLiteral(lexeme.to_string()));
            }
            Rule::number => {
                ast_nodes.push(ASTNode::Number(pair.as_str().to_string()));
            }
            Rule::operator => {
                ast_nodes.push(ASTNode::Operator(pair.as_str().to_string()));
            }
            Rule::identifier => {
                ast_nodes.push(ASTNode::Identifier(pair.as_str().to_string()));
            }
            Rule::function_call => {
                let mut inner_pairs = pair.into_inner();
                let name = inner_pairs.next().unwrap().as_str().to_string();
                let args: Vec<ASTNode> = inner_pairs
                    .map(|arg| parse_expression(arg.as_str()).unwrap())
                    .collect();
                ast_nodes.push(ASTNode::FunctionCall { name, args });
            }
            _ => {}
        }
    }

    if ast_nodes.len() == 1 {
        Ok(ast_nodes.remove(0))
    } else {
        Ok(ASTNode::Block(ast_nodes))
    }
}

#[allow(dead_code)]
pub fn parse_function(input: &str) -> Result<ASTNode, miette::Error> {
    let pairs: Pairs<Rule> = KdnLangParser::parse(Rule::function_def, input).map_err(|e| {
        let error = ParseError {
            src: NamedSource::new("input.kdn", input.to_string()),
            span: convert_location_to_span(e.location),
        };
        // Explicitly specify the target error type
        let err: miette::Error = error.into();
        err
    })?;

    let mut inner_pairs = pairs;
    let name: String = inner_pairs.next().unwrap().as_str().to_string();
    let body: Vec<ASTNode> = inner_pairs
        .map(|pair| parse_expression(pair.as_str()).unwrap())
        .collect();

    Ok(ASTNode::Function {
        name,
        parameters: Vec::new(),          // Add empty parameters list
        return_type: "void".to_string(), // Add default return type
        body,
    })
}

#[allow(dead_code)]
pub fn parse_match(input: &str) -> Result<ASTNode, miette::Error> {
    let pairs: Pairs<Rule> = KdnLangParser::parse(Rule::match_statement, input).map_err(|e| {
        let error = ParseError {
            src: NamedSource::new("input.kdn", input.to_string()),
            span: convert_location_to_span(e.location),
        };
        // Explicitly specify the target error type
        let err: miette::Error = error.into();
        err
    })?;

    let mut inner_pairs = pairs;
    let expression: Box<ASTNode> =
        Box::new(parse_expression(inner_pairs.next().unwrap().as_str())?);
    let arms: Vec<(String, ASTNode)> = inner_pairs
        .map(|pair| {
            let mut arm_pairs = pair.into_inner();
            let pattern: String = arm_pairs.next().unwrap().as_str().to_string();
            let result: ASTNode = parse_expression(arm_pairs.next().unwrap().as_str()).unwrap();
            (pattern, result)
        })
        .collect();

    Ok(ASTNode::Match { expression, arms })
}

#[allow(dead_code)]
pub fn parse_try_except(_input: &str) -> Result<ASTNode, miette::Error> {
    // We don't have a try_except rule in the grammar yet, so we'll return a placeholder
    // When we add it to the grammar, we can uncomment this code
    /*
    let pairs: Pairs<Rule> = KdnLangParser::parse(Rule::try_except, input).map_err(|e| {
        let error = ParseError {
            src: NamedSource::new("input.kdn", input.to_string()),
            span: convert_location_to_span(e.location),
        };
        // Explicitly specify the target error type
        let err: miette::Error = error.into();
        err
    })?;

    let mut inner_pairs = pairs;
    let try_block: ASTNode = parse_expression(inner_pairs.next().unwrap().as_str())?;
    let except_block: ASTNode = parse_expression(inner_pairs.next().unwrap().as_str())?;
    */

    // For now, just return an empty block
    Ok(ASTNode::Block(vec![]))
}
