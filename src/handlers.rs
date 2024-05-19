use actix_web::{web::{self, Json}, HttpResponse, Responder, App, HttpServer};
use diesel::prelude::*;
use crate::models::{NewUser, User, NewNecroprode, Necroprode, NewNecroprodeMember, NecroprodeMember, NewSelection, Selection};
use crate::schema::{users, necroprodes, necroprode_members, selections};
use crate::DbPool;
use paperclip::actix::{
    api_v2_operation,
    OpenApiExt, Apiv2Schema,
};

#[derive(serde::Deserialize, Apiv2Schema)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[api_v2_operation]
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
#[api_v2_operation]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable
    let result = users.load::<User>(&mut conn);

    match result {
        Ok(user_list) => HttpResponse::Ok().json(user_list),
        Err(_) => HttpResponse::InternalServerError().body("Error loading users"),
    }
}

#[derive(serde::Deserialize, Apiv2Schema)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
#[api_v2_operation]
pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    user: web::Json<UpdateUser>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let result = diesel::update(users.find(user_id.into_inner()))
        .set((
            user.username.as_ref().map(|u| username.eq(u)),
            user.email.as_ref().map(|e| email.eq(e)),
            user.password.as_ref().map(|p| password_hash.eq(p)),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("User updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}
#[api_v2_operation]
pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let result = diesel::delete(users.find(user_id.into_inner())).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting user"),
    }
}

// CRUD for Necroprode
#[derive(serde::Deserialize)]
pub struct CreateNecroprode {
    pub name: String,
    pub creator_id: i32,
}

pub async fn create_necroprode(
    pool: web::Data<DbPool>,
    necroprode: web::Json<CreateNecroprode>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let new_necroprode = NewNecroprode {
        name: &necroprode.name,
        creator_id: necroprode.creator_id,
        created_at: chrono::Local::now().naive_local(),
    };

    let result = diesel::insert_into(necroprodes::table)
        .values(&new_necroprode)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Necroprode created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating necroprode"),
    }
}

pub async fn get_necroprodes(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::necroprodes::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable
    let result = necroprodes.load::<Necroprode>(&mut conn);

    match result {
        Ok(necroprode_list) => HttpResponse::Ok().json(necroprode_list),
        Err(_) => HttpResponse::InternalServerError().body("Error loading necroprodes"),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateNecroprode {
    pub name: Option<String>,
}

pub async fn update_necroprode(
    pool: web::Data<DbPool>,
    necroprode_id: web::Path<i32>,
    necroprode: web::Json<UpdateNecroprode>,
) -> impl Responder {
    use crate::schema::necroprodes::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let result = diesel::update(necroprodes.find(necroprode_id.into_inner()))
        .set(necroprode.name.as_ref().map(|n| name.eq(n)))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Necroprode updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating necroprode"),
    }
}

pub async fn delete_necroprode(
    pool: web::Data<DbPool>,
    necroprode_id: web::Path<i32>,
) -> impl Responder {
    use crate::schema::necroprodes::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let result = diesel::delete(necroprodes.find(necroprode_id.into_inner())).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Necroprode deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting necroprode"),
    }
}

// CRUD for NecroprodeMember
#[derive(serde::Deserialize)]
pub struct CreateNecroprodeMember {
    pub necroprode_id: i32,
    pub user_id: i32,
}

pub async fn create_necroprode_member(
    pool: web::Data<DbPool>,
    member: web::Json<CreateNecroprodeMember>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let new_member = NewNecroprodeMember {
        necroprode_id: member.necroprode_id,
        user_id: member.user_id,
    };

    let result = diesel::insert_into(necroprode_members::table)
        .values(&new_member)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Member added"),
        Err(_) => HttpResponse::InternalServerError().body("Error adding member"),
    }
}

pub async fn get_necroprode_members(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::necroprode_members::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable
    let result = necroprode_members.load::<NecroprodeMember>(&mut conn);

    match result {
        Ok(member_list) => HttpResponse::Ok().json(member_list),
        Err(_) => HttpResponse::InternalServerError().body("Error loading members"),
    }
}

pub async fn delete_necroprode_member(
    pool: web::Data<DbPool>,
    ids: web::Path<(i32, i32)>,
) -> impl Responder {
    use crate::schema::necroprode_members::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let result = diesel::delete(necroprode_members.find((ids.0, ids.1))).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Member removed"),
        Err(_) => HttpResponse::InternalServerError().body("Error removing member"),
    }
}
// CRUD for Selection
#[derive(serde::Deserialize)]
pub struct CreateSelection {
    pub necroprode_id: i32,
    pub user_id: i32,
    pub celebrity_name: String,
}

pub async fn create_selection(
    pool: web::Data<DbPool>,
    selection: web::Json<CreateSelection>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let new_selection = NewSelection {
        necroprode_id: selection.necroprode_id,
        user_id: selection.necroprode_id,
        celebrity_name: &selection.celebrity_name,
        created_at: chrono::Local::now().naive_local(),
    };

    let result = diesel::insert_into(selections::table)
        .values(&new_selection)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Selection created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating selection"),
    }
}

pub async fn get_selections(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::selections::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable
    let result = selections.load::<Selection>(&mut conn);

    match result {
        Ok(selection_list) => HttpResponse::Ok().json(selection_list),
        Err(_) => HttpResponse::InternalServerError().body("Error loading selections"),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateSelection {
    pub celebrity_name: Option<String>,
}

pub async fn update_selection(
    pool: web::Data<DbPool>,
    selection_id: web::Path<i32>,
    selection: web::Json<UpdateSelection>,
) -> impl Responder {
    use crate::schema::selections::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let result = diesel::update(selections.find(selection_id.into_inner()))
        .set((
            selection.celebrity_name.as_ref().map(|c| celebrity_name.eq(c)),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Selection updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating selection"),
    }
}

pub async fn delete_selection(
    pool: web::Data<DbPool>,
    selection_id: web::Path<i32>,
) -> impl Responder {
    use crate::schema::selections::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut conn = conn; // Make the connection mutable

    let result = diesel::delete(selections.find(selection_id.into_inner())).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Selection deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting selection"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(get_users))
    )
    .service(
        web::resource("/users/{id}")
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user)),
    )
    .service(
        web::resource("/necroprodes")
            .route(web::post().to(create_necroprode))
            .route(web::get().to(get_necroprodes))
    )
    .service(
        web::resource("necroprodes/{id}")
            .route(web::put().to(update_necroprode))
            .route(web::delete().to(delete_necroprode)),
    )
    .service(
        web::resource("/necroprode_members")
            .route(web::post().to(create_necroprode_member))
    )
    .service(
        web::resource("/necroprode_members")
            .route(web::get().to(get_necroprode_members))
    )
    .service(
        web::resource("/selections")
            .route(web::post().to(create_selection))
            .route(web::get().to(get_selections))
    )
    .service(
        web::resource("/selections/{id}")
            .route(web::put().to(update_selection))
            .route(web::delete().to(delete_selection)),
    );
}
