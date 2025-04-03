use crate::error_handling::errors::RuntimeError;
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

pub fn evaluate(
    node: &ASTNode,
    env: &mut Environment,
    ctx: &EvalContext,
) -> Result<Value, RuntimeError> {
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
                RuntimeError::new(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!("Failed to parse number: {}", n),
                    Some(format!("Make sure '{}' is a valid number format.", n)),
                )
            })
        }
        ASTNode::StringLiteral(s) => {
            let content: String = s.trim_matches(|c: char| c == '\'' || c == '"').to_string();
            Ok(Value::String(content))
        }
        ASTNode::Identifier(name) => {
            let lookup_result: Option<Value> = env.get(name);
            lookup_result.ok_or_else(|| {
                RuntimeError::new(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!("Undefined variable: {}", name),
                    Some(format!("Make sure '{}' is defined before using it.", name)),
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

            match (operator.as_str(), &left_value, &right_value) {
                ("==", Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
                ("==", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a == b)),
                ("!=", Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
                ("!=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a != b)),
                ("<", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                (">", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                ("<=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                (">=", Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(RuntimeError::new(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!(
                        "Unsupported operation: {:?} {} {:?}",
                        left_value, operator, right_value
                    ),
                    Some("Make sure the operator is supported for these value types.".to_string()),
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
                _ => Err(RuntimeError::new(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    "Condition must evaluate to a boolean value".to_string(),
                    Some("The if condition must evaluate to true or false.".to_string()),
                )),
            }
        }
        ASTNode::Assignment {
            variable,
            type_annotation: _,
            value,
        } => {
            let eval_value: Value = evaluate(value, env, ctx)?;
            env.define(variable.clone(), eval_value);
            Ok(Value::Null)
        }
        ASTNode::Function { name, body } => {
            // Wrap the body in a Box<ASTNode> and store the function in the environment
            env.define(
                name.clone(),
                Value::Function(vec![], Box::new(ASTNode::Block(body.clone()))),
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
                let evaluated_args: Result<Vec<String>, RuntimeError> = args
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
                    _ => Err(RuntimeError::new(
                        ctx.source_code.clone(),
                        &ctx.filename,
                        ctx.span(),
                        format!("Unexpected return from built-in function: {:?}", result),
                        Some(format!(
                            "The function '{}' returned an unexpected value type.",
                            name
                        )),
                    )),
                }
            } else if let Some(Value::Function(_, body)) = env.get(name) {
                // Execute the user-defined function
                evaluate(&body, env, ctx)?;
                Ok(Value::Null)
            } else {
                Err(RuntimeError::new(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!("Undefined function: {}", name),
                    Some(format!(
                        "The function '{}' is not defined. Check for typos or make sure it's imported.",
                        name
                    )),
                ))
            }
        }
        ASTNode::Operator(_) => {
            // Operators should be handled in binary expressions
            Err(RuntimeError::new(
                ctx.source_code.clone(),
                &ctx.filename,
                ctx.span(),
                "Standalone operator not supported".to_string(),
                Some("Operators should be used in expressions.".to_string()),
            ))
        }
        _ => Err(RuntimeError::new(
            ctx.source_code.clone(),
            &ctx.filename,
            ctx.span(),
            format!("Unimplemented node type: {:?}", node),
            Some("This language feature is not yet implemented.".to_string()),
        )),
    }
}
