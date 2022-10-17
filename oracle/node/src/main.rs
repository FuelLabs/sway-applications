use reqwest;
use dotenv::dotenv;
use serde::Deserialize;
use tokio::time::{ self, Duration };
use std::env;

#[derive(Deserialize)]
struct USDPrice {
    USD: f64,
}

#[tokio::main]
async fn main() {
    let mut env_path = env::current_dir().unwrap();
    env_path.push(std::path::Path::new("node"));
    env::set_current_dir(env_path).unwrap();
    dotenv().ok();
    let api_url = std::env::var("API_URL").expect("API_URL must be set.");
    let client = reqwest::Client::new();
    let mut interval = time::interval(Duration::from_millis(10000));
    interval.tick().await;
    let mut i = 0;
    while i < 2 {
        let response = client.get(api_url.clone()).send().await.unwrap().json::<USDPrice>().await.unwrap();
        println!("{:?}", response.USD);
        i += 1;
        interval.tick().await;
    }
}
