use utils::{abi_calls::owner, test_helpers::setup};
use fuels::{prelude::Identity, tx::Address};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_owner() {
        let (user, _) = setup().await;
        let owner = owner(&user.oracle).await;
        assert_eq!(
            owner,
            Identity::Address(Address::from(user.wallet.address()))
        );
    }
}
