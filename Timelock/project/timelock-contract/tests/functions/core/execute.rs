use crate::utils::{
    interface::core::{execute, queue},
    setup::setup,
};

mod success {

    use super::*;
    crate::utils::{
        interface::info::{queued, transaction_hash},
        setup::ExecuteEvent,
    };

    #[tokio::test]
    async fn executes() {}

}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AuthorizationError")]
    async fn when_unauthorized() {}

    #[tokio::test]
    #[should_panic(expected = "InvalidTransaction")]
    async fn when_transaction_not_queued() {}

    #[tokio::test]
    #[should_panic(expected = "TimestampNotInRange")]
    async fn when_timestamp_before_start_time() {}

    #[tokio::test]
    #[should_panic(expected = "TimestampNotInRange")]
    async fn when_timestamp_after_end_time() {}

}
