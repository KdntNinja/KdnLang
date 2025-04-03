use crate::error_handling::errors::KdnLangError;
use crate::interpreter::{Environment, Value};
use crate::parser::ASTNode;
use miette::SourceSpan;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct EvalContext {
    pub source_code: String,
    pub filename: String,
    pub current_position: (usize, usize), // (offset, length)
}

impl EvalContext {
    pub fn new(source_code: String, filename: &str) -> Self {
        Self {
            source_code,
            filename: filename.to_string(),
            current_position: (0, 0),
        }
    }

    #[allow(dead_code)]
    pub fn with_position(&self, position: (usize, usize)) -> Self {
        let mut ctx: Self = self.clone();
        ctx.current_position = position;
        ctx
    }

    pub fn span(&self) -> SourceSpan {
        self.current_position.into()
    }
}

// Helper function to get the type of a value as a string
fn type_of_value(value: &Value) -> &'static str {
    match value {
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Bool(_) => "boolean",
        Value::Function(_, _) => "function",
        Value::Null => "null",
    }
}

// Helper function to check if types are compatible
fn is_compatible(expected: &str, actual: &str) -> bool {
    if expected == actual || expected == "any" {
        return true;
    }

    // Allow number to be assigned to int or float
    if (expected == "int" || expected == "float") && actual == "number" {
        return true;
    }
    
    // Existing rule for float compatibility
    if expected == "float" && actual == "number" {
        return true;
    }

    false
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
            // Special handling for boolean literals 'true' and 'false'
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
            let left_value = evaluate(left, env, ctx)?;
            let right_value = evaluate(right, env, ctx)?;

            // Additional runtime type checking
            match (operator.as_str(), &left_value, &right_value) {
                // Enforce numeric operators on numeric types
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
                                    "Division by zero",
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
                                    "Modulo by zero",
                                    "Cannot compute modulo by zero".to_string(),
                                ))
                            } else {
                                Ok(Value::Number(a % b))
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                // String concatenation
                ("+", Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),

                // Allow equality comparisons between any types
                ("==", a, b) => Ok(Value::Bool(format!("{:?}", a) == format!("{:?}", b))),
                ("!=", a, b) => Ok(Value::Bool(format!("{:?}", a) != format!("{:?}", b))),

                // Comparison operators with appropriate types
                ("<", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                (">", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                ("<=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                (">=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),

                // Type error for incompatible operations
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
            let condition_result = evaluate(condition, env, ctx)?;

            match condition_result {
                Value::Bool(true) => {
                    // Execute the then branch
                    let mut result = Value::Null;
                    for node in then_branch {
                        result = evaluate(node, env, ctx)?;
                    }
                    Ok(result)
                }
                Value::Bool(false) => {
                    // Execute the else branch if it exists
                    if let Some(else_statements) = else_branch {
                        let mut result = Value::Null;
                        for node in else_statements {
                            result = evaluate(node, env, ctx)?;
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
        } => {
            let eval_value: Value = evaluate(value, env, ctx)?;

            // Runtime type checking for assignments
            let expected_type = type_annotation.as_str();
            let value_type = type_of_value(&eval_value);

            // Check type compatibility (simple check for demonstration)
            if expected_type != "any"
                && expected_type != value_type
                && !is_compatible(expected_type, value_type)
            {
                return Err(KdnLangError::runtime_error(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!(
                        "Type error: Cannot assign value of type {} to variable '{}' of type {}",
                        value_type, variable, expected_type
                    ),
                    "Make sure the types match or use explicit type conversion.".to_string(),
                )
                .into());
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
            // Updated to use unit type () instead of Vec<String>
            env.define(
                name.clone(),
                Value::Function((), Box::new(ASTNode::Block(body.clone()))),
            );
            Ok(Value::Null)
        }
        ASTNode::FunctionCall { name, args } => {
            if name == "input" {
                let evaluated_args: Vec<Value> = args
                    .iter()
                    .map(|arg| evaluate(arg, env, ctx).unwrap_or(Value::Null))
                    .collect();

                if let Some(Value::String(prompt)) = evaluated_args.get(0) {
                    print!("{}", prompt);
                }
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed_input = input.trim_end().to_string();
                return Ok(Value::String(trimmed_input));
            }

            if name == "print" {
                let evaluated_args: Result<Vec<String>, KdnLangError> = args
                    .iter()
                    .map(|arg| {
                        evaluate(arg, env, ctx).map(|val| match val {
                            Value::String(s) => s,
                            Value::Number(n) => n.to_string(),
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
            } else if let Some(builtin) = env.get_builtin_function(name) {
                let ast_args: Vec<ASTNode> = args.clone();
                let result: ASTNode = builtin(ast_args);
                match result {
                    ASTNode::Void => Ok(Value::Null),
                    ASTNode::StringLiteral(s) => {
                        let content: String =
                            s.trim_matches(|c: char| c == '\'' || c == '"').to_string();
                        Ok(Value::String(content))
                    }
                    _ => Err(KdnLangError::runtime_error(
                        ctx.source_code.clone(),
                        &ctx.filename,
                        ctx.span(),
                        format!("Unexpected return from built-in function: {:?}", result),
                        format!("The function '{}' returned an unexpected value type.", name),
                    )),
                }
            } else if let Some(Value::Function(_, body)) = env.get(name) {
                // Execute the user-defined function
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
        ASTNode::Operator(_) => {
            // Operators should be handled in binary expressions
            Err(KdnLangError::runtime_error(
                ctx.source_code.clone(),
                &ctx.filename,
                ctx.span(),
                "Standalone operator not supported".to_string(),
                "Operators should be used in expressions.".to_string(),
            ))
        }
        _ => Err(KdnLangError::runtime_error(
            ctx.source_code.clone(),
            &ctx.filename,
            ctx.span(),
            format!("Unimplemented node type: {:?}", node),
            "This language feature is not yet implemented.".to_string(),
        )),
    }
}
