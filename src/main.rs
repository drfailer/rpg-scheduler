mod scheduler;
mod schema;
mod games_repository;
mod models;

use std::io;
use actix_web::{ web, App, HttpServer };
use diesel::SqliteConnection;
use diesel::r2d2;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new("app.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(scheduler::test_get)
            .service(scheduler::test_get_game)
            .service(scheduler::test_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
