use crate::utils::{
    abi_calls::{constructor, create_proposal, proposal},
    test_helpers::{proposal_transaction, setup},
    Identity, ProposalInfo,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn user_can_create_proposal() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 10,
                proposal_transaction,
                deadline: 14,
                executed: false,
            }
        );
    }

    #[tokio::test]
    async fn user_can_create_multiple_proposals() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;
        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 10,
                proposal_transaction: proposal_transaction.clone(),
                deadline: 14,
                executed: false,
            }
        );

        create_proposal(&user.dao_voting, 20, 20, proposal_transaction.clone()).await;
        assert_eq!(
            proposal(&user.dao_voting, 1).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 20,
                proposal_transaction: proposal_transaction.clone(),
                deadline: 26,
                executed: false,
            }
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn panics_when_duration_is_zero() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&deployer.dao_voting, 10, 0, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_with_zero_acceptance_percentage() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&deployer.dao_voting, 0, 10, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_with_over_hundred_acceptance_percentage() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&deployer.dao_voting, 101, 10, proposal_transaction.clone()).await;
    }
}
