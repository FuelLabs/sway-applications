use crate::utils::{
    abi_calls::{initialize, mint, set_mint_amount},
    test_helpers::{build_contract, setup},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_mint_and_transfer_coins() {
        let (owner, .., token_contract_id, token_instance) = setup().await;
        let mint_amount = 10000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        mint(&token_instance).await;
        let coin_id = AssetId::from(*token_contract_id.hash());
        let owner_balance = owner.get_asset_balance(&coin_id).await.unwrap();
        assert_eq!(owner_balance, mint_amount);
    }

    #[tokio::test]
    async fn can_mint_and_transfer_alternative_tokens() {
        let (owner, minter, .., token_contract_id, token_instance) = setup().await;
        let mint_amount = 10000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;
        mint(&token_instance_alternative).await;
        let coin_id = AssetId::from(*token_contract_id.hash());
        let minter_balance = minter.get_asset_balance(&coin_id).await.unwrap();
        assert_eq!(minter_balance, mint_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_uninitialized() {
        let (.., token_instance) = setup().await;
        mint(&token_instance).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_mint_closed() {
        let (owner, .., token_instance) = setup().await;
        let mint_amount = 10000;
        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;
        set_mint_amount(&token_instance, 0).await;
        mint(&token_instance).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_already_mint() {
        let (owner, .., token_instance) = setup().await;
        let mint_amount = 10000;
        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;
        mint(&token_instance).await;
        mint(&token_instance).await;
    }
}
