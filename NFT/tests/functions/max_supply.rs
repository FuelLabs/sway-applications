use crate::utils::{
    abi_calls::{constructor, max_supply},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_max_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(max_supply(&owner1.contract).await, 0);

        // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 10).await;

        assert_eq!(max_supply(&owner1.contract).await, 10);
    }
}
