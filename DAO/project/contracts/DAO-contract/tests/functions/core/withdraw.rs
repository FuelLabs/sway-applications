use crate::utils::{
    interface::core::{constructor, deposit, withdraw},
    setup::{mint, setup},
};
use fuels::{prelude::CallParameters, tx::AssetId};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::{balance, user_balance},
        setup::WithdrawEvent,
    };
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn user_can_withdraw() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;

        assert_eq!(balance(&user.dao_voting).await, asset_amount);

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );

        let response = withdraw(&user.dao_voting, asset_amount).await;

        let log = response.get_logs_with_type::<WithdrawEvent>().unwrap();
        let event = log.get(0).unwrap();

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
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;
        withdraw(&user.dao_voting, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn on_not_enough_assets() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;
        withdraw(&user.dao_voting, asset_amount * 100).await;
    }
}
