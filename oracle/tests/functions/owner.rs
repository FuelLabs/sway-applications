use crate::utils::{
    abi_calls::{constructor, owner},
    test_helpers::setup,
    Identity, Option,
};
use fuels::{prelude::*, tx::Address};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_owner() {
        let user = setup().await;
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
