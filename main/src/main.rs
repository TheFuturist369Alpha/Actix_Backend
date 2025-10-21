
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use my_crates::DB_Connect::DB;
use actix_web::{get, web::{self, post, Data}, App, HttpServer, Responder};
use my_crates::handlers::{add_user};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_connector=DB::db_connect().await;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(move || App::new()
    .app_data(web::Data::new(db_connector.clone()))
    .service(
        web::scope("/users")
        .route("/add",post().to(add_user))
    ))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
