use serde::{Serialize,Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token
{
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "current_price")]
    pub current_price: f64,
    #[serde(rename = "market_cap")]
    pub market_cap: Option<f64>,
    #[serde(rename = "market_cap_rank")]
    pub market_cap_rank: Option<i32>,
    #[serde(rename = "fully_diluted_valuation")]
    pub fully_diluted_valuation: Option<f64>,
    #[serde(rename = "total_volume")]
    pub total_volume: Option<f64>,
    #[serde(rename = "high_24h")]
    pub high_24h: Option<f64>,
    #[serde(rename = "low_24h")]
    pub low_24h: Option<f64>,
    #[serde(rename = "price_change_24h")]
    pub price_change_24h: Option<f64>,
    #[serde(rename = "price_change_percentage_24h")]
    pub price_change_percentage_24h: Option<f64>,
    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change_24h: Option<f64>,
    #[serde(rename = "market_cap_change_percentage_24h")]
    pub market_cap_change_percentage_24h: Option<f64>,
    #[serde(rename = "circulating_supply")]
    pub circulating_supply: Option<f64>,
    #[serde(rename = "total_supply")]
    pub total_supply: Option<f64>,
    #[serde(rename = "max_supply")]
    pub max_supply: Option<f64>,
    pub ath: Option<f64>,
    #[serde(rename = "ath_change_percentage")]
    pub ath_change_percentage: Option<f64>,
    #[serde(rename = "ath_date")]
    pub ath_date: String,
    pub atl: Option<f64>,
    #[serde(rename = "atl_change_percentage")]
    pub atl_change_percentage: Option<f64>,
    #[serde(rename = "atl_date")]
    pub atl_date: String,
    pub roi: Value,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokensRequest
{
    pub base_currency: String,
    pub ids: Vec<String>
}