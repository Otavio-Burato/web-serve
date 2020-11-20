use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    echo: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn home(req: String) -> impl Responder {
    web::Json(Response { echo: req })
}
