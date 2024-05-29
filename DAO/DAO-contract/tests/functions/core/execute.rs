use crate::utils::{
    interface::core::{constructor, create_proposal, deposit, execute, vote},
    setup::{proposal_transaction, setup},
};
use fuels::{prelude::CallParameters, types::AssetId};

mod success {
    use super::*;
    use crate::utils::setup::ExecuteEvent;
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    #[ignore]
    async fn user_proposal_can_execute() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 1, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        let response = execute(&user.dao_voting, 0).await;

        let log = response.decode_logs_with_type::<ExecuteEvent>().unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            ExecuteEvent {
                id: 0,
                acceptance_percentage: 10,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        // TODO actually test execution of an arbitrary transaction
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidId")]
    async fn on_invalid_proposal_id() {
        let (_gov_asset, _gov_asset_id, _deployer, user, _asset_amount) = setup().await;
        execute(&user.dao_voting, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ProposalExecuted")]
    #[ignore]
    async fn on_already_executed_proposal() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 1, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        execute(&user.dao_voting, 0).await;
        execute(&user.dao_voting, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ProposalStillActive")]
    pub async fn on_active_proposal() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 100, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        execute(&user.dao_voting, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    pub async fn on_not_enough_yes_votes() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 1, proposal_transaction.clone()).await;
        vote(&user.dao_voting, false, 0, asset_amount / 2).await;

        execute(&user.dao_voting, 0).await;
    }

    // TODO add test for reverting on a failed proposal call
}
