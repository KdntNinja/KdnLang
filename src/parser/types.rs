use crate::errors::KdnResult;

/// Available data types in KdnLang
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum DataType {
    /// 32-bit signed integer
    I32,
    /// 64-bit floating point
    F64,
    /// String type
    Str,
    /// Boolean type
    Bool,
    /// User-defined type (struct name)
    Custom(String),
    /// Function return type
    Function {
        /// Parameter types
        params: Vec<DataType>,
        /// Return type (None means void)
        return_type: Option<Box<DataType>>,
    },
    /// Void type (used for expressions with no value)
    None,
}

impl DataType {
    /// Parse a type string into a DataType enum
    pub fn from_str(type_str: &str) -> KdnResult<Self> {
        match type_str {
            "i32" => Ok(DataType::I32),
            "f64" => Ok(DataType::F64),
            "str" => Ok(DataType::Str),
            "bool" => Ok(DataType::Bool),
            "none" => Ok(DataType::None),
            custom => Ok(DataType::Custom(custom.to_string())),
        }
    }

    /// Convert a DataType to its string representation
    pub fn to_string(&self) -> String {
        match self {
            DataType::I32 => "i32".to_string(),
            DataType::F64 => "f64".to_string(),
            DataType::Str => "str".to_string(),
            DataType::Bool => "bool".to_string(),
            DataType::None => "none".to_string(),
            DataType::Custom(name) => name.clone(),
            DataType::Function {
                params,
                return_type,
            } => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                match return_type {
                    Some(ret) => format!("fn({}) -> {}", params_str, ret.to_string()),
                    None => format!("fn({})", params_str),
                }
            }
        }
    }
}
