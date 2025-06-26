use crate::error::{CompilerError, CompilerResult};

/// 字句解析器
pub struct Lexer;

impl Lexer {
    /// 新しい字句解析器を作成
    pub fn new() -> Self {
        Self
    }

    /// ソースコードをトークンに分割する
    pub fn tokenize(&self, source_code: &str) -> CompilerResult<Vec<String>> {
        // TODO: 実際の字句解析ロジックを実装
        // 現在はダミー実装
        if source_code.is_empty() {
            return Err(CompilerError::LexError(
                "Empty source code".to_string(),
            ));
        }

        // ダミー実装: ソースコードをそのまま単一トークンとして返す
        Ok(vec![source_code.to_string()])
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let lexer = Lexer::new();
        let result = lexer.tokenize("hello world");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello world"]);
    }

    #[test]
    fn test_tokenize_empty() {
        let lexer = Lexer::new();
        let result = lexer.tokenize("");
        assert!(result.is_err());
    }
}