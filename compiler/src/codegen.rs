use crate::error::{CompilerError, CompilerResult};

/// コード生成器
pub struct CodeGenerator;

impl CodeGenerator {
    /// 新しいコード生成器を作成
    pub fn new() -> Self {
        Self
    }

    /// チェック済みASTからターゲットコード（Python）を生成する
    pub fn generate(&self, checked_ast: &[String]) -> CompilerResult<String> {
        // TODO: 実際のコード生成ロジックを実装
        // - AST ノードの種類に応じた適切なPythonコード生成
        // - 変数宣言、関数定義、制御構造の変換
        // - Kururi 固有の構文から Python への変換
        // 現在はダミー実装
        if checked_ast.is_empty() {
            return Err(CompilerError::CodegenError(
                "No AST to generate code from".to_string(),
            ));
        }

        // ダミー実装: AST要素をprintステートメントに変換
        let body = checked_ast
            .iter()
            .map(|node| self.generate_print_statement(node))
            .collect::<Vec<_>>()
            .join("\n    ");

        let code = format!(
            "def main():\n    {}\n\nif __name__ == \"__main__\":\n    main()",
            body
        );

        Ok(code)
    }

    /// print文を生成する（ダミー実装用）
    fn generate_print_statement(&self, content: &str) -> String {
        format!("print(\"{}\")", content.replace('"', "\\\""))
    }

    /// 関数定義を生成する（将来の実装用）
    #[allow(dead_code)]
    fn generate_function(&self, _name: &str, _params: &[String], _body: &[String]) -> CompilerResult<String> {
        // TODO: 関数定義の生成ロジックを実装
        Ok(String::new())
    }

    /// 変数宣言を生成する（将来の実装用）
    #[allow(dead_code)]
    fn generate_variable_declaration(&self, _name: &str, _value: &str) -> CompilerResult<String> {
        // TODO: 変数宣言の生成ロジックを実装
        Ok(String::new())
    }

    /// 式を生成する（将来の実装用）
    #[allow(dead_code)]
    fn generate_expression(&self, _expr: &str) -> CompilerResult<String> {
        // TODO: 式の生成ロジックを実装
        Ok(String::new())
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_basic() {
        let generator = CodeGenerator::new();
        let ast = vec!["Hello World".to_string()];
        let result = generator.generate(&ast);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("print(\"Hello World\")"));
        assert!(code.contains("def main():"));
    }

    #[test]
    fn test_generate_empty() {
        let generator = CodeGenerator::new();
        let result = generator.generate(&[]);
        assert!(result.is_err());
        match result.unwrap_err() {
            CompilerError::CodegenError(_) => {},
            _ => panic!("Expected CodegenError"),
        }
    }

    #[test]
    fn test_generate_multiple_statements() {
        let generator = CodeGenerator::new();
        let ast = vec!["Hello".to_string(), "World".to_string()];
        let result = generator.generate(&ast);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("print(\"Hello\")"));
        assert!(code.contains("print(\"World\")"));
    }
}