use serde::{Deserialize, Serialize};

/// Kururi言語のデータ型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KururiType {
    String,
    Number,
    Void,
    Array(Box<KururiType>),
    Class(String),
}

/// AST (Abstract Syntax Tree) ノード
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    // プログラム全体
    Program(Vec<AstNode>),
    
    // 変数宣言
    VariableDeclaration {
        is_const: bool,
        name: String,
        var_type: KururiType,
        value: Box<AstNode>,
    },
    
    // 関数宣言
    FunctionDeclaration {
        name: String,
        params: Vec<(String, KururiType)>,
        return_type: KururiType,
        body: Vec<AstNode>,
        is_public: bool,
    },
    
    // クラス宣言
    ClassDeclaration {
        name: String,
        fields: Vec<(String, KururiType, AstNode)>, // name, type, default_value
        methods: Vec<AstNode>, // FunctionDeclaration nodes
    },
    
    // 制御文
    IfStatement {
        condition: Box<AstNode>,
        then_body: Vec<AstNode>,
        elseif_branches: Vec<(AstNode, Vec<AstNode>)>, // (condition, body)
        else_body: Option<Vec<AstNode>>,
    },
    
    WhileStatement {
        condition: Box<AstNode>,
        body: Vec<AstNode>,
    },
    
    ForStatement {
        counter_var: String,
        condition: Box<AstNode>,
        body: Vec<AstNode>,
    },
    
    ForeachStatement {
        var_name: String,
        iterable: Box<AstNode>,
        body: Vec<AstNode>,
    },
    
    // 式
    BinaryExpression {
        left: Box<AstNode>,
        operator: BinaryOperator,
        right: Box<AstNode>,
    },
    
    UnaryExpression {
        operator: UnaryOperator,
        operand: Box<AstNode>,
    },
    
    // 関数呼び出し
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    
    // メソッド呼び出し
    MethodCall {
        object: Box<AstNode>,
        method: String,
        args: Vec<AstNode>,
    },
    
    // 配列操作
    ArrayAccess {
        array: Box<AstNode>,
        index: Box<AstNode>,
    },
    
    ArrayLiteral(Vec<AstNode>),
    
    PropertyAccess {
        object: Box<AstNode>,
        property: String,
    },
    
    // 代入
    Assignment {
        target: Box<AstNode>,
        value: Box<AstNode>,
    },
    
    // リテラル
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    
    // 識別子
    Identifier(String),
    
    // return文
    ReturnStatement(Option<Box<AstNode>>),
    
    // new 式
    NewExpression {
        class_name: String,
        args: Vec<AstNode>,
    },
}

/// 二項演算子
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // 算術演算子
    Add,
    Subtract,
    Multiply,
    Divide,
    
    // 比較演算子
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    
    // 論理演算子
    And,
    Or,
}

/// 単項演算子
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Minus,
}

impl std::fmt::Display for KururiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KururiType::String => write!(f, "string"),
            KururiType::Number => write!(f, "number"),
            KururiType::Void => write!(f, "void"),
            KururiType::Array(inner) => write!(f, "{}[]", inner),
            KururiType::Class(name) => write!(f, "{}", name),
        }
    }
}