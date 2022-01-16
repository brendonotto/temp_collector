use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row, PgPool, Executor};

// Db model of temerature reading
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Reading {
    pub id : i32,
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

impl Reading {
    pub async fn create(reading: ReadingRequest, pool: &PgPool) -> Result<Reading> {

        let mut tx = pool.begin().await?;

        sqlx::query!(
            r#"
            SELECT r.id, $2, $3, current_timestamp
            INTO TABLE temperature              
            FROM room r WHERE room_name = $1
            "#,
            reading.room,
            reading.temperature,
            reading.humidity
        )
        .execute(&mut tx)
        .await?;

        let row_id : i32 = sqlx::query("SELECT last_insert_rowid()")
            .map(|row: PgRow| row.get(0))
            .fetch_one(&mut tx)
            .await?;

        let rec = sqlx::query!(
            r#"
            SELECT t.id, r.room_name, temperature, humidity, time 
            FROM temperature t
            INNER JOIN room r ON r.id = t.room_id
            WHERE t.id = $1
            "#,
            row_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(Reading {
            id: rec.id,
            room: rec.room,
            temperature: rec.temperature,
            humidity: rec.humidity,
            timestamp: rec.time
        })
    }
}