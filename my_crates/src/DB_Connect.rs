use actix_web::web;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use std::env;

pub struct DB{
    connection_string: String
}

impl DB{
    fn new() -> Self{
        let con=DB{
            connection_string:
            "postgresql://postgres:SEE9lu$9lu$@localhost:5432/actixdb".to_string()
        };
        return con;
    }

    
    pub async fn db_connect() -> web::Data<Arc<Pool<Postgres>>>{
        let db=DB::new();
        let database_url=env::var("DATABASE_URL")
        .expect("Database url must be configed in environment vars");
       let pool = Pool::connect(&database_url).await.expect("Could not connect");
        println!("Database pool successfully created");
       let ap= Arc::new(pool);
       return web::Data::new(ap.clone());


    }
}