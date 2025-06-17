use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SemanticRequest {
    ast: Vec<String>,
}

#[derive(Serialize)]
struct SemanticResponse {
    checked_ast: Vec<String>,
}

#[post("/semantic")]
async fn semantic(req: web::Json<SemanticRequest>) -> impl Responder {
    // とりあえずASTをそのまま返す
    web::Json(SemanticResponse {
        checked_ast: req.ast.clone(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(semantic))
        .bind("0.0.0.0:5002")?
        .run()
        .await
}