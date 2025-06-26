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
}

impl From<CompilerError> for ErrorResponse {
    fn from(error: CompilerError) -> Self {
        let error_type = match error {
            CompilerError::LexError(_) => "lexical_error",
            CompilerError::ParseError(_) => "parse_error", 
            CompilerError::SemanticError(_) => "semantic_error",
            CompilerError::CodegenError(_) => "codegen_error",
            CompilerError::InternalError(_) => "internal_error",
        };
        
        ErrorResponse {
            error: error.to_string(),
            error_type: error_type.to_string(),
        }
    }
}