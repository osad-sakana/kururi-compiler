use std::fmt;

/// コンパイラエラーの種類
#[derive(Debug, Clone)]
pub enum CompilerError {
    /// 字句解析エラー
    LexError(String),
    /// 構文解析エラー
    ParseError(String),
    /// 意味解析エラー
    SemanticError(String),
    /// コード生成エラー
    CodegenError(String),
    /// 内部エラー
    InternalError(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::LexError(msg) => write!(f, "Lexical analysis error: {}", msg),
            CompilerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CompilerError::SemanticError(msg) => write!(f, "Semantic analysis error: {}", msg),
            CompilerError::CodegenError(msg) => write!(f, "Code generation error: {}", msg),
            CompilerError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CompilerError {}

/// コンパイラの結果型
pub type CompilerResult<T> = Result<T, CompilerError>;

/// エラーを JSON レスポンス用の構造体に変換
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_type: String,
    pub details: Option<String>,
    pub suggestions: Vec<String>,
}

impl From<CompilerError> for ErrorResponse {
    fn from(error: CompilerError) -> Self {
        let (error_type, details, suggestions) = match &error {
            CompilerError::LexError(msg) => {
                let suggestions = if msg.contains("Unexpected character") {
                    vec!["Check for typos in operators and symbols".to_string()]
                } else if msg.contains("Unterminated string") {
                    vec!["Make sure string literals are properly closed with quotes".to_string()]
                } else {
                    vec!["Check the syntax of your Kururi code".to_string()]
                };
                ("lexical_error", Some("Error occurred during tokenization".to_string()), suggestions)
            },
            CompilerError::ParseError(msg) => {
                let suggestions = if msg.contains("Unexpected token") {
                    vec!["Check the syntax near the highlighted token".to_string()]
                } else {
                    vec!["Verify that your code follows Kururi syntax rules".to_string()]
                };
                ("parse_error", Some("Error occurred during syntax analysis".to_string()), suggestions)
            },
            CompilerError::SemanticError(msg) => {
                let suggestions = if msg.contains("Undefined variable") {
                    vec!["Make sure the variable is declared before use".to_string()]
                } else if msg.contains("Undefined function") {
                    vec!["Check function name spelling and make sure it exists".to_string()]
                } else if msg.contains("Type mismatch") {
                    vec!["Check that variable types match their assigned values".to_string()]
                } else {
                    vec!["Review variable declarations and function calls".to_string()]
                };
                ("semantic_error", Some("Error occurred during semantic analysis".to_string()), suggestions)
            },
            CompilerError::CodegenError(_) => {
                ("codegen_error", Some("Error occurred during code generation".to_string()), 
                 vec!["This is likely an internal error, please report it".to_string()])
            },
            CompilerError::InternalError(_) => {
                ("internal_error", Some("An unexpected internal error occurred".to_string()), 
                 vec!["Please report this issue with your source code".to_string()])
            },
        };
        
        ErrorResponse {
            error: error.to_string(),
            error_type: error_type.to_string(),
            details,
            suggestions,
        }
    }
}