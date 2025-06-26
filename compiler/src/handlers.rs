use actix_web::{web, HttpResponse, Responder};
use crate::compiler::Compiler;
use crate::error::ErrorResponse;
use crate::types::*;

/// 字句解析エンドポイント
pub async fn lex_handler(req: web::Json<LexRequest>) -> impl Responder {
    let compiler = Compiler::new();
    
    match compiler.lex_only(&req.code) {
        Ok(tokens) => HttpResponse::Ok().json(LexResponse { tokens }),
        Err(err) => {
            let error_response: ErrorResponse = err.into();
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

/// 構文解析エンドポイント
pub async fn parse_handler(req: web::Json<ParseRequest>) -> impl Responder {
    let compiler = Compiler::new();
    
    match compiler.parse_only(&req.tokens) {
        Ok(ast) => HttpResponse::Ok().json(ParseResponse { ast }),
        Err(err) => {
            let error_response: ErrorResponse = err.into();
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

/// 意味解析エンドポイント
pub async fn semantic_handler(req: web::Json<SemanticRequest>) -> impl Responder {
    let compiler = Compiler::new();
    
    match compiler.analyze_only(&req.ast) {
        Ok(checked_ast) => HttpResponse::Ok().json(SemanticResponse { checked_ast }),
        Err(err) => {
            let error_response: ErrorResponse = err.into();
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

/// コード生成エンドポイント
pub async fn codegen_handler(req: web::Json<CodegenRequest>) -> impl Responder {
    let compiler = Compiler::new();
    
    match compiler.generate_only(&req.checked_ast) {
        Ok(code) => HttpResponse::Ok().json(CodegenResponse { code }),
        Err(err) => {
            let error_response: ErrorResponse = err.into();
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

/// 完全コンパイルエンドポイント
pub async fn compile_handler(req: web::Json<CompileRequest>) -> impl Responder {
    let compiler = Compiler::new();
    
    match compiler.compile(&req.code) {
        Ok(context) => {
            let response = CompileResponse {
                code: context.generated_code,
                tokens: context.tokens,
                ast: context.ast,
                checked_ast: context.checked_ast,
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