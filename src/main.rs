use sqlx::postgres::PgPoolOptions;
use sqlx;
use std::env;
use warp::Filter;

mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url: String;
    
    match env::var("DATABASE_URL") {
        Ok(pass) => db_url = pass,
        _ => panic!("Couldn't read db password!!!"),
    }
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str()).await?;


    #[derive(sqlx::FromRow)]
    struct Room { id: i32, room_name: String }

    let rooms_db = sqlx::query_as!(
        Room,
        "SELECT * FROM room"
        )
        .fetch_all(&pool)
        .await?;
    
    let rooms = warp::path!("rooms")
        .map(|| rooms_db);
    
    warp::serve(rooms).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}



// Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
// let row: (i64,) = sqlx::query_as("SELECT $1")
// .bind(150_i64)
// .fetch_one(&pool).await?;
