use crate::parser::ast::ASTNode;
use crate::parser::parser_impl::Rule;
use crate::parser::parser_impl::{convert_location_to_span, KdnLangParser, ParseError};
use miette::{NamedSource, Result};
use pest::iterators::Pairs;
use pest::Parser;

#[allow(dead_code)]
pub fn parse_expression(input: &str) -> Result<ASTNode, ParseError> {
    let input_owned: String = input.to_string();
    let pairs: Pairs<Rule> = KdnLangParser::parse(Rule::expression, &input_owned).map_err(
        |e: pest::error::Error<Rule>| ParseError {
            src: NamedSource::new("input.kdn", input_owned.clone()),
            span: convert_location_to_span(e.location),
        },
    )?;

    let mut ast_nodes: Vec<ASTNode> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::number => {
                ast_nodes.push(ASTNode::Number(pair.as_str().to_string()));
            }
            Rule::operator => {
                ast_nodes.push(ASTNode::Operator(pair.as_str().to_string()));
            }
            Rule::identifier => {
                ast_nodes.push(ASTNode::Identifier(pair.as_str().to_string()));
            }
            Rule::string_literal => {
                ast_nodes.push(ASTNode::StringLiteral(pair.as_str().to_string()));
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
pub fn parse_function(input: &str) -> Result<ASTNode, ParseError> {
    let pairs: Pairs<Rule> =
        KdnLangParser::parse(Rule::function, input).map_err(|e: pest::error::Error<Rule>| {
            ParseError {
                src: NamedSource::new("input.kdn", input.to_string()),
                span: convert_location_to_span(e.location),
            }
        })?;

    let mut inner_pairs: Pairs<Rule> = pairs;
    let name: String = inner_pairs.next().unwrap().as_str().to_string();
    let body: Vec<ASTNode> = inner_pairs
        .map(|pair: pest::iterators::Pair<Rule>| parse_expression(pair.as_str()).unwrap())
        .collect();

    Ok(ASTNode::Function { name, body })
}

#[allow(dead_code)]
pub fn parse_match(input: &str) -> Result<ASTNode, ParseError> {
    let pairs: Pairs<Rule> = KdnLangParser::parse(Rule::match_statement, input).map_err(
        |e: pest::error::Error<Rule>| ParseError {
            src: NamedSource::new("input.kdn", input.to_string()),
            span: convert_location_to_span(e.location),
        },
    )?;

    let mut inner_pairs: Pairs<Rule> = pairs;
    let expression: Box<ASTNode> =
        Box::new(parse_expression(inner_pairs.next().unwrap().as_str())?);
    let arms: Vec<(String, ASTNode)> = inner_pairs
        .map(|pair: pest::iterators::Pair<Rule>| {
            let mut arm_pairs: Pairs<Rule> = pair.into_inner();
            let pattern: String = arm_pairs.next().unwrap().as_str().to_string();
            let result: ASTNode = parse_expression(arm_pairs.next().unwrap().as_str()).unwrap();
            (pattern, result)
        })
        .collect();

    Ok(ASTNode::Match { expression, arms })
}

#[allow(dead_code)]
pub fn parse_try_except(input: &str) -> Result<ASTNode, ParseError> {
    let pairs: Pairs<Rule> =
        KdnLangParser::parse(Rule::try_except, input).map_err(|e: pest::error::Error<Rule>| {
            ParseError {
                src: NamedSource::new("input.kdn", input.to_string()),
                span: convert_location_to_span(e.location),
            }
        })?;

    let mut inner_pairs: Pairs<Rule> = pairs;
    let try_block: ASTNode = parse_expression(inner_pairs.next().unwrap().as_str())?;
    let except_block: ASTNode = parse_expression(inner_pairs.next().unwrap().as_str())?;

    Ok(ASTNode::Block(vec![try_block, except_block]))
}
