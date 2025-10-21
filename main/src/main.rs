
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use my_crates::DB_Connect::DB;
use actix_web::{get, web::{self, post, Data}, App, HttpServer, Responder};
use my_crates::handlers::{add_user};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new()
    .service(
        web::scope("/users")
        .route("/add",post().to(add_user))
    )
    .app_data(DB::db_connect()))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
