mod expression;
mod ast;

use pest::Parser;
use pest_derive::Parser;

pub use expression::evaluate_expression_with_vars;
pub use ast::{parse_program, AstNode};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct KdnParser;