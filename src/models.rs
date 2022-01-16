use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row, PgPool};

// Db model of temerature reading
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Reading {
    pub id : i32,
    pub room : String,
    pub temperature : f32,
    pub humidity: f32,
    pub timestamp : DateTime<Utc>
}

// Request model for API
#[derive(Debug, Deserialize)]
pub struct ReadingRequest {
    pub room : String,
    pub temperature : f32,
    pub humidity: f32,
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

        let room_id: i32 = sqlx::query("SELECT id FROM room WHERE room_name = $1")
            .map(|row: PgRow| row.get(0))
            .fetch_one(&mut tx)
            .await?;

        sqlx::query!(
            r#"
            INSERT INTO temperature (room_id, temperature, humidity, time) VALUES ($1, $2, $3, $4)
            "#,
            room_id,
            reading.temperature,
            reading.humidity,
            Utc::now()
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
            room: rec.room_name,
            temperature: rec.temperature,
            humidity: rec.humidity,
            timestamp: rec.time
        })
    }
}