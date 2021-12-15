use serde::Serialize;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub async fn find_by_id(id: i32, pool: &Pool<Postgres>) -> Result<User, &str> {
        // TODO:
        // https://docs.rs/sqlx/0.5.9/sqlx/macro.query.html#query-arguments
        let user = sqlx::query_as!(User, "SELECT * FROM postgresactix.users WHERE id = 1")
            .fetch_one(&*pool)
            .await.unwrap();

        Ok(user)
    }

    pub fn to_string(&self) -> String {
        format!("{{id: {}, name: {}}}", &self.id, &self.name)
    }
}

#[get("/users")]
async fn users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let user = User::find_by_id(1, &pool).await.unwrap();

    HttpResponse::Ok().body(user.to_string())
}

// cargo install cargo-watch
// cargo watch -x 'run --bin {APP_NAME}'
// https://actix.rs/book/actix/
// https://github.com/actix/examples/blob/master/database_interactions/sqlx_todo/src/main.rs
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO:
    // env
    let conn = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("2115")
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
