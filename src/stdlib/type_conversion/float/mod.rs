use crate::error_handling::errors::KdnLangError;
use crate::interpreter::Value;
use crate::parser::ASTNode;

/// Converts a value to a float
pub fn to_float_fn(args: Vec<ASTNode>, values: &[Value]) -> Result<Value, KdnLangError> {
    if args.is_empty() || values.is_empty() {
        return Err(KdnLangError::runtime_error(
            String::new(),
            "unknown",
            (0, 0),
            "to_float() requires one argument".to_string(),
            "Call to_float() with a value to convert to float".to_string(),
        ));
    }

    let value: &Value = &values[0];
    match value {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => match s.parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(_) => Err(KdnLangError::runtime_error(
                String::new(),
                "unknown",
                (0, 0),
                format!("Cannot convert string '{}' to float", s),
                "Make sure the string contains a valid number".to_string(),
            )),
        },
        Value::Bool(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        _ => Err(KdnLangError::runtime_error(
            String::new(),
            "unknown",
            (0, 0),
            format!("Cannot convert {:?} to float", value),
            "Only strings, numbers, and booleans can be converted to floats".to_string(),
        )),
    }
}

pub fn to_float_ast_fn(args: Vec<ASTNode>) -> ASTNode {
    if args.len() != 1 {
        return ASTNode::Void;
    }

    ASTNode::Number("0.0".to_string())
}
