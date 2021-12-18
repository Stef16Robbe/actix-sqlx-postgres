use std::env;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use actix_web::{web, App, HttpServer, middleware, Responder, HttpResponse};

mod user;
mod routes;

// root (/) handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Welcome to Actix-web with SQLx postgres example.
        Available routes:
        GET /users/{id} -> get a user by it's id
        "#
    )
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
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .configure(user::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
