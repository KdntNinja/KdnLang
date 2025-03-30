use crate::compiler::ast::{ASTNode, MatchPattern};
use crate::parser::DataType;
use crate::errors::{KdnError, KdnResult, span};
use miette::NamedSource;
use std::collections::HashMap;

/// Type checker for validating KdnLang program semantics
pub struct TypeChecker {
    /// Symbol table mapping variable names to their types
    symbol_table: HashMap<String, DataType>,
    /// Current function return type (if inside a function)
    current_function_return_type: Option<DataType>,
}

impl TypeChecker {
    /// Create a new type checker with an empty symbol table
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: HashMap::new(),
            current_function_return_type: None,
        }
    }
    
    /// Validate the type correctness of an AST
    /// 
    /// # Arguments
    /// * `file_path` - Path to the source file (for error reporting)
    /// * `ast` - The AST node to type check
    /// * `source_code` - Original source code (for error reporting)
    /// 
    /// # Returns
    /// * `KdnResult<()>` - Ok if type checking succeeds, Error otherwise
    pub fn check(file_path: &str, ast: &ASTNode, source_code: &str) -> KdnResult<()> {
        let mut checker: TypeChecker = Self::new();
        checker.check_node(file_path, ast, source_code)
    }
    
    /// Check a single AST node for type correctness
    /// 
    /// # Arguments
    /// * `file_path` - Path to the source file (for error reporting)
    /// * `node` - The AST node to type check
    /// * `source_code` - Original source code (for error reporting)
    /// 
    /// # Returns
    /// * `KdnResult<()>` - Ok if type checking succeeds, Error otherwise
    fn check_node(&mut self, file_path: &str, node: &ASTNode, source_code: &str) -> KdnResult<()> {
        match node {
            ASTNode::Function { name, params, return_type, body } => {
                // Create a new scope for the function
                let outer_scope: HashMap<String, DataType> = self.symbol_table.clone();
                let outer_return_type: Option<DataType> = self.current_function_return_type.clone();
                
                // Add parameters to symbol table
                for (param_name, param_type) in params {
                    self.symbol_table.insert(param_name.clone(), param_type.clone());
                }
                
                // Set current function return type
                self.current_function_return_type = return_type.clone();
                
                // Check each statement in the function body
                for (index, statement) in body.iter().enumerate() {
                    // Add function context to any type errors
                    let result: KdnResult<()> = self.check_node(file_path, statement, source_code)
                        .map_err(|e| match e {
                            KdnError::TypeError { src, message, span } => KdnError::TypeError {
                                src,
                                message: format!("In function '{}' at statement #{}: {}", name, index + 1, message),
                                span,
                            },
                            other => other,
                        });
                    result?;
                }
                
                // Restore outer scope
                self.symbol_table = outer_scope;
                self.current_function_return_type = outer_return_type;
                
                Ok(())
            },
            
            ASTNode::Variable { name, data_type, value } => {
                // Check that the value's type matches the declared type
                let value_type: DataType = self.infer_type(file_path, value, source_code)?;
                
                if !self.types_compatible(&value_type, data_type) {
                    return Err(KdnError::TypeError {
                        src: NamedSource::new(file_path, source_code.to_string()),
                        message: format!(
                            "Type mismatch in variable '{}': expected {}, got {}", 
                            name, data_type.to_string(), value_type.to_string()
                        ),
                        span: span(0, 10), // Default span since we don't have position info
                    });
                }
                
                // Add variable to symbol table
                self.symbol_table.insert(name.clone(), data_type.clone());
                
                Ok(())
            },
            
            ASTNode::Print { expression } => {
                // Verify the expression is of a printable type
                let _expr_type: DataType = self.infer_type(file_path, expression, source_code)?;
                Ok(())
            },
            
            ASTNode::Return { value } => {
                // Check that return type matches function's declared return type
                let return_value_type: DataType = self.infer_type(file_path, value, source_code)?;
                
                if let Some(expected_type) = &self.current_function_return_type {
                    if !self.types_compatible(&return_value_type, expected_type) {
                        return Err(KdnError::TypeError {
                            src: NamedSource::new(file_path, source_code.to_string()),
                            message: format!(
                                "Return type mismatch: expected {}, got {}", 
                                expected_type.to_string(), return_value_type.to_string()
                            ),
                            span: span(0, 10), // Default span
                        });
                    }
                } else {
                    // If no return type is declared, we're in a none-returning function
                    return Err(KdnError::TypeError {
                        src: NamedSource::new(file_path, source_code.to_string()),
                        message: "Return statement in a none-returning function".to_string(),
                        span: span(0, 10), // Default span
                    });
                }
                
                Ok(())
            },
            
            ASTNode::IfStatement { condition, then_block, else_block } => {
                // Check that condition is a boolean
                let condition_type: DataType = self.infer_type(file_path, condition, source_code)?;
                if condition_type != DataType::Bool {
                    return Err(KdnError::TypeError {
                        src: NamedSource::new(file_path, source_code.to_string()),
                        message: format!(
                            "If condition must be a boolean, got {}", 
                            condition_type.to_string()
                        ),
                        span: span(0, 10), // Default span
                    });
                }
                
                // Check then block
                for stmt in then_block {
                    self.check_node(file_path, stmt, source_code)?;
                }
                
                // Check else block if it exists
                if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.check_node(file_path, stmt, source_code)?;
                    }
                }
                
                Ok(())
            },
            
            ASTNode::MatchStatement { expression, arms } => {
                // Get the type of the match expression
                let _expr_type: DataType = self.infer_type(file_path, expression, source_code)?;
                
                // Check each match arm
                for (_pattern, stmts) in arms {
                    // Check statements in the arm
                    for stmt in stmts {
                        self.check_node(file_path, stmt, source_code)?;
                    }
                }
                
                Ok(())
            },
            
            ASTNode::BinaryOp { op: _, left, right, result_type: _ } => {
                // Get types of operands
                let _left_type: DataType = self.infer_type(file_path, left, source_code)?;
                let _right_type: DataType = self.infer_type(file_path, right, source_code)?;
                
                // Return Ok(()) instead of the result_type to match the function's return type
                Ok(())
            },
            
            // Literal values are always well-typed
            ASTNode::Number(_) | ASTNode::Float(_) | ASTNode::String(_) | ASTNode::Boolean(_) => {
                Ok(())
            },
            
            ASTNode::Identifier { name, inferred_type: _ } => {
                // Check that the identifier is in scope
                if !self.symbol_table.contains_key(name) {
                    return Err(KdnError::TypeError {
                        src: NamedSource::new(file_path, source_code.to_string()),
                        message: format!("Undefined variable: {}", name),
                        span: span(0, 10), // Default span
                    });
                }
                
                Ok(())
            },
            
            ASTNode::FunctionCall { name: _, args, return_type: _ } => {
                // For now, just check that arguments are well-typed
                // In a full compiler, we'd check against function signatures
                for arg in args {
                    self.infer_type(file_path, arg, source_code)?;
                }
                
                Ok(())
            },
        }
    }
    
    /// Infer the type of an AST node
    /// 
    /// # Arguments
    /// * `file_path` - Path to the source file (for error reporting)
    /// * `node` - The AST node to infer the type of
    /// * `source_code` - Original source code (for error reporting)
    /// 
    /// # Returns
    /// * `KdnResult<DataType>` - The inferred type or an error
    fn infer_type(&mut self, file_path: &str, node: &ASTNode, source_code: &str) -> KdnResult<DataType> {
        match node {
            ASTNode::Number(_) => Ok(DataType::I32),
            ASTNode::Float(_) => Ok(DataType::F64),
            ASTNode::String(_) => Ok(DataType::Str),
            ASTNode::Boolean(_) => Ok(DataType::Bool),
            
            ASTNode::Identifier { name, inferred_type } => {
                // If we already inferred the type, return it
                if let Some(t) = inferred_type {
                    return Ok(t.clone());
                }
                
                // Look up the identifier in the symbol table
                if let Some(t) = self.symbol_table.get(name) {
                    return Ok(t.clone());
                }
                
                // If not found, it's an error
                Err(KdnError::TypeError {
                    src: NamedSource::new(file_path, source_code.to_string()),
                    message: format!("Undefined variable: {}", name),
                    span: span(0, 10), // Default span
                })
            },
            
            ASTNode::FunctionCall { name, args, return_type } => {
                // If we already inferred the return type, return it
                if let Some(t) = return_type {
                    return Ok(t.clone());
                }
                
                // For built-in functions, infer their return types
                // In a real compiler, we'd check against a function signature table
                match name.as_str() {
                    "print" => Ok(DataType::None),
                    "input" => Ok(DataType::Str),
                    "parse" => {
                        // parse() should return i32 if the argument is a string
                        if args.len() == 1 {
                            let arg_type: DataType = self.infer_type(file_path, &args[0], source_code)?;
                            if arg_type == DataType::Str {
                                Ok(DataType::I32)
                            } else {
                                Err(KdnError::TypeError {
                                    src: NamedSource::new(file_path, source_code.to_string()),
                                    message: format!("parse() requires a string argument, got {}", arg_type.to_string()),
                                    span: span(0, 10), // Default span
                                })
                            }
                        } else {
                            Err(KdnError::TypeError {
                                src: NamedSource::new(file_path, source_code.to_string()),
                                message: "parse() requires exactly one string argument".to_string(),
                                span: span(0, 10), // Default span
                            })
                        }
                    },
                    "str" => {
                        // str() should convert any value to a string
                        if args.len() == 1 {
                            // Try to infer the argument type to ensure it's valid
                            self.infer_type(file_path, &args[0], source_code)?;
                            Ok(DataType::Str)
                        } else {
                            Err(KdnError::TypeError {
                                src: NamedSource::new(file_path, source_code.to_string()),
                                message: "str() requires exactly one argument".to_string(),
                                span: span(0, 10), // Default span
                            })
                        }
                    },
                    // For user-defined functions, we'd look up their signatures
                    _ => {
                        // For now, assume all unknown functions return i32
                        // This would be replaced with proper function signature lookup
                        Ok(DataType::I32)
                    }
                }
            },
            
            ASTNode::BinaryOp { op: _op, left, right, result_type } => {
                // Get types of operands
                let _left_type: DataType = self.infer_type(file_path, left, source_code)?;
                let _right_type: DataType = self.infer_type(file_path, right, source_code)?;
                
                // Return the result_type directly since it's already a DataType, not an Option
                return Ok(result_type.clone());
            },
            
            // For other node types, we don't have a meaningful type
            _ => Ok(DataType::None),
        }
    }
    
    /// Check if two types are compatible (one can be assigned to the other)
    /// 
    /// # Arguments
    /// * `from_type` - The source type
    /// * `to_type` - The target type
    /// 
    /// # Returns
    /// * `bool` - Whether the types are compatible
    fn types_compatible(&self, from_type: &DataType, to_type: &DataType) -> bool {
        // Exact type match
        if from_type == to_type {
            return true;
        }
        
        // Numeric type compatibility (for numeric operations)
        if self.is_numeric(from_type) && self.is_numeric(to_type) {
            return true;
        }
        
        // Add more compatibility rules as needed
        
        false
    }
    
    /// Check if a type is numeric
    /// 
    /// # Arguments
    /// * `data_type` - The type to check
    /// 
    /// # Returns
    /// * `bool` - Whether the type is numeric
    fn is_numeric(&self, data_type: &DataType) -> bool {
        matches!(data_type, DataType::I32 | DataType::F64)
    }
}
