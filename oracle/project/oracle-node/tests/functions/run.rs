use crate::functions::{HardcodedPriceProvider, LoggingPriceUpdater};
use fuels::tx::Receipt;
use itertools::Itertools;
use oracle_node::spawn_oracle_updater_job;
use std::{borrow::Borrow, time::Duration};

mod success {
    use super::*;

    #[tokio::test]
    async fn price_is_polled_and_propagated_to_updater() {
        let price_updater = LoggingPriceUpdater::new();
        let invocations = price_updater.invocations();

        let the_price = 101u64;
        let period = Duration::from_millis(500);

        let (_handle, mut _receipts_receiver) = spawn_oracle_updater_job(
            price_updater,
            period,
            HardcodedPriceProvider { price: the_price },
        );

        tokio::time::sleep(period * 2).await;

        assert!(invocations
            .lock()
            .await
            .iter()
            .all(|invocation| { invocation.price == the_price }));
    }

    #[tokio::test]
    async fn delay_is_respected() {
        let price_updater = LoggingPriceUpdater::new();
        let invocations = price_updater.invocations();

        let period = Duration::from_millis(500);
        let (_handle, mut _receipts_receiver) =
            spawn_oracle_updater_job(price_updater, period, HardcodedPriceProvider { price: 10 });

        tokio::time::sleep(period * 3).await;

        assert!(invocations.lock().await.iter().tuple_windows().all(
            |(previous_invocation, current_invocation)| {
                current_invocation.time - previous_invocation.time >= period
            }
        ));
    }

    #[tokio::test]
    async fn receipts_are_streamed() {
        let mut price_updater = LoggingPriceUpdater::new();
        let receipts_from_price_updater = vec![Receipt::LogData {
            id: Default::default(),
            ra: 0,
            rb: 0,
            ptr: 0,
            len: 0,
            digest: Default::default(),
            data: 101u64.to_be_bytes().to_vec(),
            pc: 0,
            is: 0,
        }];
        price_updater.receipts = receipts_from_price_updater.clone();

        let (_handle, mut receipts_receiver) = spawn_oracle_updater_job(
            price_updater,
            Duration::from_millis(500),
            HardcodedPriceProvider { price: 101 },
        );

        let receipts = vec![
            receipts_receiver.recv().await.unwrap().unwrap(),
            receipts_receiver.recv().await.unwrap().unwrap(),
        ];

        assert_eq!(receipts, vec![receipts_from_price_updater; 2]);
    }
}
