use crate::error_handling::errors::KdnLangError;
use crate::parser::ASTNode;
use miette::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Void,
    Any,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Array(Box<Type>),
    Optional(Box<Type>),
}

impl Type {
    pub fn from_string(type_str: &str) -> Self {
        match type_str {
            "int" => Type::Int,
            "float" => Type::Float,
            "str" => Type::Str,
            "bool" => Type::Bool,
            "void" => Type::Void,
            _ if type_str.starts_with("array<") && type_str.ends_with(">") => {
                let inner_type: &str = &type_str[6..type_str.len() - 1];
                Type::Array(Box::new(Type::from_string(inner_type)))
            }
            _ if type_str.starts_with("optional<") && type_str.ends_with(">") => {
                let inner_type = &type_str[9..type_str.len() - 1];
                Type::Optional(Box::new(Type::from_string(inner_type)))
            }
            _ => Type::Any,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Str => "str".to_string(), // Changed from "string" to "str"
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
            Type::Array(inner) => format!("array<{}>", inner.to_string()),
            Type::Optional(inner) => format!("optional<{}>", inner.to_string()),
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target, Type::Any) {
            return true;
        }

        if self == target {
            return true;
        }

        match (self, target) {
            (Type::Int, Type::Float) => true,
            (_, Type::Optional(inner)) if self.is_assignable_to(inner) => true,
            _ => false,
        }
    }

    pub fn valid_operations_with(&self, other: &Type) -> Vec<&'static str> {
        match (self, other) {
            (Type::Int, Type::Int) => {
                vec!["+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">="]
            }
            (Type::Float, Type::Float) => {
                vec!["+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">="]
            }
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => {
                vec!["+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">="]
            }
            (Type::Str, Type::Str) => vec!["+", "==", "!="],
            (Type::Bool, Type::Bool) => vec!["==", "!=", "&&", "||"],
            _ => vec![],
        }
    }

    pub fn supports_operation(&self, op: &str, other: &Type) -> bool {
        self.valid_operations_with(other).contains(&op)
    }
}

pub struct TypeEnvironment {
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
    current_function_return_type: Option<Type>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        let mut env = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_function_return_type: None,
        };

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
                params: vec![Type::Str],
                return_type: Box::new(Type::Str),
            },
        );

        env.functions.insert(
            "to_int".to_string(),
            Type::Function {
                params: vec![Type::Any],
                return_type: Box::new(Type::Int),
            },
        );

        env.functions.insert(
            "to_str".to_string(),
            Type::Function {
                params: vec![Type::Any],
                return_type: Box::new(Type::Str),
            },
        );

        env.functions.insert(
            "to_float".to_string(),
            Type::Function {
                params: vec![Type::Any],
                return_type: Box::new(Type::Float),
            },
        );

        env.functions.insert(
            "to_bool".to_string(),
            Type::Function {
                params: vec![Type::Any],
                return_type: Box::new(Type::Bool),
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

    pub fn set_current_function_return_type(&mut self, return_type: Type) {
        self.current_function_return_type = Some(return_type);
    }

    pub fn create_scope(&self) -> Self {
        Self {
            variables: HashMap::new(),
            functions: self.functions.clone(),
            current_function_return_type: self.current_function_return_type.clone(),
        }
    }
}

pub fn typecheck_program(
    node: &ASTNode,
    filename: &str,
    source: &str,
) -> Result<(), miette::Error> {
    let mut env = TypeEnvironment::new();
    typecheck_node(node, &mut env, filename, source)?;
    Ok(())
}

fn typecheck_node(
    node: &ASTNode,
    env: &mut TypeEnvironment,
    filename: &str,
    source: &str,
) -> Result<Type, miette::Error> {
    match node {
        ASTNode::Number(_) => Ok(Type::Int),

        ASTNode::StringLiteral(_) => Ok(Type::Str),

        ASTNode::BooleanLiteral(_) => Ok(Type::Bool),

        ASTNode::Identifier(name) => {
            if let Some(var_type) = env.get_variable_type(name) {
                Ok(var_type.clone())
            } else {
                Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    (0, 0),
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
            type_span,
        } => {
            let value_type = typecheck_node(value, env, filename, source)?;
            let expected_type = Type::from_string(type_annotation);

            if !value_type.is_assignable_to(&expected_type) {
                let error_span = match type_span {
                    Some(span) => *span,
                    None => (0, 0),
                };

                let mut error_msg = format!(
                    "Type mismatch: Cannot assign value of type {} to variable '{}' of type {}",
                    value_type.to_string(),
                    variable,
                    expected_type.to_string()
                );

                let suggestion = match (&value_type, &expected_type) {
                    (Type::Str, Type::Int) => Some(format!("to_int({})", variable)),
                    (Type::Str, Type::Float) => Some(format!("to_float({})", variable)),
                    (Type::Int, Type::Str) => Some(format!("to_str({})", variable)),
                    (Type::Float, Type::Str) => Some(format!("to_str({})", variable)),
                    (Type::Bool, Type::Str) => Some(format!("to_str({})", variable)),
                    _ => None,
                };

                if let Some(conversion) = suggestion {
                    error_msg.push_str(&format!(
                        ". Consider using the conversion function: {}",
                        conversion
                    ));
                }

                return Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    error_span,
                    error_msg,
                    "KdnLang enforces strict type safety. Use explicit type conversion functions."
                        .to_string(),
                )
                .into());
            }

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

            if !left_type.supports_operation(operator, &right_type) {
                return Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    (0, 0),
                    format!(
                        "Type error: Operator '{}' cannot be applied to types {} and {}",
                        operator,
                        left_type.to_string(),
                        right_type.to_string()
                    ),
                    format!(
                        "Valid operations for {} are: {}",
                        left_type.to_string(),
                        left_type.valid_operations_with(&right_type).join(", ")
                    ),
                )
                .into());
            }

            match operator.as_str() {
                "+" | "-" | "*" | "/" | "%" => match (&left_type, &right_type) {
                    (Type::Int, Type::Int) => Ok(Type::Int),
                    (Type::Float, _) | (_, Type::Float) => Ok(Type::Float),
                    (Type::Str, Type::Str) if operator == "+" => Ok(Type::Str),
                    _ => Ok(Type::Any),
                },
                "==" | "!=" | "<" | ">" | "<=" | ">=" => Ok(Type::Bool),
                "&&" | "||" => {
                    if left_type == Type::Bool && right_type == Type::Bool {
                        Ok(Type::Bool)
                    } else {
                        Ok(Type::Any)
                    }
                }
                _ => Ok(Type::Any),
            }
        }

        ASTNode::FunctionCall { name, args } => {
            let func_type_option = env.get_function_type(name).cloned();

            if let Some(func_type) = func_type_option {
                if let Type::Function {
                    params,
                    return_type,
                } = func_type
                {
                    if ["to_int", "to_float", "to_str", "to_bool"].contains(&name.as_str()) {
                        if args.len() != 1 {
                            return Err(KdnLangError::runtime_error(
                                source.to_string(),
                                filename,
                                (0, 0),
                                format!("Type conversion function '{}' requires exactly one argument", name),
                                "Type conversion functions should be called with a single argument.".to_string(),
                            ).into());
                        }

                        let _ = typecheck_node(&args[0], env, filename, source)?;

                        return Ok(*return_type);
                    }

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

                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = typecheck_node(arg, env, filename, source)?;
                        let param_type = if params.len() == 1 && params[0] == Type::Any {
                            Type::Any
                        } else {
                            params[i].clone()
                        };

                        if !arg_type.is_assignable_to(&param_type) {
                            let mut error_msg = format!(
                                "Type mismatch for argument {} of function '{}': expected {}, got {}",
                                i + 1, name, param_type.to_string(), arg_type.to_string()
                            );

                            match (&arg_type, &param_type) {
                                (Type::Str, Type::Int) => {
                                    error_msg.push_str(". Try converting with to_int()");
                                }
                                (Type::Int, Type::Str) => {
                                    error_msg.push_str(". Try converting with to_str()");
                                }
                                (Type::Str, Type::Float) => {
                                    error_msg.push_str(". Try converting with to_float()");
                                }
                                (Type::Float, Type::Str) => {
                                    error_msg.push_str(". Try converting with to_str()");
                                }
                                _ => {}
                            }

                            return Err(KdnLangError::runtime_error(
                                source.to_string(),
                                filename,
                                (0, 0),
                                error_msg,
                                "Make sure the argument types match the function signature or use explicit type conversion.".to_string(),
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
            let param_types = parameters
                .iter()
                .map(|(_, type_str)| Type::from_string(type_str))
                .collect();

            let return_type_obj = Type::from_string(return_type);

            let func_type = Type::Function {
                params: param_types,
                return_type: Box::new(return_type_obj.clone()),
            };

            env.define_function(name, func_type.clone());

            env.set_current_function_return_type(return_type_obj.clone());

            let mut function_env = env.create_scope();

            for (param_name, param_type_str) in parameters {
                function_env.define_variable(param_name, Type::from_string(param_type_str));
            }

            for node in body {
                typecheck_node(node, &mut function_env, filename, source)?;
            }

            Ok(func_type)
        }

        ASTNode::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond_type = typecheck_node(condition, env, filename, source)?;
            if cond_type != Type::Bool {
                return Err(KdnLangError::runtime_error(
                    source.to_string(),
                    filename,
                    (0, 0),
                    format!(
                        "Type mismatch: If condition must be a boolean, got {}",
                        cond_type.to_string()
                    ),
                    "Make sure the condition evaluates to a boolean value.".to_string(),
                )
                .into());
            }

            let mut then_env = env.create_scope();
            let mut else_env = env.create_scope();

            let mut then_type = Type::Void;
            for node in then_branch {
                then_type = typecheck_node(node, &mut then_env, filename, source)?;
            }

            let mut else_type = Type::Void;
            if let Some(else_nodes) = else_branch {
                for node in else_nodes {
                    else_type = typecheck_node(node, &mut else_env, filename, source)?;
                }
            }

            if then_type == else_type {
                Ok(then_type)
            } else {
                Ok(Type::Void)
            }
        }

        ASTNode::Block(nodes) => {
            let mut block_env = env.create_scope();

            let mut last_type = Type::Void;
            for node in nodes {
                last_type = typecheck_node(node, &mut block_env, filename, source)?;
            }
            Ok(last_type)
        }

        _ => Ok(Type::Any),
    }
}
