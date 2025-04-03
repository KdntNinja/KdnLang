use std::collections::HashMap;
use std::sync::OnceLock;

use crate::parser::ASTNode;

mod io;
pub mod type_conversion;

pub type BuiltinFunction = fn(Vec<ASTNode>) -> ASTNode;

pub fn get_builtins() -> &'static HashMap<String, BuiltinFunction> {
    static BUILTINS: OnceLock<HashMap<String, BuiltinFunction>> = OnceLock::new();
    BUILTINS.get_or_init(|| {
        let mut map: HashMap<String, BuiltinFunction> = HashMap::new();

        // IO functions
        map.insert("print".to_string(), io::print::print_fn as BuiltinFunction);
        map.insert("input".to_string(), io::input::input_fn as BuiltinFunction);

        // Type conversion functions
        map.insert(
            "to_int".to_string(),
            type_conversion::to_int_ast_fn as BuiltinFunction,
        );
        map.insert(
            "to_float".to_string(),
            type_conversion::to_float_ast_fn as BuiltinFunction,
        );
        map.insert(
            "to_str".to_string(),
            type_conversion::to_str_ast_fn as BuiltinFunction,
        );
        map.insert(
            "to_bool".to_string(),
            type_conversion::to_bool_ast_fn as BuiltinFunction,
        );

        map
    })
}
