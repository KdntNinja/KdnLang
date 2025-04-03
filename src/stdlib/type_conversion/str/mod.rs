use crate::error_handling::errors::KdnLangError;
use crate::interpreter::Value;
use crate::parser::ASTNode;

/// Converts a value to a string
pub fn to_str_fn(args: Vec<ASTNode>, values: &[Value]) -> Result<Value, KdnLangError> {
    if args.is_empty() || values.is_empty() {
        return Err(KdnLangError::runtime_error(
            String::new(),
            "unknown",
            (0, 0),
            "to_str() requires one argument".to_string(),
            "Call to_str() with a value to convert to string".to_string(),
        ));
    }

    let value: &Value = &values[0];
    match value {
        Value::Number(n) => {
            if n.fract() == 0.0 {
                Ok(Value::String(format!("{}", n.floor() as i64)))
            } else {
                Ok(Value::String(n.to_string()))
            }
        }
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Bool(b) => Ok(Value::String(b.to_string())),
        Value::Function(_, _) => Ok(Value::String("<function>".to_string())),
        Value::Null => Ok(Value::String("null".to_string())),
    }
}

pub fn to_str_ast_fn(args: Vec<ASTNode>) -> ASTNode {
    if args.len() != 1 {
        return ASTNode::Void;
    }

    ASTNode::StringLiteral("\"\"".to_string())
}
