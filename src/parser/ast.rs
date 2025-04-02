use std::boxed::Box;
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ASTNode {
    Number(String),
    Operator(String),
    Identifier(String),
    Assignment {
        variable: String,
        type_annotation: String,
        value: Box<ASTNode>,
    },
    Function {
        name: String,
        body: Vec<ASTNode>,
    },
    Struct {
        name: String,
        fields: HashMap<String, String>,
    },
    Match {
        expression: Box<ASTNode>,
        arms: Vec<(String, ASTNode)>,
    },
    StringLiteral(String),
    Block(Vec<ASTNode>),
    FunctionCall {
        name: String,
        args: Vec<ASTNode>,
    },
    Void,
}
