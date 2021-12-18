use std::env;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use mime::APPLICATION_JSON;
use actix_web::http::header;
use serde::{Serialize, Deserialize};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUser {
    pub name: String,
}

impl DbUser {
    pub async fn find_by_id(id: i32, pool: &Pool<Postgres>) -> Result<DbUser, sqlx::Error> {
        // https://docs.rs/sqlx/0.5.9/sqlx/macro.query.html#query-arguments
        let user = sqlx::query_as!(DbUser, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;
        
        Ok(user)
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Welcome to Actix-web with SQLx postgres example.
        Available routes:
        GET /users -> get all users
        GET /users/{id} -> get a user by it's id
        POST /users -> create a user
        "#
    )
}

#[get("/users")]
async fn all_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let users = sqlx::query_as!(DbUser, "SELECT id, name FROM users")
        .fetch_all(&**pool)
        .await.unwrap();
    let json = serde_json::to_string_pretty(&users).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[get("/users/{id}")]
async fn user_by_id(id: web::Path<u32>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let user = DbUser::find_by_id(*id as i32, &pool).await.unwrap();
    let json = serde_json::to_string_pretty(&user).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[post("/users")]
async fn create_user(user: web::Json<ApiUser>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let created_user = sqlx::query_as!(
        DbUser, 
        "INSERT INTO users (name)
        VALUES ($1)
        RETURNING id, name",
        &user.name
    )
    .fetch_one(&**pool)
    .await
    .unwrap();
    let json = serde_json::to_string_pretty(&created_user).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[put("/users")]
async fn put_user(user: web::Json<DbUser>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let edited_user = sqlx::query_as!(
        DbUser,
        "UPDATE users
        SET name = $1
        WHERE id = $2
        RETURNING id, name",
        &user.name,
        &user.id
    )
    .fetch_one(&**pool)
    .await
    .unwrap();
    let json = serde_json::to_string_pretty(&edited_user).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env!("HOST");
    let port = env!("PORT").parse::<u16>().unwrap();
    let username = env!("NAME");
    let password = env!("PASSWORD");

    let conn = PgConnectOptions::new()
        .host(host)
        .port(port)
        .username(username)
        .password(password)
        .ssl_mode(PgSslMode::Prefer);

    let pool = Pool::<Postgres>::connect_with(conn).await.unwrap();
    let data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(all_users)
            .service(user_by_id)
            .service(create_user)
            .service(put_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
