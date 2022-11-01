use async_trait::async_trait;
use fuels::{
    tx::{Receipt},
};
use futures::executor::block_on;
use reqwest;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tokio::time::{sleep};

// We must allow non_snake_case because the JSON field we are deserializing is spelled that way
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct USDPrice {
    USD: f64,
}

pub fn spawn_oracle_updater_job(
    price_updater: impl PriceUpdater + Send + 'static,
    period: Duration,
    price_fetcher: impl PriceProvider + Send + 'static,
) -> (JoinHandle<()>, Receiver<anyhow::Result<Vec<Receipt>>>) {
    let (sender, receiver) = tokio::sync::mpsc::channel(100);
    let handle = tokio::task::spawn_blocking(move || loop {
        let usd_price = block_on(price_fetcher.get_price()).unwrap();
        let log_receipts = price_updater.set_price(usd_price).map(|receipts| {
            receipts
                .into_iter()
                .filter(|receipt| matches!(receipt, Receipt::Log { .. } | Receipt::LogData { .. }))
                .collect()
        });

        block_on(sender.send(log_receipts)).unwrap();
        block_on(sleep(period));
    });
    (handle, receiver)
}

#[async_trait]
pub trait PriceProvider {
    async fn get_price(&self) -> anyhow::Result<u64>;
}

#[derive(Clone)]
pub struct NetworkPriceProvider {
    client: Client,
    url: Url,
}

impl NetworkPriceProvider {
    pub fn new(client: Client, url: Url) -> Self {
        Self { client, url }
    }
}

#[async_trait]
impl PriceProvider for NetworkPriceProvider {
    async fn get_price(&self) -> anyhow::Result<u64> {
        let response = self
            .client
            .get(self.url.clone())
            .send()
            .await?
            .json::<USDPrice>()
            .await?;
        // TODO avoid hardcoding 1e9 decimal precision
        Ok((response.USD * 1e9) as u64)
    }
}

pub trait PriceUpdater {
    fn set_price(&self, price: u64) -> anyhow::Result<Vec<Receipt>>;
}

impl PriceUpdater for utils::Oracle {
    fn set_price(&self, price: u64) -> anyhow::Result<Vec<Receipt>> {
        let methods = self.methods();
        Ok(block_on(methods.set_price(price).call())?.receipts)
    }
}
