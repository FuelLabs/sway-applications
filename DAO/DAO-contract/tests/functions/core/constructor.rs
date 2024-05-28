use crate::utils::{interface::core::constructor, setup::setup};

mod success {
    use super::*;
    use crate::utils::{interface::info::governance_asset_id, setup::InitializeEvent};
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn constructs() {
        let (gov_asset_id, _other_asset_id, deployer, _user, _asset_amount) = setup().await;
        let response = constructor(&deployer.dao_voting, gov_asset_id).await;

        let log = response.decode_logs_with_type::<InitializeEvent>().unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            InitializeEvent {
                author: Identity::Address(Address::from(deployer.wallet.address())),
                asset: gov_asset_id
            }
        );
        assert_eq!(
            governance_asset_id(&deployer.dao_voting).await,
            gov_asset_id
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialize")]
    async fn when_reinitialized() {
        let (gov_asset_id, _other_asset_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;
        constructor(&deployer.dao_voting, gov_asset_id).await;
    }
}
