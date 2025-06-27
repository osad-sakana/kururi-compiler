use crate::error::{CompilerError, CompilerResult};
use crate::token::Token;

/// 字句解析器
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    /// 新しい字句解析器を作成
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            position: 0,
            current_char: None,
        }
    }

    /// ソースコードをトークンに分割する（新バージョン）
    pub fn tokenize(&mut self, source_code: &str) -> CompilerResult<Vec<Token>> {
        if source_code.is_empty() {
            return Err(CompilerError::LexError(
                "Empty source code".to_string(),
            ));
        }

        self.input = source_code.chars().collect();
        self.position = 0;
        self.current_char = self.input.get(0).copied();

        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char {
            match ch {
                // 空白文字をスキップ
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                
                // 改行は重要（セミコロン代わり）
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                }
                
                // コメント（//から行末まで）
                '/' if self.peek() == Some('/') => {
                    self.skip_comment();
                }
                
                // 文字列リテラル
                '"' => {
                    tokens.push(self.read_string()?);
                }
                
                // 数値リテラル
                c if c.is_ascii_digit() => {
                    tokens.push(self.read_number()?);
                }
                
                // 識別子またはキーワード
                c if c.is_ascii_alphabetic() || c == '_' => {
                    tokens.push(self.read_identifier());
                }
                
                // 演算子と記号
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.advance();
                }
                '=' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::Equal);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Assign);
                        self.advance();
                    }
                }
                '!' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::NotEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Not);
                        self.advance();
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::LessThanOrEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::LessThan);
                        self.advance();
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::GreaterThanOrEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::GreaterThan);
                        self.advance();
                    }
                }
                '&' => {
                    if self.peek() == Some('&') {
                        tokens.push(Token::And);
                        self.advance();
                        self.advance();
                    } else {
                        return Err(CompilerError::LexError(
                            format!("Unexpected character: {}", ch)
                        ));
                    }
                }
                '|' => {
                    if self.peek() == Some('|') {
                        tokens.push(Token::Or);
                        self.advance();
                        self.advance();
                    } else {
                        return Err(CompilerError::LexError(
                            format!("Unexpected character: {}", ch)
                        ));
                    }
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.advance();
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.advance();
                }
                
                _ => {
                    return Err(CompilerError::LexError(
                        format!("Unexpected character: {}", ch)
                    ));
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    /// 旧バージョン互換のため（デバッグ用）
    pub fn tokenize_strings(&self, source_code: &str) -> CompilerResult<Vec<String>> {
        if source_code.is_empty() {
            return Err(CompilerError::LexError(
                "Empty source code".to_string(),
            ));
        }

        // 一時的な実装: スペースで分割
        Ok(source_code.split_whitespace().map(|s| s.to_string()).collect())
    }

    /// 次の文字に進む
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    /// 次の文字を覗き見る（位置は進めない）
    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    /// コメントをスキップ
    fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// 文字列リテラルを読み取る
    fn read_string(&mut self) -> CompilerResult<Token> {
        self.advance(); // 開始の " をスキップ
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // 終了の " をスキップ
                return Ok(Token::StringLiteral(value));
            }
            if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => value.push('\n'),
                    Some('t') => value.push('\t'),
                    Some('r') => value.push('\r'),
                    Some('\\') => value.push('\\'),
                    Some('"') => value.push('"'),
                    Some(c) => {
                        return Err(CompilerError::LexError(
                            format!("Invalid escape sequence: \\{}", c)
                        ));
                    }
                    None => {
                        return Err(CompilerError::LexError(
                            "Unexpected end of input in string literal".to_string()
                        ));
                    }
                }
            } else {
                value.push(ch);
            }
            self.advance();
        }

        Err(CompilerError::LexError(
            "Unterminated string literal".to_string()
        ))
    }

    /// 数値リテラルを読み取る
    fn read_number(&mut self) -> CompilerResult<Token> {
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match value.parse::<f64>() {
            Ok(num) => Ok(Token::NumberLiteral(num)),
            Err(_) => Err(CompilerError::LexError(
                format!("Invalid number format: {}", value)
            )),
        }
    }

    /// 識別子またはキーワードを読み取る
    fn read_identifier(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::keyword_or_identifier(&value)
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
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("let x: number = 42");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::NumberType);
        assert_eq!(tokens[4], Token::Assign);
        assert_eq!(tokens[5], Token::NumberLiteral(42.0));
    }

    #[test]
    fn test_tokenize_string_literal() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize(r#"const msg: string = "Hello World""#);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        println!("Tokens: {:?}", tokens);
        assert_eq!(tokens[0], Token::Const);
        // Find the string literal token
        let string_token = tokens.iter().find(|t| matches!(t, Token::StringLiteral(_)));
        assert!(string_token.is_some());
        assert_eq!(*string_token.unwrap(), Token::StringLiteral("Hello World".to_string()));
    }

    #[test]
    fn test_tokenize_function() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("function main(): void {}");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::Function);
        assert_eq!(tokens[1], Token::Identifier("main".to_string()));
        assert_eq!(tokens[2], Token::LeftParen);
        assert_eq!(tokens[3], Token::RightParen);
        assert_eq!(tokens[4], Token::Colon);
        assert_eq!(tokens[5], Token::VoidType);
        assert_eq!(tokens[6], Token::LeftBrace);
        assert_eq!(tokens[7], Token::RightBrace);
    }

    #[test]
    fn test_tokenize_empty() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("");
        assert!(result.is_err());
    }

    #[test]
    fn test_tokenize_comments() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("let x = 5 // this is a comment\nlet y = 10");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        // コメントは無視され、改行トークンが残る
        assert!(tokens.contains(&Token::Newline));
    }

    #[test]
    fn test_tokenize_example_kururi() {
        let mut lexer = Lexer::new();
        let source = r#"function main(): void{
    const moji: string = "Hello World by Kururi!"
    output(moji)
}"#;
        let result = lexer.tokenize(source);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        
        // 主要なトークンが正しく認識されているかチェック
        let expected_tokens = [
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
        ];
        
        // 重要なトークンが含まれているかチェック
        for expected in expected_tokens.iter() {
            assert!(tokens.contains(expected), "Missing token: {:?}", expected);
        }
        
        println!("Tokenized {} tokens:", tokens.len());
        for token in &tokens {
            println!("  {:?}", token);
        }
    }
}