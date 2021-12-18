use actix_web::{Responder, Error, HttpResponse, HttpRequest};
use serde::{Serialize, Deserialize};
use sqlx::{Postgres, Pool, FromRow};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub async fn find_by_id(id: i32, pool: &Pool<Postgres>) -> Result<User, sqlx::Error> {
        // https://docs.rs/sqlx/0.5.9/sqlx/macro.query.html#query-arguments
        let user = sqlx::query_as!(
            User, "SELECT * FROM postgresactix.users WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;
        
        Ok(user)
    }
}
