use actix_web::{get, post, web, HttpResponse, Responder };
use sqlx::PgPool;

use crate::models::{ Room, Reading, ReadingRequest };

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

#[post("/reading")]
async fn create(reading: web::Json<ReadingRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Reading::create(reading.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(reading) => HttpResponse::Ok().json(reading),
        Err(err) => {
            HttpResponse::InternalServerError().body("Error creating new reading entry")
        }
    }
}