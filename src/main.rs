use actix_web::{get, post, web, App, HttpServer, Responder};
use coingecko_rs::{
    params::{MarketsOrder, PriceChangePercentage},
    CoinGeckoClient,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use tracing::{info, Level};

use crate::models::{Currency , Token};

pub mod models;


#[post("/prices")]
async fn get_price(ids: web::Json<String>) -> impl Responder {
    
    let vec_currency = ids.split(",").collect::<Vec<&str>>();    
        
    let client = CoinGeckoClient::default();            

    let prices= match client.price( &vec_currency,
            &vec!["eur"],true, true,
            true, false).await
    {
        Ok(p) => p,
        Err(_error) => panic!(""),
    };

    let mut response_price: Vec<Currency> = Vec::new();

    for cur in vec_currency
    {
        let base_cur = String::from("eur");
        let mut price:f64= 0.0;
        let mut change_24:f64 = 0.0;
        let mut market_capital:f64 = 0.0; 
        let name = String::from(cur);
        if prices.contains_key(cur)
        {            
            price = match prices[cur].eur {                    
                Some(v) => v,
                None => 0.0,
            };
            change_24 = match prices[cur].eur24_h_change {
                Some(v) => v,
                None => 0.0,
            };
            market_capital = match prices[cur].eur_market_cap {
                Some(v) => v,
                None => 0.0,
            };            
        }
        

        response_price.push(Currency
        {
            base_currency: base_cur,
            price,
            name,
            change_last_24_hours: change_24,
            market_capital: market_capital
        });
            
            
    } 

    serde_json::to_string(
    &response_price).expect("Something wrong with the request")
}

#[get("/tokens")]
async fn get_tokens() -> Result<String, Box<dyn std::error::Error>> {
    
    let client = CoinGeckoClient::default();

    //let cats = client.categories_list().await?;

    /*for cat in cats {
        print!("Id: {:?}, Name: {:?}", cat.category_id, cat.name);
    }*/
    
    let coins = client.coins_markets("eur", &[""], None,
     MarketsOrder::MarketCapDesc, 500, 1, true,&[
        PriceChangePercentage::OneHour,
        PriceChangePercentage::TwentyFourHours,
        PriceChangePercentage::SevenDays,
        PriceChangePercentage::FourteenDays,
        PriceChangePercentage::ThirtyDays,
        PriceChangePercentage::OneYear,
    ]).await?;

    
    let mut tokens:Vec<Token> = vec![]; 
    
  

    for coin in coins
    {
        //println!("{0}", coin.name);
        
        let id = coin.id;
        let name = coin.name;        
        let market_cap = coin.market_cap.unwrap();
        let total_supply:f64 = match coin.total_supply{
            Some(v) => v,
            None => 0.0
        };
        let max_supply:f64 = match coin.max_supply{
            Some(v) => v,
            None => 0.0
        };
        let cir_supply:f64 = coin.circulating_supply.unwrap(); 
        let all_time_high = coin.ath.unwrap();
        let all_time_low = coin.atl.unwrap();
        let ath_date: String = coin.ath_date.unwrap();
        let atl_date: String = coin.atl_date.unwrap();
        let current_price = coin.current_price.unwrap();
        let ath_change_parcent= coin.ath_change_percentage.unwrap();
        let atl_change_parcent= coin.atl_change_percentage.unwrap();     
        

        let image = coin.image;
        //let total_devs = devs_data.pull_request_contributors.unwrap();
        /* 
        let last4_weeks_commit = devs_data.commit_count4_weeks.unwrap();

        
        let catagories = coin.categories.iter().map(|c| c.to_string() + ",").collect::<String>();
        let description = coin.description.en.unwrap();
        let twitter_follower = coin.community_data.unwrap().twitter_followers.unwrap();
        //let market_rank = String::from(coin.market_cap_rank.as_str().unwrap());

        //println!("{:?}", coin.clone());

        */

        let token = Token{
            id,
            name,                        
            market_cap,            
            all_time_high,
            ath_date,
            all_time_low,
            atl_date,
            ath_change_parcent,
            atl_change_parcent,
            current_price,
            cir_supply,
            total_supply,
            max_supply,
            icon_url : image                     
        };

        //println!("{:?}", &token);
        
        

        

        
        tokens.push(token);

    }
    
    Ok(serde_json::to_string(&tokens).unwrap())
}

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