mod scheduler;

use std::io;
use actix_web::{ App,  HttpServer };

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(scheduler::test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
