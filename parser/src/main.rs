use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ParseRequest {
    tokens: Vec<String>,
}

#[derive(Serialize)]
struct ParseResponse {
    ast: Vec<String>,
}

#[post("/parse")]
async fn parse(req: web::Json<ParseRequest>) -> impl Responder {
    // とりあえずダミーを返却する
    web::Json(ParseResponse { ast: req.tokens.clone() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(parse))
        .bind("0.0.0.0:5001")?
        .run()
        .await
}
