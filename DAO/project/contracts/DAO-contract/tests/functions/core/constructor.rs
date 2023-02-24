use crate::utils::{interface::core::constructor, setup::setup};

mod success {
    use super::*;
    use crate::utils::{interface::info::governance_token_id, setup::InitializeEvent};
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn constructs() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        let response = constructor(&deployer.dao_voting, gov_token_id).await;

        let log = response.get_logs_with_type::<InitializeEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            InitializeEvent {
                author: Identity::Address(Address::from(deployer.wallet.address())),
                token: gov_token_id
            }
        );
        assert_eq!(
            governance_token_id(&deployer.dao_voting).await,
            gov_token_id
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialize")]
    async fn when_reinitialized() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;
        constructor(&deployer.dao_voting, gov_token_id).await;
    }
}
