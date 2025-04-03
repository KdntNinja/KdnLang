use std::boxed::Box;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Number(String),
    Operator(String),
    Identifier(String),
    BooleanLiteral(bool),
    Assignment {
        variable: String,
        type_annotation: String,
        value: Box<ASTNode>,
        type_span: Option<(usize, usize)>,
    },
    Function {
        name: String,
        parameters: Vec<(String, String)>,
        return_type: String,
        body: Vec<ASTNode>,
    },
    Match {
        expression: Box<ASTNode>,
        arms: Vec<(String, ASTNode)>,
    },
    BinaryExpression {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    StringLiteral(String),
    Block(Vec<ASTNode>),
    FunctionCall {
        name: String,
        args: Vec<ASTNode>,
    },
    If {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        else_branch: Option<Vec<ASTNode>>,
    },
    Void,
}
