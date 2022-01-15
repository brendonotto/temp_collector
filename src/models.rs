use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row, PgPool};

// Db model of temerature reading
#[derive(Debug, Deserialize)]
pub struct Reading {
    pub room : String,
    pub temperature : f64,
    pub humidity: f64,
    pub timestamp : DateTime<Local>
}

// Request model for API
#[derive(Debug, Deserialize)]
pub struct ReadingRequest {
    pub room : String,
    pub temperature : f64,
    pub humidity: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Room {
    pub id : i32,
    pub room_name : String
}

// impl Responder for Room {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         HttpResponse::Ok().json(&self)
//     }
// }

impl Room {
    pub async fn list_all(pool: &PgPool) -> Result<Vec<Room>> {
        let rooms = sqlx::query!(
            "SELECT id, room_name FROM room"
            )
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|rec| Room {
                id: rec.id,
                room_name: rec.room_name,
            })
            .collect();

        Ok(rooms)
    }
}