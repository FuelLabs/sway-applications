use crate::utils::{interface::core::cancel, setup::setup};

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::queue,
            info::{queued, transaction_hash},
        },
        setup::CancelEvent,
    };

    #[tokio::test]
    async fn cancels() {}
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AuthorizationError")]
    async fn when_unauthorized() {}

    #[tokio::test]
    #[should_panic(expected = "InvalidTransaction")]
    async fn when_transaction_not_queued() {}
}
