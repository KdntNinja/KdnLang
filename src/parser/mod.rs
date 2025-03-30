use pest_derive::Parser;
use pest::Parser;
use miette::{NamedSource, SourceSpan};
use crate::errors::{KdnError, KdnResult, span};

/// Parser for KdnLang, using the Pest parsing library
#[derive(Parser)]
#[grammar = "src/parser/grammar.pest"]
pub struct KdnParser;

/// Available data types in KdnLang
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum DataType {
    /// 32-bit signed integer
    I32,
    /// 64-bit floating point
    F64,
    /// String type
    Str,
    /// Boolean type
    Bool,
    /// User-defined type (struct name)
    Custom(String),
    /// Function return type
    Function {
        /// Parameter types
        params: Vec<DataType>,
        /// Return type (None means void)
        return_type: Option<Box<DataType>>,
    },
    /// Void type (used for expressions with no value)
    None,
}

impl DataType {
    /// Parse a type string into a DataType enum
    pub fn from_str(type_str: &str) -> KdnResult<Self> {
        match type_str {
            "i32" => Ok(DataType::I32),
            "f64" => Ok(DataType::F64),
            "str" => Ok(DataType::Str),
            "bool" => Ok(DataType::Bool),
            "none" => Ok(DataType::None),
            custom => Ok(DataType::Custom(custom.to_string())),
        }
    }
    
    /// Convert a DataType to its string representation
    pub fn to_string(&self) -> String {
        match self {
            DataType::I32 => "i32".to_string(),
            DataType::F64 => "f64".to_string(),
            DataType::Str => "str".to_string(),
            DataType::Bool => "bool".to_string(),
            DataType::None => "none".to_string(),
            DataType::Custom(name) => name.clone(),
            DataType::Function { params, return_type } => {
                let params_str = params.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                
                match return_type {
                    Some(ret) => format!("fn({}) -> {}", params_str, ret.to_string()),
                    None => format!("fn({})", params_str),
                }
            }
        }
    }
}

/// Abstract Syntax Tree node types representing program structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ASTNode {
    /// Function definition with name, parameters, return type, and body statements
    Function {
        /// Function name
        name: String,
        /// Function parameters (name, type)
        params: Vec<(String, DataType)>,
        /// Function return type
        return_type: Option<DataType>,
        /// Function body
        body: Vec<ASTNode>,
    },
    
    /// Variable declaration with name, type, and value
    Variable {
        /// Variable name
        name: String,
        /// Variable type
        data_type: DataType,
        /// Initial value
        value: Box<ASTNode>,
    },
    
    /// Print statement with expression to output
    Print {
        /// Expression to print
        expression: Box<ASTNode>,
    },
    
    /// Return statement with value to return
    Return {
        /// Value to return
        value: Box<ASTNode>,
    },
    
    /// If statement with condition, then block, and optional else block
    IfStatement {
        /// Condition expression
        condition: Box<ASTNode>,
        /// Then block statements
        then_block: Vec<ASTNode>,
        /// Optional else block statements
        else_block: Option<Vec<ASTNode>>,
    },
    
    /// Match statement with expression and arms
    MatchStatement {
        /// Expression to match on
        expression: Box<ASTNode>,
        /// Match arms (pattern, statements)
        arms: Vec<(MatchPattern, Vec<ASTNode>)>,
    },
    
    /// Binary operation
    BinaryOp {
        /// Operator type
        op: String,
        /// Left operand
        left: Box<ASTNode>,
        /// Right operand
        right: Box<ASTNode>,
        /// Result type
        result_type: DataType,
    },
    
    /// Numeric literal value
    Number(i32),
    
    /// Floating point literal value
    Float(f64),
    
    /// String literal value
    String(String),
    
    /// Boolean literal value
    Boolean(bool),
    
    /// Variable or function reference
    Identifier {
        /// Variable or function name
        name: String,
        /// Inferred type (filled in by type checker)
        inferred_type: Option<DataType>,
    },
    
    /// Function call
    FunctionCall {
        /// Function name
        name: String,
        /// Arguments
        args: Vec<ASTNode>,
        /// Return type (filled in by type checker)
        return_type: Option<DataType>,
    },
}

/// Match pattern for match statements
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MatchPattern {
    /// Range pattern (inclusive)
    Range(i32, i32),
    /// Wildcard pattern
    Wildcard,
    /// Literal value pattern
    Literal(ASTNode),
}

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
        let pairs: pest::iterators::Pairs<Rule> = KdnParser::parse(Rule::program, input)
            .map_err(|e: pest::error::Error<Rule>| -> KdnError { 
                // Extract error position information from the pest error
                // Default to position 0 if we can't determine the exact location
                let error_position: usize = 0;
                let error_span: SourceSpan = span(error_position, 1);
                
                KdnError::ParserError {
                    src: NamedSource::new(file_path, source_code.to_string()),
                    message: e.to_string(),
                    span: error_span,
                }
            })?;
            
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
    fn build_ast(file_path: &str, pairs: pest::iterators::Pairs<Rule>, source_code: &str) -> KdnResult<ASTNode> {
        let mut ast_nodes: Vec<ASTNode> = Vec::new();
        
        for pair in pairs {
            match pair.as_rule() {
                Rule::function => {
                    // Parse function definition
                    let mut inner = pair.into_inner();
                    
                    // Extract function name
                    let name = inner.next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected function name".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str().to_string();
                    
                    // Parse parameters if they exist
                    let mut params = Vec::new();
                    if let Some(parameters_pair) = inner.next() {
                        if parameters_pair.as_rule() == Rule::parameters {
                            for param_pair in parameters_pair.into_inner() {
                                let mut param_inner = param_pair.into_inner();
                                
                                let param_name = param_inner.next()
                                    .ok_or_else(|| KdnError::ParserError {
                                        src: NamedSource::new(file_path, source_code.to_string()),
                                        message: "Expected parameter name".to_string(),
                                        span: span(0, 10),
                                    })?
                                    .as_str().to_string();
                                
                                let type_str = param_inner.next()
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
                            body.push(Self::build_ast(file_path, stmt_pair.into_inner(), source_code)?);
                        }
                    }
                    
                    ast_nodes.push(ASTNode::Function { 
                        name, 
                        params, 
                        return_type, 
                        body 
                    });
                },
                
                Rule::let_statement => {
                    // Parse variable declaration
                    let mut inner = pair.into_inner();
                    
                    let name = inner.next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected variable name".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str().to_string();
                    
                    // Type is now required
                    let type_str = inner.next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected variable type".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str();
                    
                    let data_type = DataType::from_str(type_str)?;
                    
                    let value = inner.next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected variable value".to_string(),
                            span: span(0, 10),
                        })?;
                    
                    let value_node = Self::build_ast(file_path, pest::iterators::Pairs::single(value), source_code)?;
                    
                    ast_nodes.push(ASTNode::Variable { 
                        name, 
                        data_type, 
                        value: Box::new(value_node) 
                    });
                },
                
                Rule::print => {
                    // Parse print statement
                    let inner = pair.into_inner().next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected expression in print statement".to_string(),
                            span: span(0, 10),
                        })?;
                    
                    let expression = Self::build_ast(file_path, pest::iterators::Pairs::single(inner), source_code)?;
                    
                    ast_nodes.push(ASTNode::Print { 
                        expression: Box::new(expression) 
                    });
                },
                
                Rule::return_statement => {
                    // Parse return statement
                    let inner = pair.into_inner().next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected expression in return statement".to_string(),
                            span: span(0, 10),
                        })?;
                    
                    let value = Self::build_ast(file_path, pest::iterators::Pairs::single(inner), source_code)?;
                    
                    ast_nodes.push(ASTNode::Return { 
                        value: Box::new(value) 
                    });
                },
                
                Rule::number => {
                    // Parse numeric literal
                    let num_str = pair.as_str();
                    let num = num_str.parse()
                        .map_err(|_| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: format!("Failed to parse number: {}", num_str),
                            span: span(pair.as_span().start(), pair.as_span().end() - pair.as_span().start()),
                        })?;
                    
                    ast_nodes.push(ASTNode::Number(num));
                },
                
                Rule::string => {
                    // Parse string literal (strip the quotes first)
                    let str_content = pair.as_str();
                    let content = str_content.trim_matches('"');
                    
                    ast_nodes.push(ASTNode::String(content.to_string()));
                },
                
                Rule::identifier => {
                    // Parse identifier (variable or function name)
                    let id = pair.as_str().to_string();
                    
                    ast_nodes.push(ASTNode::Identifier { 
                        name: id,
                        inferred_type: None,
                    });
                },
                
                Rule::function_call => {
                    // Parse function call
                    let mut inner = pair.into_inner();
                    
                    let name = inner.next()
                        .ok_or_else(|| KdnError::ParserError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: "Expected function name in function call".to_string(),
                            span: span(0, 10),
                        })?
                        .as_str().to_string();
                    
                    // Parse arguments
                    let mut args = Vec::new();
                    if let Some(args_pair) = inner.next() {
                        if args_pair.as_rule() == Rule::arguments {
                            for arg_pair in args_pair.into_inner() {
                                args.push(Self::build_ast(file_path, pest::iterators::Pairs::single(arg_pair), source_code)?);
                            }
                        }
                    }
                    
                    ast_nodes.push(ASTNode::FunctionCall { 
                        name, 
                        args, 
                        return_type: None,  // To be filled in by type checker
                    });
                },
                
                // Try to handle other rule types
                _ => {
                    // For rules that have inner pairs, recurse to process them
                    if pair.as_rule() != Rule::EOI && !pair.as_str().is_empty() {
                        if let Some(inner_node) = Self::build_ast(file_path, pair.into_inner(), source_code).ok() {
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