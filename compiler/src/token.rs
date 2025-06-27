use serde::{Deserialize, Serialize};

/// Kururi言語のトークン
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // キーワード
    Const,
    Let,
    Function,
    Class,
    Public,
    If,
    Elseif,
    Else,
    While,
    For,
    Foreach,
    In,
    Return,
    New,
    True,
    False,
    
    // 型
    StringType,
    NumberType,
    VoidType,
    
    // 識別子とリテラル
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    
    // 演算子
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Assign,         // =
    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    LessThanOrEqual, // <=
    GreaterThan,    // >
    GreaterThanOrEqual, // >=
    And,            // &&
    Or,             // ||
    Not,            // !
    
    // 区切り文字
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Colon,          // :
    Dot,            // .
    
    // 特殊
    Newline,        // 改行（セミコロン代わり）
    Eof,            // ファイル終端
}

impl Token {
    /// キーワードの識別
    pub fn keyword_or_identifier(s: &str) -> Token {
        match s {
            "const" => Token::Const,
            "let" => Token::Let,
            "function" => Token::Function,
            "class" => Token::Class,
            "public" => Token::Public,
            "if" => Token::If,
            "elseif" => Token::Elseif,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "foreach" => Token::Foreach,
            "in" => Token::In,
            "return" => Token::Return,
            "new" => Token::New,
            "true" => Token::True,
            "false" => Token::False,
            "string" => Token::StringType,
            "number" => Token::NumberType,
            "void" => Token::VoidType,
            _ => Token::Identifier(s.to_string()),
        }
    }
    
    /// トークンの表示用文字列
    pub fn as_str(&self) -> &'static str {
        match self {
            Token::Const => "const",
            Token::Let => "let",
            Token::Function => "function",
            Token::Class => "class",
            Token::Public => "public",
            Token::If => "if",
            Token::Elseif => "elseif",
            Token::Else => "else",
            Token::While => "while",
            Token::For => "for",
            Token::Foreach => "foreach",
            Token::In => "in",
            Token::Return => "return",
            Token::New => "new",
            Token::True => "true",
            Token::False => "false",
            Token::StringType => "string",
            Token::NumberType => "number",
            Token::VoidType => "void",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Multiply => "*",
            Token::Divide => "/",
            Token::Assign => "=",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::LessThan => "<",
            Token::LessThanOrEqual => "<=",
            Token::GreaterThan => ">",
            Token::GreaterThanOrEqual => ">=",
            Token::And => "&&",
            Token::Or => "||",
            Token::Not => "!",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::LeftBracket => "[",
            Token::RightBracket => "]",
            Token::Comma => ",",
            Token::Colon => ":",
            Token::Dot => ".",
            Token::Newline => "\\n",
            Token::Eof => "EOF",
            _ => "",
        }
    }
}