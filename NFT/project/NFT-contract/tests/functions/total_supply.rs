use crate::utils::{
    abi_calls::{constructor, mint, total_supply},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_total_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 10).await;

        assert_eq!(total_supply(&owner1.contract).await, 0);

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        assert_eq!(total_supply(&owner1.contract).await, 1);
    }

    #[tokio::test]
    async fn gets_total_supply_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 10).await;

        assert_eq!(total_supply(&owner1.contract).await, 0);

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;
        assert_eq!(total_supply(&owner1.contract).await, 1);

        mint(1, &owner1.contract, &minter).await;
        assert_eq!(total_supply(&owner1.contract).await, 2);

        mint(2, &owner1.contract, &minter).await;
        assert_eq!(total_supply(&owner1.contract).await, 4);
    }
}
