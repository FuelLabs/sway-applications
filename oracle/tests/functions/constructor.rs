use crate::utils::{abi_calls::{constructor, owner}, test_helpers::setup, Identity};
use fuels::{signers::Signer, tx::Address};

mod success {
    use super::*;

    #[tokio::test]
    async fn construct() {
        let user = setup().await;
        assert!(false);
        constructor(
            &user.oracle,
            Identity::Address(Address::from(user.wallet.address())),
        )
        .await;
        let owner = owner(&user.oracle).await;
        assert_eq!(
            owner,
            Option::Some(Identity::Address(Address::from(user.wallet.address())))
        );
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
