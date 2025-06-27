//! Kururi Compiler Library
//! 
//! Kururiプログラミング言語の統合コンパイラライブラリです。
//! 字句解析、構文解析、意味解析、コード生成の全ステップを提供します。
//!
//! # 使用例
//!
//! ```rust
//! use kururi_compiler::Compiler;
//!
//! let compiler = Compiler::new();
//! let result = compiler.compile("function main(): void { output(\"Hello, World!\") }");
//! match result {
//!     Ok(context) => println!("Generated code: {}", context.generated_code),
//!     Err(err) => eprintln!("Compilation error: {}", err),
//! }
//! ```

pub mod types;
pub mod error;
pub mod token;
pub mod ast;
pub mod lexer;
// pub mod parser;
pub mod parser_new;
pub mod semantic;
pub mod codegen;
pub mod compiler;
pub mod handlers;

// 主要な型と関数を再エクスポート
pub use compiler::Compiler;
pub use error::{CompilerError, CompilerResult};
pub use types::{
    CompileContext, CompileRequest, CompileResponse,
    LexRequest, LexResponse,
    ParseRequest, ParseResponse,
    SemanticRequest, SemanticResponse,
    CodegenRequest, CodegenResponse,
};

// HTTPハンドラーを再エクスポート
pub use handlers::{
    lex_handler, parse_handler, semantic_handler,
    codegen_handler, compile_handler,
};