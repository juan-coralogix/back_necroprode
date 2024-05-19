use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{NewUser, User};
use crate::schema::users;
use crate::DbPool;

#[derive(serde::Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let new_user = NewUser {
        username: &user.username,
        email: &user.email,
        password_hash: &user.password,  // Hash the password in a real app
        created_at: chrono::Local::now().naive_local(),
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn); // Pass mutable reference here

    match result {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable
    let result = users.load::<User>(&mut conn); // Pass mutable reference here

    match result {
        Ok(user_list) => HttpResponse::Ok().json(user_list),
        Err(_) => HttpResponse::InternalServerError().body("Error loading users"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(get_users))
    );
}