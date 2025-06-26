use crate::error::{CompilerError, CompilerResult};
use crate::types::CompileContext;
use crate::{lexer::Lexer, parser::Parser, semantic::SemanticAnalyzer, codegen::CodeGenerator};

/// 統合コンパイラ - 全ステップを管理
pub struct Compiler {
    lexer: Lexer,
    parser: Parser,
    semantic_analyzer: SemanticAnalyzer,
    code_generator: CodeGenerator,
}

impl Compiler {
    /// 新しいコンパイラインスタンスを作成
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(),
            parser: Parser::new(),
            semantic_analyzer: SemanticAnalyzer::new(),
            code_generator: CodeGenerator::new(),
        }
    }

    /// 完全なコンパイルパイプラインを実行
    pub fn compile(&self, source_code: &str) -> CompilerResult<CompileContext> {
        // 1. 字句解析
        let tokens = self.lexer.tokenize(source_code)
            .map_err(|e| CompilerError::LexError(format!("Lexical analysis failed: {}", e)))?;

        // 2. 構文解析
        let ast = self.parser.parse(&tokens)
            .map_err(|e| CompilerError::ParseError(format!("Parsing failed: {}", e)))?;

        // 3. 意味解析
        let checked_ast = self.semantic_analyzer.analyze(&ast)
            .map_err(|e| CompilerError::SemanticError(format!("Semantic analysis failed: {}", e)))?;

        // 4. コード生成
        let generated_code = self.code_generator.generate(&checked_ast)
            .map_err(|e| CompilerError::CodegenError(format!("Code generation failed: {}", e)))?;

        Ok(CompileContext {
            source_code: source_code.to_string(),
            tokens,
            ast,
            checked_ast,
            generated_code,
        })
    }

    /// 字句解析のみ実行
    pub fn lex_only(&self, source_code: &str) -> CompilerResult<Vec<String>> {
        self.lexer.tokenize(source_code)
    }

    /// 構文解析のみ実行
    pub fn parse_only(&self, tokens: &[String]) -> CompilerResult<Vec<String>> {
        self.parser.parse(tokens)
    }

    /// 意味解析のみ実行
    pub fn analyze_only(&self, ast: &[String]) -> CompilerResult<Vec<String>> {
        self.semantic_analyzer.analyze(ast)
    }

    /// コード生成のみ実行
    pub fn generate_only(&self, checked_ast: &[String]) -> CompilerResult<String> {
        self.code_generator.generate(checked_ast)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_full_pipeline() {
        let compiler = Compiler::new();
        let result = compiler.compile("test code");
        assert!(result.is_ok());
        
        let context = result.unwrap();
        assert_eq!(context.source_code, "test code");
        assert!(!context.tokens.is_empty());
        assert!(!context.ast.is_empty());
        assert!(!context.checked_ast.is_empty());
        assert!(context.generated_code.contains("def main():"));
    }

    #[test]
    fn test_lex_only() {
        let compiler = Compiler::new();
        let result = compiler.lex_only("test code");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["test code"]);
    }

    #[test]
    fn test_parse_only() {
        let compiler = Compiler::new();
        let tokens = vec!["token1".to_string()];
        let result = compiler.parse_only(&tokens);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), tokens);
    }
}