use actix_web::error;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::r2d2;
use diesel::SqliteConnection;

use crate::games_repository::{delete_game, get_game, insert_game};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/scheduler/test")]
pub async fn test_get() -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[get("/scheduler/game/{id}")]
pub async fn test_get_game(
    pool: web::Data<DbPool>,
    id: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = id.into_inner();
    let game = web::block(move || {
        let mut conn = pool.get().expect("Error: can't connect to db");
        get_game(&mut conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(game))
}

#[post("/scheduler/game/{name}")]
pub async fn test_post(
    pool: web::Data<DbPool>,
    name: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let (name,) = name.into_inner();
    let game = web::block(move || {
        let mut conn = pool.get().expect("Error: can't connect to db");
        insert_game(&mut conn, name)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(game))
}

#[delete("/scheduler/game/{id}")]
pub async fn test_delete(
    pool: web::Data<DbPool>,
    id: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = id.into_inner();
    let _ = web::block(move || {
        let mut conn = pool.get().expect("Error: can't connect to db");
        delete_game(&mut conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok())
}
