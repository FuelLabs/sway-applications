mod success {
    use crate::utils::{
        interface::{
            core::{constructor, deposit},
            info::balance,
        },
        setup::setup,
    };
    use fuels::{prelude::CallParameters, types::AssetId};

    #[tokio::test]
    pub async fn user_can_check_contract_balance() {
        let (gov_token_id, _other_token_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_token_id), 100_000);
        assert_eq!(balance(&user.dao_voting).await, 0);
        deposit(&user.dao_voting, call_params).await;
        assert_eq!(balance(&user.dao_voting).await, asset_amount);
    }
}
