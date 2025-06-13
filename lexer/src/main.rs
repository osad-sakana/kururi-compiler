use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LexRequest { code: String }

#[derive(Serialize)]
struct LexResponse { tokens: Vec<String> }

#[post("/lex")]
async fn lex(req: web::Json<LexRequest>) -> impl Responder {
    // とりあえずダミーを返却する
    web::Json(LexResponse { tokens: vec![req.code.clone()] })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(lex))
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
