use actix_web::{get, post, web, HttpResponse, Responder };
use sqlx::PgPool;

use crate::models::Room;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(rooms);
}

#[get("/rooms")]
async fn rooms(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Room::list_all(db_pool.get_ref()).await;
    match result {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(err) => {
            HttpResponse::InternalServerError()
                .body("Error trying to read all rooms from database")
        }
    }
}