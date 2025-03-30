use super::types::DataType;

/// Abstract Syntax Tree node types representing program structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ASTNode {
    /// Function definition with name, parameters, return type, and body statements
    Function {
        /// Function name
        name: String,
        /// Function parameters (name, type)
        params: Vec<(String, DataType)>,
        /// Function return type
        return_type: Option<DataType>,
        /// Function body
        body: Vec<ASTNode>,
    },

    /// Variable declaration with name, type, and value
    Variable {
        /// Variable name
        name: String,
        /// Variable type
        data_type: DataType,
        /// Initial value
        value: Box<ASTNode>,
    },

    /// Print statement with expression to output
    Print {
        /// Expression to print
        expression: Box<ASTNode>,
    },

    /// Return statement with value to return
    Return {
        /// Value to return
        value: Box<ASTNode>,
    },

    /// If statement with condition, then block, and optional else block
    IfStatement {
        /// Condition expression
        condition: Box<ASTNode>,
        /// Then block statements
        then_block: Vec<ASTNode>,
        /// Optional else block statements
        else_block: Option<Vec<ASTNode>>,
    },

    /// Match statement with expression and arms
    MatchStatement {
        /// Expression to match on
        expression: Box<ASTNode>,
        /// Match arms (pattern, statements)
        arms: Vec<(MatchPattern, Vec<ASTNode>)>,
    },

    /// Binary operation
    BinaryOp {
        /// Operator type
        op: String,
        /// Left operand
        left: Box<ASTNode>,
        /// Right operand
        right: Box<ASTNode>,
        /// Result type
        result_type: DataType,
    },

    /// Numeric literal value
    Number(i32),

    /// Floating point literal value
    Float(f64),

    /// String literal value
    String(String),

    /// Boolean literal value
    Boolean(bool),

    /// Variable or function reference
    Identifier {
        /// Variable or function name
        name: String,
        /// Inferred type (filled in by type checker)
        inferred_type: Option<DataType>,
    },

    /// Function call
    FunctionCall {
        /// Function name
        name: String,
        /// Arguments
        args: Vec<ASTNode>,
        /// Return type (filled in by type checker)
        return_type: Option<DataType>,
    },
}

/// Match pattern for match statements
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MatchPattern {
    /// Range pattern (inclusive)
    Range(i32, i32),
    /// Wildcard pattern
    Wildcard,
    /// Literal value pattern
    Literal(ASTNode),
}
