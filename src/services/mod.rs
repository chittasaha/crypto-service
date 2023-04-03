use async_trait::async_trait;
use models::Token;
use reqwest::header::{ACCEPT, USER_AGENT};

pub mod models;

#[async_trait]
pub trait CoingeckoServiceTrait : Sync + Send
{    
    async fn get_tokens(&self, base_currency:&str, ids:&str) -> Result<Vec::<Token>, Box<dyn std::error::Error>>;    
}

#[derive(Clone)]
pub struct CoingeckoService;

#[async_trait]
impl CoingeckoServiceTrait for CoingeckoService
{
    async fn get_tokens(&self, base_currency:&str, ids:&str) -> Result<Vec::<Token>, Box<dyn std::error::Error>> {    
        
        let coin_gecko_url = format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency={base_currency}&ids={ids}&order=market_cap_desc&per_page=5000&page=1&sparkline=false&locale=en");
        
        let http_client = reqwest::Client::new();
        let response = http_client
                        .get(coin_gecko_url)
                        .header(ACCEPT, "application/json")
                        .header(USER_AGENT, "mozilla")
                        .send().await?;
        
        Ok(response.json::<Vec<Token>>().await?)        
    }
}
