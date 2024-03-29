use crate::utils::{interface::core::queue, setup::setup};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{queued, transaction_hash},
        setup::QueueEvent,
    };

    #[tokio::test]
    async fn queues() {}
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AuthorizationError")]
    async fn when_unauthorized() {}

    #[tokio::test]
    #[should_panic(expected = "DuplicateTransaction")]
    async fn when_transaction_is_a_duplicate() {}

    #[tokio::test]
    #[should_panic(expected = "TimestampNotInRange")]
    async fn when_timestamp_before_delay() {}

    #[tokio::test]
    #[should_panic(expected = "TimestampNotInRange")]
    async fn when_timestamp_after_delay() {}
}
