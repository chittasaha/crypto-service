use actix_web::{get, post, web, App, HttpServer, Responder};
use coingecko_rs::{CoinGeckoClient, response::coins::MarketData};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use tracing::{info, Level};

use crate::models::{Currency , Token};

pub mod models;


#[post("/prices")]
async fn get_price(ids: String) -> impl Responder {
    
    let vec_currency = ids.split(",").collect::<Vec<&str>>() ;
    
    //let vec_currency = vec!["acala","cardano","altair","altura","astar","avalanche-2","centrifuge","coti","curve-dao-token","polkadot","efinity","ethereum","fantom","moonbeam","kilt-protocol","kintsugi","calamari-network","kusama","decentraland","moonriver","harmony","parallel-finance","the-sandbox","shiden", "bitcoin","pha","interlay","litentry", "nodle-network", "origintrail","unique-network","polkadex","bifrost-native-coin"];
        
        //let currencies = vec_currency.iter().map(|x| x.to_string() + ",").collect::<String>();
        
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
                //println!("{0}",cur);
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
                println!("{0}",price);
            }
            else {
                //println!("{0}-{1}",name, 0.0);
            }

            response_price.push(Currency
                {
                    base_currency: base_cur,
                    price: price,
                    name : name,
                    change_last_24_hours: Some(change_24),
                    market_capital: Some(market_capital)
                });
            
            
        } 

    serde_json::to_string(
    &response_price).expect("Something wrong with the request")
}

#[get("/tokens")]
async fn get_tokens() -> String {
    
    let client = CoinGeckoClient::default();

    
    let mut tokens:Vec<Token> = vec![]; 
    
    let coint_list = client.coins_list(false)
    .await.unwrap();

    for t in coint_list
    {
        
        if t.id == "" 
        {
            continue;
        }

        
        
        let coin = match client.coin(&t.id, false,false,true,true,true,false).await
        {
            Ok(c) => c,
            Err(_e) => continue,
        };
        
        
        
        println!("{0}", coin.name);
        
        let id = coin.id;
        let name = coin.name;        
        let mut market_cap = 0.0;
        let mut total_supply:f64 = 0.0;
        let mut max_supply:f64 = 0.0;
        let mut circulating_supply:f64 = 0.0; 
        let mut all_time_high = 0.0;
        let mut all_time_low = 0.0;
        let mut ath_date: String = String::from("");
        let mut atl_date: String = String::from("");
        let mut current_price = 0.0;
        let mut ath_change_parcent= 0.0;
        let mut atl_change_parcent= 0.0;

        match coin.market_data{
            Some(v) => {
                current_price = match v.current_price.usd{
                    Some(cp) => cp,
                    _ => 0.0,
                };
                all_time_high = match v.ath.usd{
                    Some(ath) => ath,
                    _ => 0.0,
                };
                ath_date = match v.ath_date.usd{
                    Some(athd) => athd,
                    _ => String::from(""),
                };
                atl_date = match v.atl_date.usd{
                    Some(atld) => atld,
                    _ => String::from(""),
                };
                all_time_low = match v.atl.usd{
                    Some(alt) => alt,
                    _ => 0.0,
                };
                ath_change_parcent = match v.ath_change_percentage.usd{
                    Some(athp) => athp,
                    _ => 0.0,
                };
                atl_change_parcent = match v.atl_change_percentage.usd{
                    Some(atlp) => atlp,
                    _ => 0.0,
                };
                market_cap = match v.market_cap.usd{
                    Some(mk) => mk,
                    _ => 0.0,
                };
                total_supply = match v.total_supply {
                    serde_json::value::Value::Number(ts) => ts.as_f64().unwrap(),
                    _ => 0.0,
                };
                max_supply = match v.max_supply{
                    serde_json::value::Value::Number(ms) => ms.as_f64().unwrap(),
                    _ => 0.0,
                };
                circulating_supply =  match v.circulating_supply{
                    
                    serde_json::value::Value::Number(cs) => cs.as_f64().unwrap(),
                    _ => 0.0,
                };

            },
            None => {},
        };

        if total_supply < 100.0
        {
            continue;
        }
        

        let devs_data = coin.developer_data.unwrap();
        let total_devs = devs_data.pull_request_contributors.unwrap();
        let last4_weeks_commit = devs_data.commit_count4_weeks.unwrap();

        
        let catagories = coin.categories.iter().map(|c| c.to_string() + ",").collect::<String>();
        let description = coin.description.en.unwrap();
        let twitter_follower = coin.community_data.unwrap().twitter_followers.unwrap();
        //let market_rank = String::from(coin.market_cap_rank.as_str().unwrap());

        //println!("{:?}", coin.clone());

        let token = Token{
            id : id,
            name : name,
            catagories : catagories,
            description : description,
            market_cap: market_cap,
            market_rank : String::from(""),
            all_time_high : all_time_high,
            ath_date : ath_date,
            all_time_low : all_time_low,
            atl_date : atl_date,
            ath_change_parcent : ath_change_parcent,
            atl_change_parcent : atl_change_parcent,
            current_price : current_price,
            cir_supply : circulating_supply,
            total_supply : total_supply,
            max_supply : max_supply,
            total_devs : total_devs,
            last4_weeks_commit : last4_weeks_commit,
            twitter_followers : twitter_follower,            
        };

        println!("{:?}", &token);
        
        

        

        
        tokens.push(token);

    }
    
    serde_json::to_string(&tokens).expect("Something went wrong!")
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