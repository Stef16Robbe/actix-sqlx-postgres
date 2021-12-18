use mime::APPLICATION_JSON;
use actix_web::http::header;
use sqlx::{Pool, Postgres};
use actix_web::{get, web, HttpResponse, Responder};

use crate::user::User;

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(user_by_id);
        // .service(find)
        // .service(create)
        // .service(update)
        // .service(delete);
}

#[get("/users/{id}")]
async fn user_by_id(id: web::Path<i32>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let user = User::find_by_id(*id, &pool).await.unwrap();
    let json = serde_json::to_string_pretty(&user).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}
