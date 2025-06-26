use actix_web::{web, App, HttpServer};
use kururi_compiler::{
    lex_handler, parse_handler, semantic_handler,
    codegen_handler, compile_handler,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Kururi Compiler Server starting on http://0.0.0.0:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/lex", web::post().to(lex_handler))
            .route("/parse", web::post().to(parse_handler))
            .route("/semantic", web::post().to(semantic_handler))
            .route("/codegen", web::post().to(codegen_handler))
            .route("/compile", web::post().to(compile_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}