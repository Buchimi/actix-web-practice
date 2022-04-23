use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod ano;

use ano::Status;

async fn greet() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Hello".to_string(),
    })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at https://127.0.0.1:8080/");

    HttpServer::new(|| {
        App::new().route("/hello", web::get().to(greet))
        //.route("/hello", web::get().to(|| async { "Hello World!" }))
        //.service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
