use actix_web::{get, post, web::{self, Path}, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
    age: u32
}

#[get("/user/{username}/email/{email}/age/{age}")]
async fn hello(path: Path<(String, String, u32)>) -> impl Responder {
    let (nm, em, ag)=path.into_inner();

    let user = User {
        
        name: nm,
        email:em,
        age:ag
        
    };

    web::Json(user)
}