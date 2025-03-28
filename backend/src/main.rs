use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use uuid::Uuid;

mod blockchain;
use blockchain::Block;

struct AppState {
    db: PgPool,
    difficulty: usize,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                difficulty: 4,
            }))
            .service(get_blocks)
            .service(mine_block)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/blocks")]
async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
    let blocks = sqlx::query_as::<_, Block>("SELECT * FROM blocks ORDER BY timestamp DESC")
        .fetch_all(&data.db)
        .await
        .unwrap();
    HttpResponse::Ok().json(blocks)
}

#[post("/mine")]
async fn mine_block(data: web::Data<AppState>) -> impl Responder {
    let last_block = sqlx::query_as::<_, Block>("SELECT * FROM blocks ORDER BY timestamp DESC LIMIT 1")
        .fetch_one(&data.db)
        .await;

    let previous_hash = match last_block {
        Ok(block) => block.hash,
        Err(_) => "0".to_string(),
    };

    let mut new_block = Block::new(&previous_hash, "New Transaction");
    new_block.mine(data.difficulty);

    sqlx::query!(
        r#"
        INSERT INTO blocks (id, hash, previous_hash, transactions, nonce, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        new_block.id,
        new_block.hash,
        new_block.previous_hash,
        new_block.transactions,
        new_block.nonce as i64,
        new_block.timestamp
    )
    .execute(&data.db)
    .await
    .unwrap();

    HttpResponse::Ok().json(new_block)
}
