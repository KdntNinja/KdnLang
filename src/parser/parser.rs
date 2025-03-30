use crate::errors::{span, KdnError, KdnResult};
use crate::compiler::ast::{ASTNode, MatchPattern};
use crate::parser::DataType;
use miette::NamedSource;
use pest::Parser;
use pest_derive::Parser;
use miette::SourceSpan;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct KdnParser;

impl KdnParser {
    /// Parse a KdnLang program from a string input
    ///
    /// # Arguments
    /// * `file_path` - Path to the source file (for error reporting)
    /// * `input` - The tokenized input to parse
    /// * `source_code` - Original source code (for error reporting)
    ///
    /// # Returns
    /// * `KdnResult<ASTNode>` - The root AST node or an error
    pub fn parse_program(file_path: &str, input: &str, source_code: &str) -> KdnResult<ASTNode> {
        // Parse the input according to the grammar rules
        let pairs: pest::iterators::Pairs<Rule> = KdnParser::parse(Rule::program, input).map_err(
            |e: pest::error::Error<Rule>| -> KdnError {
                // Extract error position information from the pest error
                // Default to position 0 if we can't determine the exact location
                let error_position: usize = 0;
                let error_span: SourceSpan = span(error_position, 1);

                KdnError::ParserError {
                    src: NamedSource::new(file_path, source_code.to_string()),
                    message: e.to_string(),
                    span: error_span,
                }
            },
        )?;

        Self::build_ast(file_path, pairs, source_code)
    }

    /// Recursively build an AST from parse results
    ///
    /// # Arguments
    /// * `file_path` - Path to the source file (for error reporting)
    /// * `pairs` - The pairs of matched rules and tokens from pest
    /// * `source_code` - Original source code (for error reporting)
    ///
    /// # Returns
    /// * `KdnResult<ASTNode>` - An AST node or an error
    fn build_ast(
        file_path: &str,
        pairs: pest::iterators::Pairs<Rule>,
        source_code: &str,
    ) -> KdnResult<ASTNode> {
        let mut ast_nodes: Vec<ASTNode> = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::function => {
                    // Parse function definition
                    let mut inner = pair.into_inner();

                    // Extract function name
                    let name = inner
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected function name".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str()
                        .to_string();

                    // Parse parameters if they exist
                    let mut params = Vec::new();
                    if let Some(parameters_pair) = inner.next() {
                        if parameters_pair.as_rule() == Rule::parameters {
                            for param_pair in parameters_pair.into_inner() {
                                let mut param_inner = param_pair.into_inner();

                                let param_name = param_inner
                                    .next()
                                    .ok_or_else(|| KdnError::ParserError {
                                        src: NamedSource::new(file_path, source_code.to_string()),
                                        message: "Expected parameter name".to_string(),
                                        span: span(0, 10),
                                    })?
                                    .as_str()
                                    .to_string();

                                let type_str = param_inner
                                    .next()
                                    .ok_or_else(|| KdnError::ParserError {
                                        src: NamedSource::new(file_path, source_code.to_string()),
                                        message: "Expected parameter type".to_string(),
                                        span: span(0, 10),
                                    })?
                                    .as_str();

                                let param_type = DataType::from_str(type_str)?;
                                params.push((param_name, param_type));
                            }
                        }
                    }

                    // Parse return type
                    let return_type = if let Some(type_pair) = inner.next() {
                        if type_pair.as_rule() == Rule::type_declaration {
                            let type_str = type_pair.as_str();
                            if type_str == "none" {
                                None
                            } else {
                                Some(DataType::from_str(type_str)?)
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    // Parse function body statements
                    let mut body = Vec::new();
                    for stmt_pair in inner {
                        if stmt_pair.as_rule() == Rule::statement {
                            body.push(Self::build_ast(
                                file_path,
                                stmt_pair.into_inner(),
                                source_code,
                            )?);
                        }
                    }

                    ast_nodes.push(ASTNode::Function {
                        name,
                        params,
                        return_type,
                        body,
                    });
                }

                Rule::let_statement => {
                    // Parse variable declaration
                    let mut inner = pair.into_inner();

                    let name = inner
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected variable name".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str()
                        .to_string();

                    // Type is now required
                    let type_str = inner
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected variable type".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str();

                    let data_type = DataType::from_str(type_str)?;

                    let value = inner.next().ok_or_else(|| KdnError::ParserError {
                        src: NamedSource::new(file_path, source_code.to_string()),
                        message: "Expected variable value".to_string(),
                        span: span(0, 10),
                    })?;

                    let value_node = Self::build_ast(
                        file_path,
                        pest::iterators::Pairs::single(value),
                        source_code,
                    )?;

                    ast_nodes.push(ASTNode::Variable {
                        name,
                        data_type,
                        value: Box::new(value_node),
                    });
                }

                Rule::print => {
                    // Parse print statement
                    let inner = pair
                        .into_inner()
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected expression in print statement".to_string(),
                            span: span(0, 10),
                        })?;

                    let expression = Self::build_ast(
                        file_path,
                        pest::iterators::Pairs::single(inner),
                        source_code,
                    )?;

                    ast_nodes.push(ASTNode::Print {
                        expression: Box::new(expression),
                    });
                }

                Rule::return_statement => {
                    // Parse return statement
                    let inner = pair
                        .into_inner()
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected expression in return statement".to_string(),
                            span: span(0, 10),
                        })?;

                    let value = Self::build_ast(
                        file_path,
                        pest::iterators::Pairs::single(inner),
                        source_code,
                    )?;

                    ast_nodes.push(ASTNode::Return {
                        value: Box::new(value),
                    });
                }

                Rule::if_statement => {
                    // Parse if statement
                    let mut inner = pair.into_inner();

                    let condition = inner
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected condition in if statement".to_string(),
                            span: span(0, 10),
                        })?;

                    let condition_node = Self::build_ast(
                        file_path,
                        pest::iterators::Pairs::single(condition),
                        source_code,
                    )?;

                    let mut then_block = Vec::new();
                    let mut else_block = None;

                    for stmt_pair in inner {
                        if stmt_pair.as_rule() == Rule::statement {
                            then_block.push(Self::build_ast(
                                file_path,
                                stmt_pair.into_inner(),
                                source_code,
                            )?);
                        } else if stmt_pair.as_rule() == Rule::else_block {
                            let mut else_stmts = Vec::new();
                            for else_stmt_pair in stmt_pair.into_inner() {
                                else_stmts.push(Self::build_ast(
                                    file_path,
                                    else_stmt_pair.into_inner(),
                                    source_code,
                                )?);
                            }
                            else_block = Some(else_stmts);
                        }
                    }

                    ast_nodes.push(ASTNode::IfStatement {
                        condition: Box::new(condition_node),
                        then_block,
                        else_block,
                    });
                }

                Rule::match_statement => {
                    // Parse match statement
                    let mut inner = pair.into_inner();

                    let expression = inner
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected expression in match statement".to_string(),
                            span: span(0, 10),
                        })?;

                    let expression_node = Self::build_ast(
                        file_path,
                        pest::iterators::Pairs::single(expression),
                        source_code,
                    )?;

                    let mut arms = Vec::new();

                    for arm_pair in inner {
                        let mut arm_inner = arm_pair.into_inner();

                        let pattern = arm_inner
                            .next()
                            .ok_or_else(|| KdnError::ParserError {
                                src: NamedSource::new(file_path, source_code.to_string()),
                                message: "Expected pattern in match arm".to_string(),
                                span: span(0, 10),
                            })?;

                        let pattern_node = match pattern.as_rule() {
                            Rule::range => {
                                let mut range_inner = pattern.into_inner();
                                let start = range_inner
                                    .next()
                                    .ok_or_else(|| KdnError::ParserError {
                                        src: NamedSource::new(file_path, source_code.to_string()),
                                        message: "Expected start of range".to_string(),
                                        span: span(0, 10),
                                    })?
                                    .as_str()
                                    .parse()
                                    .map_err(|_| KdnError::ParserError {
                                        src: NamedSource::new(file_path, source_code.to_string()),
                                        message: "Failed to parse start of range".to_string(),
                                        span: span(0, 10),
                                    })?;

                                let end = range_inner
                                    .next()
                                    .ok_or_else(|| KdnError::ParserError {
                                        src: NamedSource::new(file_path, source_code.to_string()),
                                        message: "Expected end of range".to_string(),
                                        span: span(0, 10),
                                    })?
                                    .as_str()
                                    .parse()
                                    .map_err(|_| KdnError::ParserError {
                                        src: NamedSource::new(file_path, source_code.to_string()),
                                        message: "Failed to parse end of range".to_string(),
                                        span: span(0, 10),
                                    })?;

                                MatchPattern::Range(start, end)
                            }
                            Rule::literal_pattern => {
                                let literal_node = Self::build_ast(
                                    file_path,
                                    pest::iterators::Pairs::single(pattern),
                                    source_code,
                                )?;
                                MatchPattern::Literal(literal_node)
                            }
                            Rule::wildcard => MatchPattern::Wildcard,
                            _ => {
                                return Err(KdnError::ParserError {
                                    src: NamedSource::new(file_path, source_code.to_string()),
                                    message: "Unexpected pattern in match arm".to_string(),
                                    span: span(0, 10),
                                });
                            }
                        };

                        let mut statements = Vec::new();
                        for stmt_pair in arm_inner {
                            statements.push(Self::build_ast(
                                file_path,
                                stmt_pair.into_inner(),
                                source_code,
                            )?);
                        }

                        arms.push((pattern_node, statements));
                    }

                    ast_nodes.push(ASTNode::MatchStatement {
                        expression: Box::new(expression_node),
                        arms,
                    });
                }

                Rule::number => {
                    // Parse numeric literal
                    let num_str = pair.as_str();
                    let num = num_str.parse().map_err(|_| KdnError::ParserError {
                        src: NamedSource::new(file_path, source_code.to_string()),
                        message: format!("Failed to parse number: {}", num_str),
                        span: span(
                            pair.as_span().start(),
                            pair.as_span().end() - pair.as_span().start(),
                        ),
                    })?;

                    ast_nodes.push(ASTNode::Number(num));
                }

                Rule::string => {
                    // Parse string literal (strip the quotes first)
                    let str_content = pair.as_str();
                    let content = str_content.trim_matches('"');

                    ast_nodes.push(ASTNode::String(content.to_string()));
                }

                Rule::identifier => {
                    // Parse identifier (variable or function name)
                    let id = pair.as_str().to_string();

                    ast_nodes.push(ASTNode::Identifier {
                        name: id,
                        inferred_type: None,
                    });
                }

                Rule::function_call => {
                    // Parse function call
                    let mut inner = pair.into_inner();

                    let name = inner
                        .next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected function name in function call".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str()
                        .to_string();

                    // Parse arguments
                    let mut args = Vec::new();
                    if let Some(args_pair) = inner.next() {
                        if args_pair.as_rule() == Rule::arguments {
                            for arg_pair in args_pair.into_inner() {
                                args.push(Self::build_ast(
                                    file_path,
                                    pest::iterators::Pairs::single(arg_pair),
                                    source_code,
                                )?);
                            }
                        }
                    }

                    ast_nodes.push(ASTNode::FunctionCall {
                        name,
                        args,
                        return_type: None, // To be filled in by type checker
                    });
                }

                // Try to handle other rule types
                _ => {
                    // For rules that have inner pairs, recurse to process them
                    if pair.as_rule() != Rule::EOI && !pair.as_str().is_empty() {
                        if let Some(inner_node) =
                            Self::build_ast(file_path, pair.into_inner(), source_code).ok()
                        {
                            ast_nodes.push(inner_node);
                        }
                    }
                }
            }
        }

        // Return the last AST node or error if empty
        ast_nodes.pop().ok_or_else(|| KdnError::ParserError {
            src: NamedSource::new(file_path, source_code.to_string()),
            message: "Failed to build AST: empty input".to_string(),
            span: span(0, 1),
        })
    }
}
