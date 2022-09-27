use crate::utils::{
    abi_calls::{mint, set_mint_amount},
    test_helpers::{build_contract, setup_and_initialize},
    MyTokenBuilder,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_mint_and_transfer_coins() {
        let (owner, _minter, mint_amount, token_contract_id, token_instance) =
            setup_and_initialize().await;

        mint(&token_instance).await;
        let coin_id = AssetId::from(*token_contract_id.hash());
        let owner_balance = owner.get_asset_balance(&coin_id).await.unwrap();
        assert_eq!(owner_balance, mint_amount);
    }

    #[tokio::test]
    async fn can_mint_and_transfer_alternative_tokens() {
        let (_owner, minter, mint_amount, token_contract_id, _token_instance) =
            setup_and_initialize().await;

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
        let owner = launch_provider_and_get_wallet().await;

        let token_contract_id = Contract::deploy(
            "../token/out/debug/token.bin",
            &owner,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let token_instance =
            MyTokenBuilder::new(token_contract_id.to_string(), owner.clone()).build();

        mint(&token_instance).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_mint_closed() {
        let (.., token_instance) = setup_and_initialize().await;
        set_mint_amount(&token_instance, 0).await;
        mint(&token_instance).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_already_mint() {
        let (.., token_instance) = setup_and_initialize().await;
        mint(&token_instance).await;
        mint(&token_instance).await;
    }
}
