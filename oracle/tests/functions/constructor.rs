use crate::utils::{abi_calls::constructor, test_helpers::setup, Identity};
use fuels::signers::Signer;
use fuels::tx::Address;

mod success {
    use super::*;

    #[tokio::test]
    async fn construct() {
        let user = setup().await;
        constructor(
            &user.oracle,
            Identity::Address(Address::from(user.wallet.address())),
        )
        .await;
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn panics_when_reinitialized() {
        let user = setup().await;
        constructor(
            &user.oracle,
            Identity::Address(Address::from(user.wallet.address())),
        )
        .await;
        constructor(
            &user.oracle,
            Identity::Address(Address::from(user.wallet.address())),
        )
        .await;
    }
}
