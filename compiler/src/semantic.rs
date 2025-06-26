use crate::error::{CompilerError, CompilerResult};

/// 意味解析器
pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    /// 新しい意味解析器を作成
    pub fn new() -> Self {
        Self
    }

    /// ASTに対して意味解析を行う
    pub fn analyze(&self, ast: &[String]) -> CompilerResult<Vec<String>> {
        // TODO: 実際の意味解析ロジックを実装
        // - 型チェック
        // - 変数の宣言・使用チェック
        // - 関数の存在チェック
        // - スコープ管理
        // 現在はダミー実装
        if ast.is_empty() {
            return Err(CompilerError::SemanticError(
                "No AST to analyze".to_string(),
            ));
        }

        // ダミー実装: ASTをそのまま返す
        Ok(ast.to_vec())
    }

    /// 型チェックを行う（将来の実装用）
    #[allow(dead_code)]
    fn type_check(&self, _ast: &[String]) -> CompilerResult<()> {
        // TODO: 型チェックロジックを実装
        Ok(())
    }

    /// 変数の宣言と使用をチェックする（将来の実装用）
    #[allow(dead_code)]
    fn check_variable_usage(&self, _ast: &[String]) -> CompilerResult<()> {
        // TODO: 変数チェックロジックを実装
        Ok(())
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_basic() {
        let analyzer = SemanticAnalyzer::new();
        let ast = vec!["node1".to_string(), "node2".to_string()];
        let result = analyzer.analyze(&ast);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ast);
    }

    #[test]
    fn test_analyze_empty() {
        let analyzer = SemanticAnalyzer::new();
        let result = analyzer.analyze(&[]);
        assert!(result.is_err());
        match result.unwrap_err() {
            CompilerError::SemanticError(_) => {},
            _ => panic!("Expected SemanticError"),
        }
    }
}