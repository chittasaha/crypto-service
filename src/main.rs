use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use handlers::get_tokens;
use tracing::{info, Level};
use services::{CoingeckoService,CoingeckoServiceTrait};

mod services;
mod handlers;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let service:Arc<dyn CoingeckoServiceTrait> = Arc::new(CoingeckoService{});
    
    HttpServer::new(move || {
        App::new()        
        .service(get_tokens)
        .app_data(web::Data::new(service.clone()))
        
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}     