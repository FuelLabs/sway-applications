use fuels::tx::Address;
use fuels::signers::Signer;
use crate::utils::{
    abi_calls::constructor,
    test_helpers::setup,
    Identity,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn construct() {
        let user = setup().await;
        constructor(&user.oracle, Identity::Address(Address::from(user.wallet.address()))).await;
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn panics_when_reinitialized() {
        let user = setup().await;
        constructor(&user.oracle, Identity::Address(Address::from(user.wallet.address()))).await;
        constructor(&user.oracle, Identity::Address(Address::from(user.wallet.address()))).await;
    }
}