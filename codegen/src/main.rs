use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CodegenRequest {
    checked_ast: Vec<String>,
}

#[derive(Serialize)]
struct CodegenResponse {
    code: String,
}

#[post("/codegen")]
async fn codegen(req: web::Json<CodegenRequest>) -> impl Responder {
    // AST 要素から Python コードを組み立てる（ダミー実装）
    let body = req
        .checked_ast
        .iter()
        .map(|node| format!("print(\"{}\")", node))
        .collect::<Vec<_>>()
        .join("\n    ");
    let code = format!(
        "def main():\n    {}\n\nif __name__ == \"__main__\":\n    main()",
        body
    );
    web::Json(CodegenResponse { code })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(codegen))
        .bind("0.0.0.0:5003")?
        .run()
        .await
}
