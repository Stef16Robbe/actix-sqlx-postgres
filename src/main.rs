use std::env;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use mime::APPLICATION_JSON;
use actix_web::http::header;
use serde::{Serialize, Deserialize};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub async fn find_by_id(id: i32, pool: &Pool<Postgres>) -> Result<User, sqlx::Error> {
        // https://docs.rs/sqlx/0.5.9/sqlx/macro.query.html#query-arguments
        let user = sqlx::query_as!(User, "SELECT * FROM postgresactix.users WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;
        
        Ok(user)
    }
}

#[get("/users")]
async fn users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let user = User::find_by_id(1, &pool).await.unwrap();
    let json = serde_json::to_string_pretty(&user).unwrap();

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
            .service(users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
