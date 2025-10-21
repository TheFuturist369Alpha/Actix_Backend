
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use my_crates::DB_Connect::DB;
use actix_web::{get, web::{self, post, Data}, App, HttpServer, Responder};
use my_crates::handlers::{add_user};



struct AppConf{
    api_key: String,
    database: String,
    debug: bool
}

struct StateCounter{
    count: u32
}

#[get("/app-info")]
async fn get_app(config: Data<Arc<AppConf>>) ->impl Responder{
    format!("database {}, debug {}", config.database, config.debug)
}

#[get("/app-counter")]
async fn get_counter(data: web::Data<Arc<TokioMutex<StateCounter>>>) ->impl Responder{
   let mut counter=data.lock().await;
    counter.count+=1;
    format!("count value = {}", counter.count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conf_pntr= Arc::new(AppConf{
        api_key:"super_duper_secret_key".to_string(),
        database: "postgres://user:@mydb".to_string(),
        debug: true
    });

    let count_ptr=
    Arc::new(TokioMutex::new(StateCounter{count: 0}));

    HttpServer::new(move || App::new()
    .service(get_app)
    .service(get_counter)
    .service(
        web::scope("/users")
        .route("/add",post().to(add_user))
    )
    .app_data(Data::new(conf_pntr.clone()))
    .app_data(Data::new(count_ptr.clone()))
    .app_data(DB::db_connect()))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
