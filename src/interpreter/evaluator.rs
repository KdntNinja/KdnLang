use crate::error_handling::errors::KdnLangError;
use crate::interpreter::{Environment, Value};
use crate::parser::ASTNode;
use crate::stdlib::type_conversion::{bool, float, int, str};
use miette::SourceSpan;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct EvalContext {
    pub source_code: String,
    pub filename: String,
    pub current_position: (usize, usize),
}

impl EvalContext {
    pub fn new(source_code: String, filename: &str) -> Self {
        Self {
            source_code,
            filename: filename.to_string(),
            current_position: (0, 0),
        }
    }

    pub fn span(&self) -> SourceSpan {
        self.current_position.into()
    }
}

fn type_of_value(value: &Value) -> &'static str {
    match value {
        Value::Number(n) => {
            if n.fract() == 0.0 {
                "int"
            } else {
                "float"
            }
        }
        Value::String(_) => "str",
        Value::Bool(_) => "bool",
        Value::Function(_, _) => "function",
        Value::Null => "null",
    }
}

fn is_compatible(expected: &str, actual: &str) -> bool {
    if expected == actual || expected == "any" {
        return true;
    }

    match (expected, actual) {
        ("int", "number") | ("float", "number") => true,
        ("float", "int") => true,
        _ => false,
    }
}

pub fn evaluate(
    node: &ASTNode,
    env: &mut Environment,
    ctx: &EvalContext,
) -> Result<Value, KdnLangError> {
    match node {
        ASTNode::Block(nodes) => {
            let mut result: Value = Value::Null;
            for node in nodes {
                result = evaluate(node, env, ctx)?;
            }
            Ok(result)
        }
        ASTNode::Number(n) => {
            let parsed_result: Result<f64, _> = n.parse::<f64>();
            parsed_result.map(Value::Number).map_err(|_| {
                KdnLangError::runtime_error(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!("Failed to parse number: {}", n),
                    format!("Make sure '{}' is a valid number format.", n),
                )
            })
        }
        ASTNode::StringLiteral(s) => {
            let content: String = s.trim_matches(|c: char| c == '\'' || c == '"').to_string();
            Ok(Value::String(content))
        }
        ASTNode::BooleanLiteral(b) => Ok(Value::Bool(*b)),
        ASTNode::Identifier(name) => {
            if name == "true" {
                return Ok(Value::Bool(true));
            } else if name == "false" {
                return Ok(Value::Bool(false));
            }

            let lookup_result: Option<Value> = env.get(name);
            lookup_result.ok_or_else(|| {
                KdnLangError::runtime_error(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!("Undefined variable: {}", name),
                    format!("Make sure '{}' is defined before using it.", name),
                )
            })
        }
        ASTNode::BinaryExpression {
            left,
            operator,
            right,
        } => {
            let left_value: Value = evaluate(left, env, ctx)?;
            let right_value: Value = evaluate(right, env, ctx)?;

            match (operator.as_str(), &left_value, &right_value) {
                ("+" | "-" | "*" | "/" | "%", Value::Number(a), Value::Number(b)) => {
                    match operator.as_str() {
                        "+" => Ok(Value::Number(a + b)),
                        "-" => Ok(Value::Number(a - b)),
                        "*" => Ok(Value::Number(a * b)),
                        "/" => {
                            if *b == 0.0 {
                                Err(KdnLangError::runtime_error(
                                    ctx.source_code.clone(),
                                    &ctx.filename,
                                    ctx.span(),
                                    "Division by zero".to_string(),
                                    "Cannot divide by zero".to_string(),
                                ))
                            } else {
                                Ok(Value::Number(a / b))
                            }
                        }
                        "%" => {
                            if *b == 0.0 {
                                Err(KdnLangError::runtime_error(
                                    ctx.source_code.clone(),
                                    &ctx.filename,
                                    ctx.span(),
                                    "Modulo by zero".to_string(),
                                    "Cannot compute modulo by zero".to_string(),
                                ))
                            } else {
                                Ok(Value::Number(a % b))
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                ("+", Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),

                ("==", Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
                ("!=", Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),

                ("==", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a == b)),
                ("!=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a != b)),

                ("==", Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
                ("!=", Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),

                ("==", a, b) => Ok(Value::Bool(format!("{:?}", a) == format!("{:?}", b))),
                ("!=", a, b) => Ok(Value::Bool(format!("{:?}", a) != format!("{:?}", b))),

                ("<", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                (">", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                ("<=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                (">=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),

                _ => Err(KdnLangError::runtime_error(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!(
                        "Type error: Cannot apply operator '{}' to values of type {} and {}",
                        operator,
                        type_of_value(&left_value),
                        type_of_value(&right_value)
                    ),
                    "Ensure both operands have compatible types for this operation.".to_string(),
                )),
            }
        }
        ASTNode::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_result: Value = evaluate(condition, env, ctx)?;

            match condition_result {
                Value::Bool(true) => {
                    let mut result: Value = Value::Null;
                    // Create a new environment that inherits from the parent
                    let mut block_env = env.clone();
                    for node in then_branch {
                        // Use the block's environment for evaluating statements
                        result = evaluate(node, &mut block_env, ctx)?;

                        // Copy any new variables from block_env back to parent env
                        for (key, value) in block_env.get_all_vars() {
                            env.define(key.clone(), value.clone());
                        }
                    }
                    Ok(result)
                }
                Value::Bool(false) => {
                    if let Some(else_statements) = else_branch {
                        let mut result: Value = Value::Null;
                        // Create a new environment for else block
                        let mut block_env = env.clone();
                        for node in else_statements {
                            // Use the block's environment for evaluating statements
                            result = evaluate(node, &mut block_env, ctx)?;

                            // Copy any new variables from block_env back to parent env
                            for (key, value) in block_env.get_all_vars() {
                                env.define(key.clone(), value.clone());
                            }
                        }
                        Ok(result)
                    } else {
                        Ok(Value::Null)
                    }
                }
                _ => Err(KdnLangError::runtime_error(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    "Condition must evaluate to a boolean value".to_string(),
                    "The if condition must evaluate to true or false.".to_string(),
                )),
            }
        }
        ASTNode::Assignment {
            variable,
            type_annotation,
            value,
            type_span,
        } => {
            let eval_value: Value = evaluate(value, env, ctx)?;

            let expected_type: &str = type_annotation.as_str();
            let value_type: &str = type_of_value(&eval_value);

            if expected_type != "any"
                && expected_type != value_type
                && !is_compatible(expected_type, value_type)
            {
                let error_span: (usize, usize) = match type_span {
                    Some(span) => *span,
                    None => ctx.current_position,
                };

                let suggestion: Option<String> = match (value_type, expected_type) {
                    ("str", "int") => Some(format!("to_int({})", variable)),
                    ("str", "float") => Some(format!("to_float({})", variable)),
                    ("int", "str") => Some(format!("to_str({})", variable)),
                    ("float", "str") => Some(format!("to_str({})", variable)),
                    ("bool", "str") => Some(format!("to_str({})", variable)),
                    _ => None,
                };

                return Err(KdnLangError::runtime_error(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    error_span,
                    format!(
                        "Type mismatch: Cannot assign value of type {} to variable '{}' of type {}",
                        value_type, variable, expected_type
                    ),
                    format!(
                        "Make sure the types match or use explicit type conversion.{}",
                        suggestion
                            .map(|s| format!(" For example: {}", s))
                            .unwrap_or_default()
                    ),
                ));
            }

            env.define(variable.clone(), eval_value);
            Ok(Value::Null)
        }
        ASTNode::Function {
            name,
            parameters: _,
            return_type: _,
            body,
        } => {
            env.define(
                name.clone(),
                Value::Function((), Box::new(ASTNode::Block(body.clone()))),
            );
            Ok(Value::Null)
        }
        ASTNode::FunctionCall { name, args } => match name.as_str() {
            "to_int" | "to_float" | "to_str" | "to_bool" => {
                let evaluated_args: Result<Vec<Value>, KdnLangError> =
                    args.iter().map(|arg| evaluate(arg, env, ctx)).collect();

                let values: Vec<Value> = evaluated_args?;

                match name.as_str() {
                    "to_int" => int::to_int_fn(args.clone(), &values),
                    "to_float" => float::to_float_fn(args.clone(), &values),
                    "to_str" => str::to_str_fn(args.clone(), &values),
                    "to_bool" => bool::to_bool_fn(args.clone(), &values),
                    _ => unreachable!(),
                }
            }
            "input" => {
                let evaluated_args: Vec<Value> = args
                    .iter()
                    .map(|arg| evaluate(arg, env, ctx).unwrap_or(Value::Null))
                    .collect();

                if let Some(Value::String(prompt)) = evaluated_args.get(0) {
                    print!("{}", prompt);
                }
                io::stdout().flush().unwrap();

                let mut input: String = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed_input: String = input.trim_end().to_string();
                Ok(Value::String(trimmed_input))
            }
            "print" => {
                let evaluated_args: Result<Vec<String>, KdnLangError> = args
                    .iter()
                    .map(|arg| {
                        evaluate(arg, env, ctx).map(|val| match val {
                            Value::String(s) => s,
                            Value::Number(n) => {
                                if n.fract() == 0.0 {
                                    format!("{}", n as i64)
                                } else {
                                    n.to_string()
                                }
                            }
                            Value::Bool(b) => b.to_string(),
                            _ => format!("{:?}", val),
                        })
                    })
                    .collect();

                match evaluated_args {
                    Ok(args_strs) => {
                        println!("{}", args_strs.join(" "));
                        Ok(Value::Null)
                    }
                    Err(e) => Err(e),
                }
            }
            _ => {
                if let Some(builtin) = env.get_builtin_function(name) {
                    let ast_args: Vec<ASTNode> = args.clone();
                    let result: ASTNode = builtin(ast_args);
                    match result {
                        ASTNode::Void => Ok(Value::Null),
                        ASTNode::StringLiteral(s) => {
                            let content: String =
                                s.trim_matches(|c: char| c == '\'' || c == '"').to_string();
                            Ok(Value::String(content))
                        }
                        ASTNode::Number(n) => n.parse::<f64>().map(Value::Number).map_err(|_| {
                            KdnLangError::runtime_error(
                                ctx.source_code.clone(),
                                &ctx.filename,
                                ctx.span(),
                                format!("Invalid number from function: {}", n),
                                "The function returned an invalid number format.".to_string(),
                            )
                        }),
                        ASTNode::BooleanLiteral(b) => Ok(Value::Bool(b)),
                        _ => Err(KdnLangError::runtime_error(
                            ctx.source_code.clone(),
                            &ctx.filename,
                            ctx.span(),
                            format!("Unexpected return from built-in function: {:?}", result),
                            format!("The function '{}' returned an unexpected value type.", name),
                        )),
                    }
                } else if let Some(Value::Function(_, body)) = env.get(name) {
                    evaluate(&body, env, ctx)?;
                    Ok(Value::Null)
                } else {
                    Err(KdnLangError::runtime_error(
                            ctx.source_code.clone(),
                            &ctx.filename,
                            ctx.span(),
                            format!("Undefined function: {}", name),
                            format!(
                                "The function '{}' is not defined. Check for typos or make sure it's imported.",
                                name
                            ),
                        ))
                }
            }
        },
        ASTNode::Operator(_) => Err(KdnLangError::runtime_error(
            ctx.source_code.clone(),
            &ctx.filename,
            ctx.span(),
            "Standalone operator not supported".to_string(),
            "Operators should be used in expressions.".to_string(),
        )),
        _ => Err(KdnLangError::runtime_error(
            ctx.source_code.clone(),
            &ctx.filename,
            ctx.span(),
            format!("Unimplemented node type: {:?}", node),
            "This language feature is not yet implemented.".to_string(),
        )),
    }
}
