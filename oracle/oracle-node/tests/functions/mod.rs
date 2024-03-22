use async_trait::async_trait;
use fuels::tx::Receipt;
use futures::executor::block_on;
use oracle_node::{PriceProvider, PriceUpdater};
use std::{borrow::BorrowMut, sync::Arc, time::Instant};
use tokio::sync::Mutex;

mod run;

struct Invocation {
    // Used to ensure the polled price is correct
    price: u64,
    // Used to ensure the polling delay is correct
    time: Instant,
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
    // Hardcoded price to provide for testing
    price: u64,
}

#[cfg(test)]
#[async_trait]
impl PriceProvider for HardcodedPriceProvider {
    async fn get_price(&self) -> anyhow::Result<u64> {
        async { Ok(self.price) }.await
    }
}
