use crate::utils::{
    abi_calls::{balance, constructor, deposit, user_balance},
    test_helpers::{mint, setup},
    GovToken,
};
use fuels::{
    prelude::{CallParameters, Contract, StorageConfiguration, TxParameters},
    tx::{AssetId, ContractId, Salt},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn user_can_deposit() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            &deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        assert_eq!(balance(&user.dao_voting).await, 0);

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            0
        );

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;

        // Make sure that deposit did not erroneously work with 0
        assert!(asset_amount != 0);

        assert_eq!(balance(&user.dao_voting).await, asset_amount);

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "ContractNotInitialized")]
    async fn when_not_initialized() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            &deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetSent")]
    async fn with_incorrect_asset() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        let another_asset_id = Contract::deploy_with_parameters(
            "./tests/artifacts/gov_token/out/debug/gov_token.bin",
            &deployer.wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./tests/artifacts/gov_token/out/debug/gov_token-storage_slots.json".to_string(),
            )),
            Salt::from([1u8; 32]),
        )
        .await
        .unwrap();

        let another_asset = GovToken::new(another_asset_id.clone(), deployer.wallet.clone());
        let id: ContractId = another_asset_id.into();

        mint(&another_asset, asset_amount, user.wallet.address()).await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params =
            CallParameters::new(Some(asset_amount), Some(AssetId::from(*id)), Some(100_000));
        deposit(&user.dao_voting, call_params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AmountCannotBeZero")]
    async fn on_zero_deposit() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            &deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params =
            CallParameters::new(Some(0), Some(AssetId::from(*gov_token_id)), Some(100_000));
        deposit(&user.dao_voting, call_params).await;
    }
}
