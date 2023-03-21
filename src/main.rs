use actix_web::{App, HttpServer};
use apis::{get_price, get_tokens};

use tracing::{info, Level};

mod apis;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
        .service(get_price)
        .service(get_tokens)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}     