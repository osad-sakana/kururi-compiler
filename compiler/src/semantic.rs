use crate::error::{CompilerError, CompilerResult};
use crate::ast::{AstNode, KururiType};
use std::collections::HashMap;

/// 意味解析器
pub struct SemanticAnalyzer {
    /// 変数のスコープ情報
    scopes: Vec<HashMap<String, KururiType>>,
    /// 関数の型情報
    functions: HashMap<String, (Vec<KururiType>, KururiType)>, // (引数型, 戻り値型)
    /// 現在の関数の戻り値型（return文の型チェック用）
    current_function_return_type: Option<KururiType>,
}

impl SemanticAnalyzer {
    /// 新しい意味解析器を作成
    pub fn new() -> Self {
        let mut analyzer = Self {
            scopes: vec![HashMap::new()], // グローバルスコープ
            functions: HashMap::new(),
            current_function_return_type: None,
        };
        
        // 組み込み関数を登録
        analyzer.functions.insert(
            "output".to_string(),
            (vec![KururiType::String], KururiType::Void)
        );
        
        analyzer
    }

    /// ASTに対して意味解析を行う（新バージョン）
    pub fn analyze_ast(&mut self, ast: &AstNode) -> CompilerResult<AstNode> {
        match ast {
            AstNode::Program(statements) => {
                let mut analyzed_statements = Vec::new();
                for stmt in statements {
                    analyzed_statements.push(self.analyze_ast(stmt)?);
                }
                Ok(AstNode::Program(analyzed_statements))
            }
            
            AstNode::FunctionDeclaration { name, params, return_type, body, is_public } => {
                // 関数を関数テーブルに追加
                let _param_types: Vec<KururiType> = params.iter().map(|(_, t)| t.clone()).collect();
                
                // 関数本体の解析
                let mut analyzed_body = Vec::new();
                for stmt in body {
                    analyzed_body.push(self.analyze_ast(stmt)?);
                }
                
                Ok(AstNode::FunctionDeclaration {
                    name: name.clone(),
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: analyzed_body,
                    is_public: *is_public,
                })
            }
            
            AstNode::VariableDeclaration { is_const, name, var_type, value } => {
                // 値の型をチェック
                let analyzed_value = Box::new(self.analyze_ast(value)?);
                let value_type = self.get_expression_type(value)?;
                
                // 宣言された型と値の型が一致するかチェック
                if !self.types_compatible(var_type, &value_type) {
                    return Err(CompilerError::SemanticError(
                        format!("Type mismatch: expected {}, found {}", var_type, value_type)
                    ));
                }
                
                // 変数を現在のスコープに追加
                if let Some(current_scope) = self.scopes.last_mut() {
                    current_scope.insert(name.clone(), var_type.clone());
                }
                
                Ok(AstNode::VariableDeclaration {
                    is_const: *is_const,
                    name: name.clone(),
                    var_type: var_type.clone(),
                    value: analyzed_value,
                })
            }
            
            AstNode::FunctionCall { name, args } => {
                // 関数が存在するかチェック
                if let Some((param_types, _return_type)) = self.functions.get(name).cloned() {
                    // 引数の数をチェック
                    if args.len() != param_types.len() {
                        return Err(CompilerError::SemanticError(
                            format!("Function {} expects {} arguments, got {}", 
                                   name, param_types.len(), args.len())
                        ));
                    }
                    
                    // 引数の型をチェック
                    let mut analyzed_args = Vec::new();
                    for (i, arg) in args.iter().enumerate() {
                        let analyzed_arg = self.analyze_ast(arg)?;
                        let arg_type = self.get_expression_type(arg)?;
                        let expected_type = &param_types[i];
                        
                        if !self.types_compatible(expected_type, &arg_type) {
                            return Err(CompilerError::SemanticError(
                                format!("Argument {} type mismatch: expected {}, found {}", 
                                       i + 1, expected_type, arg_type)
                            ));
                        }
                        analyzed_args.push(analyzed_arg);
                    }
                    
                    Ok(AstNode::FunctionCall {
                        name: name.clone(),
                        args: analyzed_args,
                    })
                } else {
                    Err(CompilerError::SemanticError(
                        format!("Undefined function: {}", name)
                    ))
                }
            }
            
            AstNode::Identifier(name) => {
                // 変数が定義されているかチェック
                if self.is_variable_defined(name) {
                    Ok(ast.clone())
                } else {
                    Err(CompilerError::SemanticError(
                        format!("Undefined variable: {}", name)
                    ))
                }
            }
            
            // リテラルはそのまま通す
            AstNode::StringLiteral(_) | 
            AstNode::NumberLiteral(_) | 
            AstNode::BooleanLiteral(_) => Ok(ast.clone()),
            
            AstNode::ForStatement { counter_var, condition, body } => {
                // 新しいスコープを作成
                self.scopes.push(std::collections::HashMap::new());
                
                // カウンター変数をスコープに追加
                if let Some(current_scope) = self.scopes.last_mut() {
                    current_scope.insert(counter_var.clone(), KururiType::Number);
                }
                
                // 条件と本体を解析
                let analyzed_condition = Box::new(self.analyze_ast(condition)?);
                let mut analyzed_body = Vec::new();
                for stmt in body {
                    analyzed_body.push(self.analyze_ast(stmt)?);
                }
                
                // スコープを閉じる
                self.scopes.pop();
                
                Ok(AstNode::ForStatement {
                    counter_var: counter_var.clone(),
                    condition: analyzed_condition,
                    body: analyzed_body,
                })
            }
            
            AstNode::IfStatement { condition, then_body, elseif_branches, else_body } => {
                let analyzed_condition = Box::new(self.analyze_ast(condition)?);
                
                let mut analyzed_then_body = Vec::new();
                for stmt in then_body {
                    analyzed_then_body.push(self.analyze_ast(stmt)?);
                }
                
                let analyzed_else_body = if let Some(else_stmts) = else_body {
                    let mut analyzed_else = Vec::new();
                    for stmt in else_stmts {
                        analyzed_else.push(self.analyze_ast(stmt)?);
                    }
                    Some(analyzed_else)
                } else {
                    None
                };
                
                Ok(AstNode::IfStatement {
                    condition: analyzed_condition,
                    then_body: analyzed_then_body,
                    elseif_branches: elseif_branches.clone(), // 簡略化
                    else_body: analyzed_else_body,
                })
            }
            
            AstNode::Assignment { target, value } => {
                // ターゲットが識別子であることをチェック
                if let AstNode::Identifier(var_name) = target.as_ref() {
                    if !self.is_variable_defined(var_name) {
                        return Err(CompilerError::SemanticError(
                            format!("Undefined variable: {}", var_name)
                        ));
                    }
                } else {
                    return Err(CompilerError::SemanticError(
                        "Assignment target must be an identifier".to_string()
                    ));
                }
                
                let analyzed_value = Box::new(self.analyze_ast(value)?);
                
                Ok(AstNode::Assignment {
                    target: target.clone(),
                    value: analyzed_value,
                })
            }
            
            AstNode::BinaryExpression { left, operator, right } => {
                let analyzed_left = Box::new(self.analyze_ast(left)?);
                let analyzed_right = Box::new(self.analyze_ast(right)?);
                
                Ok(AstNode::BinaryExpression {
                    left: analyzed_left,
                    operator: operator.clone(),
                    right: analyzed_right,
                })
            }
            
            // その他のノードも基本的にはそのまま通す（簡略化）
            _ => Ok(ast.clone()),
        }
    }

    /// ASTに対して意味解析を行う（旧バージョン互換）
    pub fn analyze(&self, ast: &[String]) -> CompilerResult<Vec<String>> {
        if ast.is_empty() {
            return Err(CompilerError::SemanticError(
                "No AST to analyze".to_string(),
            ));
        }
        Ok(ast.to_vec())
    }

    /// 式の型を取得
    fn get_expression_type(&self, expr: &AstNode) -> CompilerResult<KururiType> {
        match expr {
            AstNode::StringLiteral(_) => Ok(KururiType::String),
            AstNode::NumberLiteral(_) => Ok(KururiType::Number),
            AstNode::BooleanLiteral(_) => Ok(KururiType::String), // 簡略化
            
            AstNode::Identifier(name) => {
                self.get_variable_type(name)
            }
            
            AstNode::FunctionCall { name, .. } => {
                if let Some((_, return_type)) = self.functions.get(name) {
                    Ok(return_type.clone())
                } else {
                    Err(CompilerError::SemanticError(
                        format!("Undefined function: {}", name)
                    ))
                }
            }
            
            AstNode::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    Ok(KururiType::Array(Box::new(KururiType::String))) // デフォルト
                } else {
                    let first_type = self.get_expression_type(&elements[0])?;
                    Ok(KururiType::Array(Box::new(first_type)))
                }
            }
            
            AstNode::BinaryExpression { left, operator, right } => {
                let left_type = self.get_expression_type(left)?;
                let right_type = self.get_expression_type(right)?;
                
                match operator {
                    crate::ast::BinaryOperator::Add => {
                        // 加算は数値同士なら数値、文字列結合なら文字列
                        if left_type == KururiType::Number && right_type == KururiType::Number {
                            Ok(KururiType::Number)
                        } else {
                            Ok(KururiType::String) // 文字列結合
                        }
                    }
                    crate::ast::BinaryOperator::Subtract |
                    crate::ast::BinaryOperator::Multiply |
                    crate::ast::BinaryOperator::Divide => Ok(KururiType::Number),
                    crate::ast::BinaryOperator::LessThan |
                    crate::ast::BinaryOperator::LessThanOrEqual |
                    crate::ast::BinaryOperator::GreaterThan |
                    crate::ast::BinaryOperator::GreaterThanOrEqual |
                    crate::ast::BinaryOperator::Equal |
                    crate::ast::BinaryOperator::NotEqual => Ok(KururiType::String), // 簡略化：Boolean型の代わり
                    _ => Ok(KururiType::String), // 簡略化
                }
            }
            
            _ => Ok(KururiType::String), // 簡略化
        }
    }

    /// 変数が定義されているかチェック
    fn is_variable_defined(&self, name: &str) -> bool {
        // 内側のスコープから外側に向かって検索
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }

    /// 変数の型を取得
    fn get_variable_type(&self, name: &str) -> CompilerResult<KururiType> {
        // 内側のスコープから外側に向かって検索
        for scope in self.scopes.iter().rev() {
            if let Some(var_type) = scope.get(name) {
                return Ok(var_type.clone());
            }
        }
        Err(CompilerError::SemanticError(
            format!("Undefined variable: {}", name)
        ))
    }

    /// 型の互換性をチェック
    fn types_compatible(&self, expected: &KururiType, actual: &KururiType) -> bool {
        expected == actual
    }

    /// 新しいスコープを開始
    #[allow(dead_code)]
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// 現在のスコープを終了
    #[allow(dead_code)]
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    /// 変数を現在のスコープに追加
    #[allow(dead_code)]
    fn declare_variable(&mut self, name: String, var_type: KururiType) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, var_type);
        }
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

    #[test]
    fn test_analyze_function_call() {
        let mut analyzer = SemanticAnalyzer::new();
        
        // output("hello") をテスト
        let output_call = AstNode::FunctionCall {
            name: "output".to_string(),
            args: vec![AstNode::StringLiteral("hello".to_string())],
        };
        
        let result = analyzer.analyze_ast(&output_call);
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_undefined_function() {
        let mut analyzer = SemanticAnalyzer::new();
        
        // undefined_func() をテスト
        let undefined_call = AstNode::FunctionCall {
            name: "undefined_func".to_string(),
            args: vec![],
        };
        
        let result = analyzer.analyze_ast(&undefined_call);
        assert!(result.is_err());
        match result.unwrap_err() {
            CompilerError::SemanticError(msg) => {
                assert!(msg.contains("Undefined function"));
            },
            _ => panic!("Expected SemanticError"),
        }
    }
}