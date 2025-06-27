use crate::error::{CompilerError, CompilerResult};
use crate::ast::{AstNode, KururiType, BinaryOperator, UnaryOperator};

/// コード生成器
pub struct CodeGenerator;

impl CodeGenerator {
    /// 新しいコード生成器を作成
    pub fn new() -> Self {
        Self
    }

    /// チェック済みASTからターゲットコード（Python）を生成する（新バージョン）
    pub fn generate_ast(&self, ast: &AstNode) -> CompilerResult<String> {
        match ast {
            AstNode::Program(statements) => {
                let mut code_sections = Vec::new();
                
                for stmt in statements {
                    let generated = self.generate_ast(stmt)?;
                    if !generated.trim().is_empty() {
                        code_sections.push(generated);
                    }
                }
                
                Ok(code_sections.join("\n\n"))
            }
            
            AstNode::FunctionDeclaration { name, params, body, .. } => {
                self.generate_function_declaration(name, params, body)
            }
            
            AstNode::VariableDeclaration { name, value, .. } => {
                let value_code = self.generate_ast(value)?;
                Ok(format!("{} = {}", name, value_code))
            }
            
            AstNode::FunctionCall { name, args } => {
                self.generate_function_call(name, args)
            }
            
            AstNode::StringLiteral(value) => {
                Ok(format!("\"{}\"", value.replace('\"', "\\\"")))
            }
            
            AstNode::NumberLiteral(value) => {
                Ok(value.to_string())
            }
            
            AstNode::BooleanLiteral(value) => {
                Ok(if *value { "True" } else { "False" }.to_string())
            }
            
            AstNode::Identifier(name) => {
                Ok(name.clone())
            }
            
            AstNode::BinaryExpression { left, operator, right } => {
                let left_code = self.generate_ast(left)?;
                let right_code = self.generate_ast(right)?;
                
                let op_code = match operator {
                    crate::ast::BinaryOperator::Add => "+",
                    crate::ast::BinaryOperator::Subtract => "-",
                    crate::ast::BinaryOperator::Multiply => "*",
                    crate::ast::BinaryOperator::Divide => "/",
                    crate::ast::BinaryOperator::Equal => "==",
                    crate::ast::BinaryOperator::NotEqual => "!=",
                    crate::ast::BinaryOperator::LessThan => "<",
                    crate::ast::BinaryOperator::LessThanOrEqual => "<=",
                    crate::ast::BinaryOperator::GreaterThan => ">",
                    crate::ast::BinaryOperator::GreaterThanOrEqual => ">=",
                    crate::ast::BinaryOperator::And => "and",
                    crate::ast::BinaryOperator::Or => "or",
                };
                
                // 文字列結合の場合、数値を文字列に変換
                if matches!(operator, crate::ast::BinaryOperator::Add) {
                    Ok(format!("str({}) {} str({})", left_code, op_code, right_code))
                } else {
                    Ok(format!("{} {} {}", left_code, op_code, right_code))
                }
            }
            
            AstNode::UnaryExpression { operator, operand } => {
                let operand_code = self.generate_ast(operand)?;
                let op_code = self.generate_unary_operator(operator);
                Ok(format!("{}{}", op_code, operand_code))
            }
            
            AstNode::ArrayLiteral(elements) => {
                let element_codes: Result<Vec<_>, _> = elements
                    .iter()
                    .map(|elem| self.generate_ast(elem))
                    .collect();
                Ok(format!("[{}]", element_codes?.join(", ")))
            }
            
            AstNode::ArrayAccess { array, index } => {
                let array_code = self.generate_ast(array)?;
                let index_code = self.generate_ast(index)?;
                Ok(format!("{}[{}]", array_code, index_code))
            }
            
            AstNode::IfStatement { condition, then_body, elseif_branches, else_body } => {
                self.generate_if_statement(condition, then_body, elseif_branches, else_body)
            }
            
            AstNode::WhileStatement { condition, body } => {
                let condition_code = self.generate_ast(condition)?;
                let body_code = self.generate_statements_body(body)?;
                Ok(format!("while {}:\n{}", condition_code, body_code))
            }
            
            AstNode::ForStatement { counter_var, condition, body } => {
                // Pythonのfor range loop風に変換
                // for i < 9 → for i in range(9)
                let body_code = self.generate_statements_body(body)?;
                if let AstNode::BinaryExpression { left: _, operator: crate::ast::BinaryOperator::LessThan, right } = condition.as_ref() {
                    if let AstNode::NumberLiteral(limit) = right.as_ref() {
                        return Ok(format!("for {} in range(int({})):\n{}", counter_var, limit, body_code));
                    }
                }
                // Fallback
                Ok(format!("for {} in range(10):\n{}", counter_var, body_code))
            }
            
            AstNode::Assignment { target, value } => {
                let target_code = self.generate_ast(target)?;
                let value_code = self.generate_ast(value)?;
                Ok(format!("{} = {}", target_code, value_code))
            }
            
            
            AstNode::ReturnStatement(value) => {
                if let Some(val) = value {
                    let value_code = self.generate_ast(val)?;
                    Ok(format!("return {}", value_code))
                } else {
                    Ok("return".to_string())
                }
            }
            
            _ => {
                // 未実装のノードは空文字列を返す
                Ok(String::new())
            }
        }
    }

    /// チェック済みASTからターゲットコード（Python）を生成する（旧バージョン互換）
    pub fn generate(&self, checked_ast: &[String]) -> CompilerResult<String> {
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

    /// 関数宣言を生成する
    fn generate_function_declaration(&self, name: &str, params: &[(String, KururiType)], body: &[AstNode]) -> CompilerResult<String> {
        let param_names: Vec<String> = params.iter().map(|(name, _)| name.clone()).collect();
        let params_str = param_names.join(", ");
        
        let body_code = self.generate_statements_body(body)?;
        
        Ok(format!("def {}({}):\n{}", name, params_str, body_code))
    }
    
    /// 関数呼び出しを生成する
    fn generate_function_call(&self, name: &str, args: &[AstNode]) -> CompilerResult<String> {
        // output関数の特別処理
        if name == "output" {
            if args.len() == 1 {
                let arg_code = self.generate_ast(&args[0])?;
                return Ok(format!("print({})", arg_code));
            }
        }
        
        let arg_codes: Result<Vec<_>, _> = args
            .iter()
            .map(|arg| self.generate_ast(arg))
            .collect();
        
        Ok(format!("{}({})", name, arg_codes?.join(", ")))
    }
    
    /// 文のブロックを生成する
    fn generate_statements_body(&self, statements: &[AstNode]) -> CompilerResult<String> {
        if statements.is_empty() {
            return Ok("    pass".to_string());
        }
        
        let mut body_lines = Vec::new();
        for stmt in statements {
            let stmt_code = self.generate_ast(stmt)?;
            if !stmt_code.trim().is_empty() {
                // 各行にインデントを追加
                for line in stmt_code.lines() {
                    if !line.trim().is_empty() {
                        body_lines.push(format!("    {}", line));
                    }
                }
            }
        }
        
        if body_lines.is_empty() {
            Ok("    pass".to_string())
        } else {
            Ok(body_lines.join("\n"))
        }
    }
    
    /// if文を生成する
    fn generate_if_statement(&self, condition: &AstNode, then_body: &[AstNode], elseif_branches: &[(AstNode, Vec<AstNode>)], else_body: &Option<Vec<AstNode>>) -> CompilerResult<String> {
        let condition_code = self.generate_ast(condition)?;
        let then_code = self.generate_statements_body(then_body)?;
        
        let mut code = format!("if {}:\n{}", condition_code, then_code);
        
        // elseif分岐
        for (elseif_condition, elseif_body) in elseif_branches {
            let elseif_condition_code = self.generate_ast(elseif_condition)?;
            let elseif_body_code = self.generate_statements_body(elseif_body)?;
            code.push_str(&format!("\nelif {}:\n{}", elseif_condition_code, elseif_body_code));
        }
        
        // else分岐
        if let Some(else_statements) = else_body {
            let else_code = self.generate_statements_body(else_statements)?;
            code.push_str(&format!("\nelse:\n{}", else_code));
        }
        
        Ok(code)
    }
    
    /// 二項演算子を生成する
    fn generate_binary_operator(&self, operator: &BinaryOperator) -> &'static str {
        match operator {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::And => "and",
            BinaryOperator::Or => "or",
        }
    }
    
    /// 単項演算子を生成する
    fn generate_unary_operator(&self, operator: &UnaryOperator) -> &'static str {
        match operator {
            UnaryOperator::Not => "not ",
            UnaryOperator::Minus => "-",
        }
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

    #[test]
    fn test_generate_ast_function_call() {
        let generator = CodeGenerator::new();
        
        // output("Hello World") をテスト
        let output_call = AstNode::FunctionCall {
            name: "output".to_string(),
            args: vec![AstNode::StringLiteral("Hello World".to_string())],
        };
        
        let result = generator.generate_ast(&output_call);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert_eq!(code, "print(\"Hello World\")");
    }

    #[test]
    fn test_generate_ast_main_function() {
        let generator = CodeGenerator::new();
        
        // function main(): void { const moji: string = "Hello World by Kururi!" output(moji) }
        let const_declaration = AstNode::VariableDeclaration {
            is_const: true,
            name: "moji".to_string(),
            var_type: KururiType::String,
            value: Box::new(AstNode::StringLiteral("Hello World by Kururi!".to_string())),
        };
        
        let output_call = AstNode::FunctionCall {
            name: "output".to_string(),
            args: vec![AstNode::Identifier("moji".to_string())],
        };
        
        let main_function = AstNode::FunctionDeclaration {
            name: "main".to_string(),
            params: vec![],
            return_type: KururiType::Void,
            body: vec![const_declaration, output_call],
            is_public: false,
        };
        
        let program = AstNode::Program(vec![main_function]);
        
        let result = generator.generate_ast(&program);
        assert!(result.is_ok());
        let code = result.unwrap();
        
        // 生成されたコードの確認
        assert!(code.contains("def main():"));
        assert!(code.contains("moji = \"Hello World by Kururi!\""));
        assert!(code.contains("print(moji)"));
    }

    #[test]
    fn test_generate_ast_literals() {
        let generator = CodeGenerator::new();
        
        // 文字列リテラル
        let string_result = generator.generate_ast(&AstNode::StringLiteral("test".to_string()));
        assert_eq!(string_result.unwrap(), "\"test\"");
        
        // 数値リテラル
        let number_result = generator.generate_ast(&AstNode::NumberLiteral(42.0));
        assert_eq!(number_result.unwrap(), "42");
        
        // 真偽値リテラル
        let bool_true_result = generator.generate_ast(&AstNode::BooleanLiteral(true));
        assert_eq!(bool_true_result.unwrap(), "True");
        
        let bool_false_result = generator.generate_ast(&AstNode::BooleanLiteral(false));
        assert_eq!(bool_false_result.unwrap(), "False");
        
        // 識別子
        let identifier_result = generator.generate_ast(&AstNode::Identifier("variable".to_string()));
        assert_eq!(identifier_result.unwrap(), "variable");
    }
}