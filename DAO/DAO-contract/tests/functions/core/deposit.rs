use crate::utils::{
    interface::core::{constructor, deposit},
    setup::setup,
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
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;

        constructor(&deployer.dao_voting, gov_asset_id).await;

        assert_eq!(balance(&user.dao_voting).await, 0);

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            0
        );

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        let response = deposit(&user.dao_voting, call_params).await;

        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();

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

    #[tokio::test]
    #[should_panic(expected = "ContractNotInitialized")]
    async fn when_not_initialized() {
        let (gov_asset_id, _other_asset_id, _deployer, user, asset_amount) = setup().await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetSent")]
    async fn with_incorrect_asset() {
        let (gov_asset_id, other_asset_id, deployer, _user, asset_amount) = setup().await;

        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params =
            CallParameters::new(asset_amount, AssetId::from(*other_asset_id), 100_000);
        deposit(&deployer.dao_voting, call_params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AmountCannotBeZero")]
    async fn on_zero_deposit() {
        let (gov_asset_id, _other_asset_id, deployer, user, _asset_amount) = setup().await;

        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(0, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
    }
}
