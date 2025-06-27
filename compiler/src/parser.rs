use crate::error::{CompilerError, CompilerResult};
use crate::token::Token;
use crate::ast::{AstNode, KururiType, BinaryOperator, UnaryOperator};
use std::cell::RefCell;

/// 構文解析器
pub struct Parser {
    state: RefCell<ParserState>,
}

#[derive(Default)]
struct ParserState {
    tokens: Vec<Token>,
    position: usize,
    current_token: Option<Token>,
}

impl Parser {
    /// 新しい構文解析器を作成
    pub fn new() -> Self {
        Self {
            state: RefCell::new(ParserState::default()),
        }
    }

    /// トークンからASTを生成する
    pub fn parse(&self, tokens: &[Token]) -> CompilerResult<AstNode> {
        if tokens.is_empty() {
            return Err(CompilerError::ParseError(
                "No tokens to parse".to_string(),
            ));
        }

        let mut state = self.state.borrow_mut();
        state.tokens = tokens.to_vec();
        state.position = 0;
        state.current_token = state.tokens.get(0).cloned();
        drop(state);

        self.parse_program()
    }

    /// トークンからASTを生成する（旧バージョン互換）
    pub fn parse_tokens(&self, tokens: &[String]) -> CompilerResult<Vec<String>> {
        if tokens.is_empty() {
            return Err(CompilerError::ParseError(
                "No tokens to parse".to_string(),
            ));
        }
        Ok(tokens.to_vec())
    }

    /// プログラム全体を解析
    fn parse_program(&self) -> CompilerResult<AstNode> {
        let mut statements = Vec::new();

        while self.current_token.is_some() && self.current_token != Some(Token::Eof) {
            // 改行をスキップ
            if self.current_token == Some(Token::Newline) {
                self.advance();
                continue;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(AstNode::Program(statements))
    }

    /// 文を解析
    fn parse_statement(&mut self) -> CompilerResult<AstNode> {
        match &self.current_token {
            Some(Token::Function) => self.parse_function_declaration(),
            Some(Token::Class) => self.parse_class_declaration(),
            Some(Token::Let) | Some(Token::Const) => self.parse_variable_declaration(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::For) => self.parse_for_statement(),
            Some(Token::Foreach) => self.parse_foreach_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// 関数宣言を解析
    fn parse_function_declaration(&mut self) -> CompilerResult<AstNode> {
        // 'function' キーワードをスキップ
        self.consume(Token::Function)?;

        // 関数名
        let name = self.parse_identifier()?;

        // '('
        self.consume(Token::LeftParen)?;

        // パラメータリスト
        let mut params = Vec::new();
        while self.current_token != Some(Token::RightParen) {
            let param_name = self.parse_identifier()?;
            self.consume(Token::Colon)?;
            let param_type = self.parse_type()?;
            params.push((param_name, param_type));

            if self.current_token == Some(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        // ')'
        self.consume(Token::RightParen)?;

        // ':'
        self.consume(Token::Colon)?;

        // 戻り値の型
        let return_type = self.parse_type()?;

        // '{'
        self.consume(Token::LeftBrace)?;

        // 関数本体
        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
            if self.current_token == Some(Token::Newline) {
                self.advance();
                continue;
            }
            body.push(self.parse_statement()?);
        }

        // '}'
        self.consume(Token::RightBrace)?;

        Ok(AstNode::FunctionDeclaration {
            name,
            params,
            return_type,
            body,
            is_public: false, // デフォルトはprivate
        })
    }

    /// クラス宣言を解析（簡略化）
    fn parse_class_declaration(&mut self) -> CompilerResult<AstNode> {
        self.consume(Token::Class)?;
        let name = self.parse_identifier()?;
        self.consume(Token::LeftBrace)?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
            if self.current_token == Some(Token::Newline) {
                self.advance();
                continue;
            }

            if self.current_token == Some(Token::Function) || self.current_token == Some(Token::Public) {
                methods.push(self.parse_function_declaration()?);
            } else {
                // フィールド宣言（簡略化）
                let field_name = self.parse_identifier()?;
                self.consume(Token::Colon)?;
                let field_type = self.parse_type()?;
                self.consume(Token::Assign)?;
                let default_value = self.parse_expression()?;
                fields.push((field_name, field_type, default_value));
            }
        }

        self.consume(Token::RightBrace)?;

        Ok(AstNode::ClassDeclaration {
            name,
            fields,
            methods,
        })
    }

    /// 変数宣言を解析
    fn parse_variable_declaration(&mut self) -> CompilerResult<AstNode> {
        let is_const = self.current_token == Some(Token::Const);
        self.advance(); // let or const をスキップ

        let name = self.parse_identifier()?;
        self.consume(Token::Colon)?;
        let var_type = self.parse_type()?;
        self.consume(Token::Assign)?;
        let value = Box::new(self.parse_expression()?);

        Ok(AstNode::VariableDeclaration {
            is_const,
            name,
            var_type,
            value,
        })
    }

    /// if文を解析
    fn parse_if_statement(&mut self) -> CompilerResult<AstNode> {
        self.consume(Token::If)?;
        let condition = Box::new(self.parse_expression()?);
        self.consume(Token::LeftBrace)?;

        let mut then_body = Vec::new();
        while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
            if self.current_token == Some(Token::Newline) {
                self.advance();
                continue;
            }
            then_body.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace)?;

        let mut elseif_branches = Vec::new();
        let mut else_body = None;

        // elseif分岐
        while self.current_token == Some(Token::Elseif) {
            self.advance();
            let elseif_condition = self.parse_expression()?;
            self.consume(Token::LeftBrace)?;
            let mut elseif_body = Vec::new();
            while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
                if self.current_token == Some(Token::Newline) {
                    self.advance();
                    continue;
                }
                elseif_body.push(self.parse_statement()?);
            }
            self.consume(Token::RightBrace)?;
            elseif_branches.push((elseif_condition, elseif_body));
        }

        // else分岐
        if self.current_token == Some(Token::Else) {
            self.advance();
            self.consume(Token::LeftBrace)?;
            let mut body = Vec::new();
            while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
                if self.current_token == Some(Token::Newline) {
                    self.advance();
                    continue;
                }
                body.push(self.parse_statement()?);
            }
            self.consume(Token::RightBrace)?;
            else_body = Some(body);
        }

        Ok(AstNode::IfStatement {
            condition,
            then_body,
            elseif_branches,
            else_body,
        })
    }

    /// while文を解析
    fn parse_while_statement(&mut self) -> CompilerResult<AstNode> {
        self.consume(Token::While)?;
        let condition = Box::new(self.parse_expression()?);
        self.consume(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
            if self.current_token == Some(Token::Newline) {
                self.advance();
                continue;
            }
            body.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace)?;

        Ok(AstNode::WhileStatement { condition, body })
    }

    /// for文を解析
    fn parse_for_statement(&mut self) -> CompilerResult<AstNode> {
        self.consume(Token::For)?;
        let counter_var = self.parse_identifier()?;
        let condition = Box::new(self.parse_expression()?);
        self.consume(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
            if self.current_token == Some(Token::Newline) {
                self.advance();
                continue;
            }
            body.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace)?;

        Ok(AstNode::ForStatement {
            counter_var,
            condition,
            body,
        })
    }

    /// foreach文を解析
    fn parse_foreach_statement(&mut self) -> CompilerResult<AstNode> {
        self.consume(Token::Foreach)?;
        let var_name = self.parse_identifier()?;
        self.consume(Token::In)?;
        let iterable = Box::new(self.parse_expression()?);
        self.consume(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) && self.current_token.is_some() {
            if self.current_token == Some(Token::Newline) {
                self.advance();
                continue;
            }
            body.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace)?;

        Ok(AstNode::ForeachStatement {
            var_name,
            iterable,
            body,
        })
    }

    /// return文を解析
    fn parse_return_statement(&mut self) -> CompilerResult<AstNode> {
        self.consume(Token::Return)?;
        
        // return後に式があるかチェック
        let value = if self.current_token == Some(Token::Newline) || 
                       self.current_token == Some(Token::RightBrace) ||
                       self.current_token == Some(Token::Eof) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        Ok(AstNode::ReturnStatement(value))
    }

    /// 式文を解析
    fn parse_expression_statement(&mut self) -> CompilerResult<AstNode> {
        self.parse_expression()
    }

    /// 式を解析
    fn parse_expression(&mut self) -> CompilerResult<AstNode> {
        self.parse_logical_or()
    }

    /// 論理OR式を解析
    fn parse_logical_or(&mut self) -> CompilerResult<AstNode> {
        let mut left = self.parse_logical_and()?;

        while self.current_token == Some(Token::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 論理AND式を解析
    fn parse_logical_and(&mut self) -> CompilerResult<AstNode> {
        let mut left = self.parse_equality()?;

        while self.current_token == Some(Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 等価性比較を解析
    fn parse_equality(&mut self) -> CompilerResult<AstNode> {
        let mut left = self.parse_comparison()?;

        while let Some(op) = &self.current_token {
            let binary_op = match op {
                Token::Equal => BinaryOperator::Equal,
                Token::NotEqual => BinaryOperator::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator: binary_op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 比較式を解析
    fn parse_comparison(&mut self) -> CompilerResult<AstNode> {
        let mut left = self.parse_term()?;

        while let Some(op) = &self.current_token {
            let binary_op = match op {
                Token::LessThan => BinaryOperator::LessThan,
                Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator: binary_op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 加減算を解析
    fn parse_term(&mut self) -> CompilerResult<AstNode> {
        let mut left = self.parse_factor()?;

        while let Some(op) = &self.current_token {
            let binary_op = match op {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator: binary_op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 乗除算を解析
    fn parse_factor(&mut self) -> CompilerResult<AstNode> {
        let mut left = self.parse_unary()?;

        while let Some(op) = &self.current_token {
            let binary_op = match op {
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator: binary_op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 単項式を解析
    fn parse_unary(&mut self) -> CompilerResult<AstNode> {
        match &self.current_token {
            Some(Token::Not) => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(AstNode::UnaryExpression {
                    operator: UnaryOperator::Not,
                    operand,
                })
            }
            Some(Token::Minus) => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(AstNode::UnaryExpression {
                    operator: UnaryOperator::Minus,
                    operand,
                })
            }
            _ => self.parse_postfix(),
        }
    }

    /// 後置式を解析
    fn parse_postfix(&mut self) -> CompilerResult<AstNode> {
        let mut expr = self.parse_primary()?;

        loop {
            match &self.current_token {
                Some(Token::LeftParen) => {
                    // 関数呼び出し
                    self.advance();
                    let mut args = Vec::new();
                    while self.current_token != Some(Token::RightParen) {
                        args.push(self.parse_expression()?);
                        if self.current_token == Some(Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.consume(Token::RightParen)?;
                    
                    if let AstNode::Identifier(name) = expr {
                        expr = AstNode::FunctionCall { name, args };
                    } else {
                        return Err(CompilerError::ParseError(
                            "Invalid function call".to_string()
                        ));
                    }
                }
                Some(Token::LeftBracket) => {
                    // 配列アクセス
                    self.advance();
                    let index = Box::new(self.parse_expression()?);
                    self.consume(Token::RightBracket)?;
                    expr = AstNode::ArrayAccess {
                        array: Box::new(expr),
                        index,
                    };
                }
                Some(Token::Dot) => {
                    // プロパティアクセス
                    self.advance();
                    let property = self.parse_identifier()?;
                    expr = AstNode::PropertyAccess {
                        object: Box::new(expr),
                        property,
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// 基本式を解析
    fn parse_primary(&mut self) -> CompilerResult<AstNode> {
        match &self.current_token {
            Some(Token::StringLiteral(value)) => {
                let value = value.clone();
                self.advance();
                Ok(AstNode::StringLiteral(value))
            }
            Some(Token::NumberLiteral(value)) => {
                let value = *value;
                self.advance();
                Ok(AstNode::NumberLiteral(value))
            }
            Some(Token::True) => {
                self.advance();
                Ok(AstNode::BooleanLiteral(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(AstNode::BooleanLiteral(false))
            }
            Some(Token::Identifier(_)) => {
                let name = self.parse_identifier()?;
                Ok(AstNode::Identifier(name))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen)?;
                Ok(expr)
            }
            Some(Token::LeftBracket) => {
                // 配列リテラル
                self.advance();
                let mut elements = Vec::new();
                while self.current_token != Some(Token::RightBracket) {
                    elements.push(self.parse_expression()?);
                    if self.current_token == Some(Token::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.consume(Token::RightBracket)?;
                Ok(AstNode::ArrayLiteral(elements))
            }
            Some(Token::New) => {
                self.advance();
                let class_name = self.parse_identifier()?;
                // コンストラクタ引数（簡略化）
                let args = Vec::new();
                Ok(AstNode::NewExpression { class_name, args })
            }
            _ => Err(CompilerError::ParseError(
                format!("Unexpected token: {:?}", self.current_token)
            )),
        }
    }

    /// 型を解析
    fn parse_type(&mut self) -> CompilerResult<KururiType> {
        match &self.current_token {
            Some(Token::StringType) => {
                self.advance();
                Ok(KururiType::String)
            }
            Some(Token::NumberType) => {
                self.advance();
                Ok(KururiType::Number)
            }
            Some(Token::VoidType) => {
                self.advance();
                Ok(KururiType::Void)
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                // 配列型をチェック
                if self.current_token == Some(Token::LeftBracket) {
                    self.advance();
                    self.consume(Token::RightBracket)?;
                    Ok(KururiType::Array(Box::new(KururiType::Class(name))))
                } else {
                    Ok(KururiType::Class(name))
                }
            }
            _ => {
                // 配列型
                let base_type = match &self.current_token {
                    Some(Token::StringType) => {
                        self.advance();
                        KururiType::String
                    }
                    Some(Token::NumberType) => {
                        self.advance();
                        KururiType::Number
                    }
                    _ => return Err(CompilerError::ParseError(
                        "Expected type".to_string()
                    )),
                };

                if self.current_token == Some(Token::LeftBracket) {
                    self.advance();
                    self.consume(Token::RightBracket)?;
                    Ok(KururiType::Array(Box::new(base_type)))
                } else {
                    Ok(base_type)
                }
            }
        }
    }

    /// 識別子を解析
    fn parse_identifier(&mut self) -> CompilerResult<String> {
        match &self.current_token {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(CompilerError::ParseError(
                "Expected identifier".to_string()
            )),
        }
    }

    /// 次のトークンに進む
    fn advance(&mut self) {
        self.position += 1;
        self.current_token = self.tokens.get(self.position).cloned();
    }

    /// 特定のトークンを消費
    fn consume(&mut self, expected: Token) -> CompilerResult<()> {
        if self.current_token == Some(expected.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(CompilerError::ParseError(
                format!("Expected {:?}, found {:?}", expected, self.current_token)
            ))
        }
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
        let mut parser = Parser::new();
        let tokens = vec![
            Token::Function,
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::Colon,
            Token::VoidType,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Eof,
        ];
        let result = parser.parse(&tokens);
        assert!(result.is_ok());
        
        if let Ok(AstNode::Program(statements)) = result {
            assert_eq!(statements.len(), 1);
            if let AstNode::FunctionDeclaration { name, .. } = &statements[0] {
                assert_eq!(name, "main");
            }
        }
    }

    #[test]
    fn test_parse_example_kururi() {
        let mut parser = Parser::new();
        let tokens = vec![
            Token::Function,
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::Colon,
            Token::VoidType,
            Token::LeftBrace,
            Token::Const,
            Token::Identifier("moji".to_string()),
            Token::Colon,
            Token::StringType,
            Token::Assign,
            Token::StringLiteral("Hello World by Kururi!".to_string()),
            Token::Identifier("output".to_string()),
            Token::LeftParen,
            Token::Identifier("moji".to_string()),
            Token::RightParen,
            Token::RightBrace,
            Token::Eof,
        ];
        
        let result = parser.parse(&tokens);
        assert!(result.is_ok());
        
        if let Ok(AstNode::Program(statements)) = result {
            assert_eq!(statements.len(), 1);
            if let AstNode::FunctionDeclaration { name, body, .. } = &statements[0] {
                assert_eq!(name, "main");
                assert_eq!(body.len(), 2); // const宣言とoutput呼び出し
            }
        }
    }

    #[test]
    fn test_parse_empty() {
        let mut parser = Parser::new();
        let result = parser.parse(&[]);
        assert!(result.is_err());
        match result.unwrap_err() {
            CompilerError::ParseError(_) => {},
            _ => panic!("Expected ParseError"),
        }
    }
}