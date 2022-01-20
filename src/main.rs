mod models;

use crate::models::Status;
use actix_web::{App, HttpServer, web, Responder};
use std::io;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status : "UP".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Starting server at 127.0.0.1:5000");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))  
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
