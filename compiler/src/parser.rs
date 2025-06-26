use crate::error::{CompilerError, CompilerResult};

/// 構文解析器
pub struct Parser;

impl Parser {
    /// 新しい構文解析器を作成
    pub fn new() -> Self {
        Self
    }

    /// トークンからASTを生成する
    pub fn parse(&self, tokens: &[String]) -> CompilerResult<Vec<String>> {
        // TODO: 実際の構文解析ロジックを実装
        // 現在はダミー実装
        if tokens.is_empty() {
            return Err(CompilerError::ParseError(
                "No tokens to parse".to_string(),
            ));
        }

        // ダミー実装: トークンをそのままASTとして返す
        Ok(tokens.to_vec())
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let parser = Parser::new();
        let tokens = vec!["token1".to_string(), "token2".to_string()];
        let result = parser.parse(&tokens);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), tokens);
    }

    #[test]
    fn test_parse_empty() {
        let parser = Parser::new();
        let result = parser.parse(&[]);
        assert!(result.is_err());
        match result.unwrap_err() {
            CompilerError::ParseError(_) => {},
            _ => panic!("Expected ParseError"),
        }
    }
}