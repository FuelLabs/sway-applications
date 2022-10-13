use reqwest;
use dotenv::dotenv;

#[derive(Deserialize)]
struct USDPrice {
    USD: u64,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_url = std::env::var("API_URL").expect("API_URL must be set.");
    let client = reqwest::Client::new();

    // let response = client.get(api_url).send().await.unwrap();
    let response = client.get(api_url).send().await.unwrap().json::<USDPrice>().await.unwrap();

    println!("{}", response);

    //let body = response.text().await.unwrap();


    //println!("Body:\n{}", body["USD"]);
}
