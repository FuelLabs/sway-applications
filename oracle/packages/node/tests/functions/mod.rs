use async_trait::async_trait;
use fuels::tx::Receipt;
use futures::executor::block_on;
use node::{PriceProvider, PriceUpdater};
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

mod get_price;
mod new;
mod run;

struct Invocation {
    price: u64,
    time: Instant
}

struct LoggingPriceUpdater {
    invocations: Arc<Mutex<Vec<Invocation>>>,
    receipts: Vec<Receipt>,
}

impl LoggingPriceUpdater {
    pub fn new() -> Self {
        Self {
            invocations: Arc::new(Mutex::new(vec![])),
            receipts: vec![],
        }
    }

    pub fn invocations(&self) -> Arc<Mutex<Vec<Invocation>>> {
        Arc::clone(&self.invocations)
    }
}

impl PriceUpdater for LoggingPriceUpdater {
    fn set_price(&self, price: u64) -> anyhow::Result<Vec<Receipt>> {
        block_on(self.invocations.lock())
            .borrow_mut()
            .push(Invocation {
                price,
                time: Instant::now(),
            });
        Ok(self.receipts.clone())
    }
}

#[derive(Clone)]
struct HardcodedPriceProvider {
    price: u64,
}

#[cfg(test)]
#[async_trait]
impl PriceProvider for HardcodedPriceProvider {
    async fn get_price(&self) -> anyhow::Result<u64> {
        async { Ok(self.price) }.await
    }
}
