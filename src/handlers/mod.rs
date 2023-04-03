use std::sync::Arc;
use actix_web::{post, web};
use crate::services::{CoingeckoServiceTrait, models::GetTokensRequest};



#[post("/tokens")]
async fn get_tokens(req: web::Json<GetTokensRequest>, coingecko_service: web::Data<Arc<dyn CoingeckoServiceTrait>>) 
    -> Result<String, Box<dyn std::error::Error>>{    
    
    let base_currency = req.base_currency.clone();
    let ids = &req.ids.join(",");

    let coins = coingecko_service.get_tokens(&base_currency, ids).await?;

    Ok(serde_json::to_string(&coins).unwrap())
}