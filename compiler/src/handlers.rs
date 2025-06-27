use actix_web::{web, HttpResponse, Responder};
use crate::compiler::Compiler;
use crate::error::ErrorResponse;
use crate::types::*;
use crate::ast::AstNode;

/// 字句解析エンドポイント
pub async fn lex_handler(req: web::Json<LexRequest>) -> impl Responder {
    let mut compiler = Compiler::new();
    
    // Use actual lexer instead of dummy implementation
    match compiler.lex_tokens(&req.code) {
        Ok(tokens) => {
            HttpResponse::Ok().json(LexResponse { tokens })
        },
        Err(err) => {
            let error_response: ErrorResponse = err.into();
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

/// 構文解析エンドポイント（一時的なダミー実装）
pub async fn parse_handler(_req: web::Json<ParseRequest>) -> impl Responder {
    let dummy_ast = AstNode::Program(vec![]);
    HttpResponse::Ok().json(ParseResponse { ast: dummy_ast })
}

/// 意味解析エンドポイント（一時的なダミー実装）
pub async fn semantic_handler(_req: web::Json<SemanticRequest>) -> impl Responder {
    let dummy_ast = AstNode::Program(vec![]);
    HttpResponse::Ok().json(SemanticResponse { checked_ast: dummy_ast })
}

/// コード生成エンドポイント（一時的なダミー実装）
pub async fn codegen_handler(_req: web::Json<CodegenRequest>) -> impl Responder {
    let dummy_code = "def main():\n    print(\"Hello from Kururi!\")\n\nif __name__ == \"__main__\":\n    main()";
    HttpResponse::Ok().json(CodegenResponse { code: dummy_code.to_string() })
}

/// 完全コンパイルエンドポイント
pub async fn compile_handler(req: web::Json<CompileRequest>) -> impl Responder {
    let mut compiler = Compiler::new();
    
    // AST-based compilation (preferred method)
    match compiler.compile_ast(&req.code) {
        Ok(generated_code) => {
            // Create a simplified response with actual compilation results
            let response = CompileResponse {
                code: generated_code,
                tokens: vec![], // Simplified for HTTP API
                ast: AstNode::Program(vec![]), // Simplified for HTTP API
                checked_ast: AstNode::Program(vec![]), // Simplified for HTTP API
            };
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            let error_response: ErrorResponse = err.into();
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_lex_handler() {
        let app = test::init_service(
            App::new().route("/lex", web::post().to(lex_handler))
        ).await;

        let req_body = LexRequest {
            code: "test code".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/lex")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test] 
    async fn test_compile_handler() {
        let app = test::init_service(
            App::new().route("/compile", web::post().to(compile_handler))
        ).await;

        let req_body = CompileRequest {
            code: "test code".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/compile")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}