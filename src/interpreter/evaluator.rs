use crate::error_handling::errors::RuntimeError;
use crate::interpreter::{Environment, Value};
use crate::parser::ASTNode;
use crate::stdlib; // Add missing import
use miette::SourceSpan;

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
            // Remove quotes from string literals
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
        ASTNode::Assignment {
            variable,
            type_annotation: _,
            value,
        } => {
            let eval_value: Value = evaluate(value, env, ctx)?;
            env.define(variable.clone(), eval_value);
            Ok(Value::Null)
        }
        ASTNode::FunctionCall { name, args } => {
            // Check if it's a built-in function
            let builtin_function: Option<stdlib::BuiltinFunction> = env.get_builtin_function(name);

            if let Some(builtin) = builtin_function {
                let ast_args: Vec<ASTNode> = args.clone();
                let result: ASTNode = builtin(ast_args);
                match result {
                    ASTNode::Void => Ok(Value::Null),
                    ASTNode::StringLiteral(s) => {
                        // Remove quotes from string literals
                        let content: String =
                            s.trim_matches(|c: char| c == '\'' || c == '"').to_string();
                        Ok(Value::String(content))
                    }
                    _ => {
                        let error: RuntimeError = RuntimeError::new(
                            ctx.source_code.clone(),
                            &ctx.filename,
                            ctx.span(),
                            format!("Unexpected return from built-in function: {:?}", result),
                            Some(format!(
                                "The function '{}' returned an unexpected value type.",
                                name
                            )),
                        );
                        Err(error)
                    }
                }
            } else {
                // User-defined functions would be handled here
                let error: RuntimeError = RuntimeError::new(
                    ctx.source_code.clone(),
                    &ctx.filename,
                    ctx.span(),
                    format!("Undefined function: {}", name),
                    Some(format!("The function '{}' is not defined. Check for typos or make sure it's imported.", name)),
                );
                Err(error)
            }
        }
        // Other node types would be implemented here
        _ => {
            let error: RuntimeError = RuntimeError::new(
                ctx.source_code.clone(),
                &ctx.filename,
                ctx.span(),
                format!("Unimplemented node type: {:?}", node),
                Some("This language feature is not yet implemented.".to_string()),
            );
            Err(error)
        }
    }
}
