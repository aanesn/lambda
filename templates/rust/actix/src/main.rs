use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening on http://localhost:8080");
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
