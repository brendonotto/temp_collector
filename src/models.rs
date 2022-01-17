use anyhow::Result;
use chrono::{DateTime, Utc,};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

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

        let record = sqlx::query!("SELECT id FROM room WHERE room_name = $1", reading.room)
            .fetch_one(&mut tx)
            .await?;

        let room_id = record.id;

        log::debug!("Just got room id {}", room_id);

        let row = sqlx::query!(
            r#"
            INSERT INTO temperature (room_id, temperature, humidity, time) VALUES ($1, $2, $3, $4)
            RETURNING id, temperature, humidity, time
            "#,
            room_id,
            reading.temperature,
            reading.humidity,
            Utc::now()
        )
        .fetch_one(&mut tx)
        .await?;

        log::debug!("Just inserted the reading");

        tx.commit().await?;

        log::debug!("Transaction committed");

        Ok(Reading {
            id: row.id,
            room: reading.room,
            temperature: row.temperature,
            humidity: row.humidity,
            timestamp: row.time
        })
    }
}