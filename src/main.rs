use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use database::establish_connection;
use models::{create_user, User};
use serde::{Deserialize, Serialize};

mod database;
mod models;
mod schema;

#[derive(Default, Debug, Serialize, Deserialize)]
struct Hello {
    hello: String,
    fav_numbers: Vec<u32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Hello {
        hello: "world".to_owned(),
        fav_numbers: vec![1, 2, 3],
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut db_connection = establish_connection();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
