use crate::utils::{
    interface::core::{constructor, deposit, withdraw},
    setup::setup,
};
use fuels::{prelude::CallParameters, types::AssetId};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::{balance, user_balance},
        setup::WithdrawEvent,
    };
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn user_can_withdraw() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;

        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        assert_eq!(balance(&user.dao_voting).await, asset_amount);

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );

        let response = withdraw(&user.dao_voting, asset_amount).await;

        let log = response.decode_logs_with_type::<WithdrawEvent>().unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            WithdrawEvent {
                amount: asset_amount,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            0
        );

        assert_eq!(balance(&user.dao_voting).await, 0);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AmountCannotBeZero")]
    async fn on_withdraw_zero() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;

        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
        withdraw(&user.dao_voting, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn on_not_enough_assets() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;

        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
        withdraw(&user.dao_voting, asset_amount * 100).await;
    }
}
