use crate::utils::{
    abi_calls::{constructor, governance_token_id},
    test_helpers::setup,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn constructs() {
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
    #[should_panic]
    async fn panics_when_reinitialized() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;
        constructor(&deployer.dao_voting, gov_token_id).await;
    }
}
