use crate::utils::{
    interface::core::{constructor, deposit},
    setup::{mint, setup, GOVERNANCE_TOKEN_BINARY_PATH, GOVERNANCE_TOKEN_STORAGE_PATH},
};
use fuels::{prelude::CallParameters, types::AssetId};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::{balance, user_balance},
        setup::DepositEvent,
    };
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn user_can_deposit() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
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

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_token_id), 100_000);
        let response = deposit(&user.dao_voting, call_params).await;

        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            DepositEvent {
                amount: asset_amount,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

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
    use crate::utils::setup::GovToken;
    use fuels::prelude::{
        Contract, ContractId, LoadConfiguration, StorageConfiguration, TxParameters,
    };

    #[tokio::test]
    #[should_panic(expected = "ContractNotInitialized")]
    async fn when_not_initialized() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_token_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetSent")]
    async fn with_incorrect_asset() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        let storage_configuration = StorageConfiguration::load_from(GOVERNANCE_TOKEN_STORAGE_PATH);
        let configuration = LoadConfiguration::default()
            .set_storage_configuration(storage_configuration.unwrap())
            .set_salt([1u8; 32]);

        let another_asset_id = Contract::load_from(GOVERNANCE_TOKEN_BINARY_PATH, configuration)
            .unwrap()
            .deploy(&deployer.wallet, TxParameters::default())
            .await
            .unwrap();

        let another_asset = GovToken::new(another_asset_id.clone(), deployer.wallet.clone());
        let id: ContractId = another_asset_id.into();

        mint(&another_asset, asset_amount, user.wallet.address()).await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*id), 100_000);
        deposit(&user.dao_voting, call_params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AmountCannotBeZero")]
    async fn on_zero_deposit() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params = CallParameters::new(0, AssetId::from(*gov_token_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
    }
}
