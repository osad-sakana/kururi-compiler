use serde::{Deserialize, Serialize};

/// 字句解析のリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexRequest {
    pub code: String,
}

/// 字句解析のレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct LexResponse {
    pub tokens: Vec<String>,
}

/// 構文解析のリクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct ParseRequest {
    pub tokens: Vec<String>,
}

/// 構文解析のレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct ParseResponse {
    pub ast: Vec<String>,
}

/// 意味解析のリクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticRequest {
    pub ast: Vec<String>,
}

/// 意味解析のレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct SemanticResponse {
    pub checked_ast: Vec<String>,
}

/// コード生成のリクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CodegenRequest {
    pub checked_ast: Vec<String>,
}

/// コード生成のレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct CodegenResponse {
    pub code: String,
}

/// 完全コンパイルのリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileRequest {
    pub code: String,
}

/// 完全コンパイルのレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct CompileResponse {
    pub code: String,
    pub tokens: Vec<String>,
    pub ast: Vec<String>,
    pub checked_ast: Vec<String>,
}

/// コンパイルの中間データを表現する構造体
#[derive(Debug, Clone)]
pub struct CompileContext {
    pub source_code: String,
    pub tokens: Vec<String>,
    pub ast: Vec<String>,
    pub checked_ast: Vec<String>,
    pub generated_code: String,
}