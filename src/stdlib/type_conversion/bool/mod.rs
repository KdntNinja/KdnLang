use crate::error_handling::errors::KdnLangError;
use crate::interpreter::Value;
use crate::parser::ASTNode;

pub fn to_bool_fn(args: Vec<ASTNode>, values: &[Value]) -> Result<Value, KdnLangError> {
    if args.is_empty() || values.is_empty() {
        return Err(KdnLangError::runtime_error(
            String::new(),
            "unknown",
            (0, 0),
            "to_bool() requires one argument".to_string(),
            "Call to_bool() with a value to convert to boolean".to_string(),
        ));
    }

    let value: &Value = &values[0];
    match value {
        Value::Number(n) => Ok(Value::Bool(*n != 0.0)),
        Value::String(s) => {
            let lowercased: String = s.to_lowercase();
            if lowercased == "true" || lowercased == "1" || lowercased == "yes" {
                Ok(Value::Bool(true))
            } else if lowercased == "false"
                || lowercased == "0"
                || lowercased == "no"
                || s.is_empty()
            {
                Ok(Value::Bool(false))
            } else {
                Err(KdnLangError::runtime_error(
                    String::new(),
                    "unknown",
                    (0, 0),
                    format!("Cannot convert string '{}' to boolean", s),
                    "String must be 'true', 'false', '1', '0', 'yes', 'no', or empty".to_string(),
                ))
            }
        }
        Value::Bool(b) => Ok(Value::Bool(*b)),
        Value::Function(_, _) => Ok(Value::Bool(true)),
        Value::Null => Ok(Value::Bool(false)),
    }
}

pub fn to_bool_ast_fn(args: Vec<ASTNode>) -> ASTNode {
    if args.len() != 1 {
        return ASTNode::Void;
    }

    ASTNode::BooleanLiteral(false)
}
