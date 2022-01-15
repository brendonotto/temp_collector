use actix_web::{ middleware, web, App, HttpResponse, HttpServer, Responder };
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx;
use std::env;
use anyhow::Result;

mod models;
mod routes;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Welcome to the temperature collection point.
        Available routes:
        GET  /rooms -> list of all rooms
        POST /temp -> create a new temparture entry
        "#
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .expect("PORT should be u16");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str()).await?;
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .configure(routes::init)
    })
    .bind((host, port))?;

    server.run().await?;

    Ok(())
}