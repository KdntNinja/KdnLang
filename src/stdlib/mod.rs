use std::collections::HashMap;
use std::sync::OnceLock;

use crate::parser::ASTNode;

// Import modules
mod io;

pub type BuiltinFunction = fn(Vec<ASTNode>) -> ASTNode;

pub fn get_builtins() -> &'static HashMap<String, BuiltinFunction> {
    static BUILTINS: OnceLock<HashMap<String, BuiltinFunction>> = OnceLock::new();
    BUILTINS.get_or_init(|| {
        let mut map = HashMap::new();

        // IO functions
        map.insert("print".to_string(), io::print::print_fn as BuiltinFunction);
        map.insert("input".to_string(), io::input::input_fn as BuiltinFunction);

        // Add more functions as they are implemented

        map
    })
}
