use serde::{Deserialize, Serialize};
use crate::token::Token;
use crate::ast::AstNode;

/// 字句解析のリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexRequest {
    pub code: String,
}

/// 字句解析のレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct LexResponse {
    pub tokens: Vec<Token>,
}

/// 構文解析のリクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct ParseRequest {
    pub tokens: Vec<Token>,
}

/// 構文解析のレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct ParseResponse {
    pub ast: AstNode,
}

/// 意味解析のリクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticRequest {
    pub ast: AstNode,
}

/// 意味解析のレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct SemanticResponse {
    pub checked_ast: AstNode,
}

/// コード生成のリクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CodegenRequest {
    pub checked_ast: AstNode,
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
    pub tokens: Vec<Token>,
    pub ast: AstNode,
    pub checked_ast: AstNode,
}

/// コンパイルの中間データを表現する構造体
#[derive(Debug, Clone)]
pub struct CompileContext {
    pub source_code: String,
    pub tokens: Vec<Token>,
    pub ast: AstNode,
    pub checked_ast: AstNode,
    pub generated_code: String,
}