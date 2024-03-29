mod success {
    use crate::utils::{
        interface::{
            core::{constructor, deposit},
            info::user_balance,
        },
        setup::setup,
    };
    use fuels::{prelude::CallParameters, types::AssetId};

    #[tokio::test]
    pub async fn user_can_check_user_balance() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            0
        );
        deposit(&user.dao_voting, call_params).await;
        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );
    }
}
