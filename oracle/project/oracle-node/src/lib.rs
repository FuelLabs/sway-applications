use async_trait::async_trait;
use fuels::{accounts::wallet::WalletUnlocked, tx::Receipt};
use futures::executor::block_on;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::time::Duration;
use tokio::{sync::mpsc::Receiver, task::JoinHandle, time::sleep};

// Decimal precision of the asset we are pushing prices to
const DECIMAL_PRECISION: f64 = 1e9;

// Used to deserialize the USD price of ETH from an api endpoint
// We must allow non_snake_case because the JSON field we are deserializing is spelled that way
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct USDPrice {
    USD: f64,
}

/// Spawns a thread to periodically fetch the price of an asset and update the oracle smart contract with that price
///
/// # Arguments
/// - `price_updater` - updates the oracle contract with new prices
/// - `period` - duration to wait before fetching and updating the price for the oracle
/// - `price_fetcher` - fetches the latest price for an asset
pub fn spawn_oracle_updater_job(
    price_updater: impl PriceUpdater + Send + 'static,
    period: Duration,
    price_fetcher: impl PriceProvider + Send + 'static,
) -> (JoinHandle<()>, Receiver<anyhow::Result<Vec<Receipt>>>) {
    // Variables to send log receipts out of the thread
    let (sender, receiver) = tokio::sync::mpsc::channel(100);
    let handle = tokio::task::spawn_blocking(move || loop {
        let usd_price = block_on(price_fetcher.get_price()).unwrap();
        // Update the oracle with the latest price and get the log receipts
        let log_receipts = price_updater.set_price(usd_price).map(|receipts| {
            receipts
                .into_iter()
                .filter(|receipt| matches!(receipt, Receipt::Log { .. } | Receipt::LogData { .. }))
                .collect()
        });

        // Send log receipts out of the thread
        block_on(sender.send(log_receipts)).unwrap();
        block_on(sleep(period));
    });
    // Return the thread handle and channel receiver
    // This allows us to control the thread
    // and receive the log receipts from outside the thread
    (handle, receiver)
}

/// Fetches the latest price info to provide to the oracle contract
#[async_trait]
pub trait PriceProvider {
    async fn get_price(&self) -> anyhow::Result<u64>;
}

#[derive(Clone)]
pub struct NetworkPriceProvider {
    // Makes network requests to fetch price info
    client: Client,
    // Url endpoint to make requests on
    url: Url,
}

impl NetworkPriceProvider {
    pub fn new(client: Client, url: Url) -> Self {
        Self { client, url }
    }
}

#[async_trait]
impl PriceProvider for NetworkPriceProvider {
    /// Get the latest price from an api endpoint
    /// and return it as an u64
    async fn get_price(&self) -> anyhow::Result<u64> {
        let response = self
            .client
            .get(self.url.clone())
            .send()
            .await?
            .json::<USDPrice>()
            .await?;
        Ok((response.USD * DECIMAL_PRECISION) as u64)
    }
}

/// Updates the oracle contract with the specified price
pub trait PriceUpdater {
    fn set_price(&self, price: u64) -> anyhow::Result<Vec<Receipt>>;
}

impl PriceUpdater for utils::Oracle<WalletUnlocked> {
    /// Set the price for the oracle contract and return the log receipts
    fn set_price(&self, price: u64) -> anyhow::Result<Vec<Receipt>> {
        let methods = self.methods();
        Ok(block_on(methods.set_price(price).call())?.receipts)
    }
}
