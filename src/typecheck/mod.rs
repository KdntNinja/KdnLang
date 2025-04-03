use crate::error_handling::errors::KdnLangError;
use crate::parser::ASTNode; // Updated import
use miette::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
    Any,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
}

impl Type {
    pub fn from_string(type_str: &str) -> Self {
        match type_str {
            "int" => Type::Int,
            "float" => Type::Float,
            "str" => Type::String,
            "bool" => Type::Bool,
            "void" => Type::Void,
            _ => Type::Any,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::String => "str".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::Any => "any".to_string(),
            Type::Function {
                params,
                return_type,
            } => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("fn({}) -> {}", params_str, return_type.to_string())
            }
        }
    }

    // Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            // Any type is compatible with any other type
            (Type::Any, _) | (_, Type::Any) => true,

            // Int can be used as Float
            (Type::Int, Type::Float) => true,

            // Custom compatibility rules can be added here
            _ => false,
        }
    }
}

pub struct TypeEnvironment {
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        let mut env = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };

        // Add built-in functions
        env.functions.insert(
            "print".to_string(),
            Type::Function {
                params: vec![Type::Any],
                return_type: Box::new(Type::Void),
            },
        );

        env.functions.insert(
            "input".to_string(),
            Type::Function {
                params: vec![Type::String],
                return_type: Box::new(Type::String),
            },
        );

        env
    }

    pub fn define_variable(&mut self, name: &str, var_type: Type) {
        self.variables.insert(name.to_string(), var_type);
    }

    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    pub fn define_function(&mut self, name: &str, func_type: Type) {
        self.functions.insert(name.to_string(), func_type);
    }

    pub fn get_function_type(&self, name: &str) -> Option<&Type> {
        self.functions.get(name)
    }
}

pub fn typecheck_program(
    node: &ASTNode,
    filename: &str,
    source: &str,
) -> Result<(), miette::Error> {
    let mut env = TypeEnvironment::new();
    typecheck_node(node, &mut env, filename, source)?; // Add ? to unwrap the result
    Ok(()) // Return an empty result if no errors
}

fn typecheck_node(
    node: &ASTNode,
    env: &mut TypeEnvironment,
    filename: &str,
    source: &str,
) -> Result<Type, miette::Error> {
    match node {
        ASTNode::Number(_) => Ok(Type::Int),

        ASTNode::StringLiteral(_) => Ok(Type::String),

        ASTNode::BooleanLiteral(_) => Ok(Type::Bool),

        ASTNode::Identifier(name) => {
            if let Some(var_type) = env.get_variable_type(name) {
                Ok(var_type.clone())
            } else {
                Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    (0, 0), // This should be the proper span in real use
                    format!("Undefined variable: {}", name),
                    format!("Make sure '{}' is defined before using it.", name),
                )
                .into())
            }
        }

        ASTNode::Assignment {
            variable,
            type_annotation,
            value,
        } => {
            // Check the type of the value
            let value_type = typecheck_node(value, env, filename, source)?;

            // Get the expected type from the annotation
            let expected_type = Type::from_string(type_annotation);

            // Check if the types are compatible
            if !value_type.is_compatible_with(&expected_type) && expected_type != Type::Any {
                return Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    (0, 0), // This should be the proper span
                    format!(
                        "Type mismatch: Cannot assign value of type {} to variable '{}' of type {}",
                        value_type.to_string(),
                        variable,
                        expected_type.to_string()
                    ),
                    "Make sure the types match or use explicit type conversion.".to_string(),
                )
                .into());
            }

            // Define the variable in our type environment
            env.define_variable(variable, expected_type.clone());

            Ok(expected_type)
        }

        ASTNode::BinaryExpression {
            left,
            operator,
            right,
        } => {
            let left_type = typecheck_node(left, env, filename, source)?;
            let right_type = typecheck_node(right, env, filename, source)?;

            match operator.as_str() {
                // Arithmetic operators
                "+" | "-" | "*" | "/" | "%" => {
                    if (left_type == Type::Int || left_type == Type::Float)
                        && (right_type == Type::Int || right_type == Type::Float)
                    {
                        // If either operand is a float, the result is a float
                        if left_type == Type::Float || right_type == Type::Float {
                            Ok(Type::Float)
                        } else {
                            Ok(Type::Int)
                        }
                    } else if operator == "+"
                        && left_type == Type::String
                        && right_type == Type::String
                    {
                        // String concatenation
                        Ok(Type::String)
                    } else {
                        Err(KdnLangError::runtime_error(
                            source.to_string(),
                            filename,
                            (0, 0),
                            format!(
                                "Type mismatch: Cannot apply operator '{}' to types {} and {}",
                                operator,
                                left_type.to_string(),
                                right_type.to_string()
                            ),
                            "Make sure the operands have compatible types.".to_string(),
                        )
                        .into())
                    }
                }

                // Comparison operators
                "==" | "!=" => {
                    if left_type.is_compatible_with(&right_type)
                        || right_type.is_compatible_with(&left_type)
                    {
                        Ok(Type::Bool)
                    } else {
                        Err(KdnLangError::runtime_error(
                            source.to_string(),
                            filename,
                            (0, 0),
                            format!(
                                "Type mismatch: Cannot compare types {} and {}",
                                left_type.to_string(),
                                right_type.to_string()
                            ),
                            "Make sure the operands have compatible types.".to_string(),
                        )
                        .into())
                    }
                }

                "<" | ">" | "<=" | ">=" => {
                    if (left_type == Type::Int || left_type == Type::Float)
                        && (right_type == Type::Int || right_type == Type::Float)
                    {
                        Ok(Type::Bool)
                    } else {
                        Err(KdnLangError::runtime_error(
                            source.to_string(),
                            filename,
                            (0, 0),
                            format!(
                                "Type mismatch: Cannot apply operator '{}' to types {} and {}",
                                operator,
                                left_type.to_string(),
                                right_type.to_string()
                            ),
                            "Comparison operators only work on numeric types.".to_string(),
                        )
                        .into())
                    }
                }

                // Logical operators
                "&&" | "||" => {
                    if left_type == Type::Bool && right_type == Type::Bool {
                        Ok(Type::Bool)
                    } else {
                        Err(KdnLangError::runtime_error(
                            source.to_string(),
                            filename,
                            (0, 0),
                            format!(
                                "Type mismatch: Cannot apply operator '{}' to types {} and {}",
                                operator,
                                left_type.to_string(),
                                right_type.to_string()
                            ),
                            "Logical operators only work on boolean types.".to_string(),
                        )
                        .into())
                    }
                }

                _ => Ok(Type::Any), // Default to Any for unknown operators
            }
        }

        ASTNode::FunctionCall { name, args } => {
            // Check if the function exists
            let func_type_option = env.get_function_type(name).cloned(); // Clone the function type to avoid borrowing issues

            if let Some(func_type) = func_type_option {
                if let Type::Function {
                    params,
                    return_type,
                } = func_type
                {
                    // Check if argument count matches
                    if args.len() != params.len() && !(params.len() == 1 && params[0] == Type::Any)
                    {
                        return Err(KdnLangError::runtime_error(
                            source.to_string(),
                            filename,
                            (0, 0),
                            format!("Wrong number of arguments for function '{}': expected {}, got {}", 
                                   name, params.len(), args.len()),
                            "Check the function signature and provide the correct number of arguments.".to_string(),
                        ).into());
                    }

                    // Check each argument's type
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = typecheck_node(arg, env, filename, source)?;
                        let param_type = if params.len() == 1 && params[0] == Type::Any {
                            Type::Any // Use Type::Any directly instead of reference
                        } else {
                            params[i].clone() // Clone to avoid borrowing issues
                        };

                        if !arg_type.is_compatible_with(&param_type) {
                            return Err(KdnLangError::runtime_error(
                                source.to_string(),
                                filename,
                                (0, 0),
                                format!("Type mismatch for argument {} of function '{}': expected {}, got {}", 
                                       i+1, name, param_type.to_string(), arg_type.to_string()),
                                "Make sure the argument types match the function signature.".to_string(),
                            ).into());
                        }
                    }

                    Ok(*return_type)
                } else {
                    Err(KdnLangError::runtime_error(
                        source.to_string(),
                        filename,
                        (0, 0),
                        format!("'{}' is not a function", name),
                        "Make sure you're calling a function.".to_string(),
                    )
                    .into())
                }
            } else {
                Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    (0, 0),
                    format!("Undefined function: {}", name),
                    format!(
                        "Make sure the function '{}' is defined before calling it.",
                        name
                    ),
                )
                .into())
            }
        }

        ASTNode::Function {
            name,
            parameters,
            return_type,
            body,
        } => {
            // Create function type
            let param_types = parameters
                .iter()
                .map(|(_, type_str)| Type::from_string(type_str))
                .collect();

            let func_type = Type::Function {
                params: param_types,
                return_type: Box::new(Type::from_string(return_type)),
            };

            // Add function to environment
            env.define_function(name, func_type.clone());

            // Type check the function body (we could check return statements match the return type)
            for node in body {
                typecheck_node(node, env, filename, source)?;
            }

            Ok(func_type)
        }

        ASTNode::If {
            condition,
            then_branch,
            else_branch,
        } => {
            // Condition must be a boolean
            let cond_type = typecheck_node(condition, env, filename, source)?;
            if cond_type != Type::Bool && cond_type != Type::Any {
                return Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    (0, 0),
                    format!(
                        "Type mismatch: Condition must be a boolean, got {}",
                        cond_type.to_string()
                    ),
                    "Make sure the condition evaluates to a boolean value.".to_string(),
                )
                .into());
            }

            // Type check branches
            for node in then_branch {
                typecheck_node(node, env, filename, source)?;
            }

            if let Some(else_nodes) = else_branch {
                for node in else_nodes {
                    typecheck_node(node, env, filename, source)?;
                }
            }

            Ok(Type::Void)
        }

        ASTNode::Block(nodes) => {
            let mut last_type = Type::Void;
            for node in nodes {
                last_type = typecheck_node(node, env, filename, source)?;
            }
            Ok(last_type)
        }

        _ => Ok(Type::Any), // Default case
    }
}
