use crate::utils::{
    abi_calls::{constructor, create_proposal, proposal},
    test_helpers::{proposal_transaction, setup},
    Identity, ProposalInfo,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn user_can_create_proposal() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, false);
        create_proposal(&user.dao_voting, 10, 10, governor_id, proposal_transaction.clone()).await;

        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 10,
                id: governor_id,
                proposal_transaction,
                deadline: 15,
                executed: false,
            }
        );
    }

    #[tokio::test]
    async fn user_can_create_multiple_proposals() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&user.dao_voting, 10, 10, governor_id, proposal_transaction.clone()).await;
        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 10,
                id: governor_id,
                proposal_transaction: proposal_transaction.clone(),
                deadline: 15,
                executed: false,
            }
        );

        create_proposal(&user.dao_voting, 20, 20, governor_id, proposal_transaction.clone()).await;
        assert_eq!(
            proposal(&user.dao_voting, 1).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 20,
                id: governor_id,
                proposal_transaction: proposal_transaction.clone(),
                deadline: 27,
                executed: false,
            }
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_duration_is_zero() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&deployer.dao_voting, 10, 0, governor_id, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn with_zero_acceptance_percentage() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&deployer.dao_voting, 0, 10, governor_id, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn with_over_hundred_acceptance_percentage() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&deployer.dao_voting, 101, 10, governor_id, proposal_transaction.clone()).await;
    }
}
