use actix_web::{
    post,
    web::{Data, Json},
    Error, HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use crate::Entities::{User, CreateUserRequest};


pub async fn add_user(
    pool: Data<Arc<Pool<Postgres>>>,
    req: Json<CreateUserRequest>,
) -> Result<impl Responder, Error> {
    let new_user = req.into_inner();

    println!("Adding new user: {:?}", new_user);

    // Attempt to insert the user into the database
    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING id, username, email, created_at
        "#,
        new_user.username,
        new_user.email
    )
    .fetch_one(pool.get_ref().as_ref())
    .await
    .map_err(|e|{

        eprintln!("Error creating user: {:?}", e);

            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    return actix_web::error::ErrorConflict(
                        "User with this email already exists.",
                    );
                }
            }

            actix_web::error::ErrorInternalServerError(
                "Failed to create user.",
            )

    })?;

    println!("User created: {:?}", user);
    Ok(HttpResponse::Created().json(user))
}
