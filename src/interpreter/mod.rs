mod environment;
mod evaluator;
mod value;

pub use environment::Environment;
pub use evaluator::{evaluate, EvalContext};
pub use value::Value;

use crate::parser::ASTNode;
use miette::Result;

pub struct Interpreter {
    environment: Environment,
    source_code: String,
    filename: String,
}

impl Interpreter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            source_code: String::new(),
            filename: String::from("unknown"),
        }
    }

    pub fn with_source(source: &str, filename: &str) -> Self {
        Self {
            environment: Environment::new(),
            source_code: source.to_string(),
            filename: filename.to_string(),
        }
    }

    pub fn interpret(&mut self, ast: &ASTNode) -> Result<Value> {
        let ctx: EvalContext = EvalContext::new(self.source_code.clone(), &self.filename);
        evaluate(ast, &mut self.environment, &ctx).map_err(|e| e.into())
    }
}
