use crate::utils::{
    abi_calls::{constructor, governance_token_id},
    test_helpers::setup,
};

mod success {
    use super::*;

    #[tokio::test]
    pub async fn user_can_get_governance_token_id() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;
        assert_eq!(
            governance_token_id(&deployer.dao_voting).await,
            gov_token_id
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    pub async fn on_not_inialized() {
        let (_gov_token, _gov_token_id, deployer, _user, _asset_amount) = setup().await;
        governance_token_id(&deployer.dao_voting).await;
    }
}
