use crate::utils::{interface::info::governance_asset_id, setup::setup};

mod success {
    use super::*;
    use crate::utils::interface::core::constructor;

    #[tokio::test]
    pub async fn user_can_get_governance_asset_id() {
        let (gov_asset_id, _other_asset_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;
        assert_eq!(
            governance_asset_id(&deployer.dao_voting).await,
            gov_asset_id
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "ContractNotInitialized")]
    pub async fn on_not_inialized() {
        let (_gov_asset, _gov_asset_id, deployer, _user, _asset_amount) = setup().await;
        governance_asset_id(&deployer.dao_voting).await;
    }
}
