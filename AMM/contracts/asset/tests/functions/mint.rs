use crate::utils::{
    abi_calls::{mint, set_mint_amount},
    test_helpers::{build_contract, setup_and_initialize},
    AssetBuilder,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_mint_and_transfer_coins() {
        let (owner, _minter, mint_amount, asset_contract_id, asset_instance) =
            setup_and_initialize().await;

        mint(&asset_instance).await;
        let coin_id = AssetId::from(*asset_contract_id.hash());
        let owner_balance = owner.get_asset_balance(&coin_id).await.unwrap();
        assert_eq!(owner_balance, mint_amount);
    }

    #[tokio::test]
    async fn can_mint_and_transfer_alternative_assets() {
        let (_owner, minter, mint_amount, asset_contract_id, _asset_instance) =
            setup_and_initialize().await;

        let asset_instance_alternative =
            build_contract(asset_contract_id.clone(), minter.clone()).await;

        mint(&asset_instance_alternative).await;

        let coin_id = AssetId::from(*asset_contract_id.hash());
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

        let asset_contract_id = Contract::deploy(
            "../asset/out/debug/asset.bin",
            &owner,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let asset_instance =
            AssetBuilder::new(asset_contract_id.to_string(), owner.clone()).build();

        mint(&asset_instance).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_mint_closed() {
        let (.., asset_instance) = setup_and_initialize().await;
        set_mint_amount(&asset_instance, 0).await;
        mint(&asset_instance).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_already_mint() {
        let (.., asset_instance) = setup_and_initialize().await;
        mint(&asset_instance).await;
        mint(&asset_instance).await;
    }
}
