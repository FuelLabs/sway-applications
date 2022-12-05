use crate::utils::{
    abi_calls::{constructor, max_supply},
    test_helpers::setup,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_max_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(max_supply(&owner1.contract).await, None);

        constructor(None, &deploy_wallet.contract, Some(10)).await;

        assert_eq!(max_supply(&owner1.contract).await, Some(10));
    }
}
