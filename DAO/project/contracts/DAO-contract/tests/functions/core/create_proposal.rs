use crate::utils::{
    interface::core::{constructor, create_proposal},
    setup::{proposal_transaction, setup},
};
<<<<<<< HEAD:DAO/project/DAO-contract/tests/functions/create_proposal.rs
use fuels::types::Identity;
=======
>>>>>>> origin/master:DAO/project/contracts/DAO-contract/tests/functions/core/create_proposal.rs

mod success {
    use super::*;
    use crate::utils::{
        interface::info::proposal,
        setup::{CreateProposalEvent, ProposalInfo},
    };
    use fuels::types::Identity;

    #[tokio::test]
    async fn user_can_create_proposal() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        let response =
            create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        let log = response
            .get_logs_with_type::<CreateProposalEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        let expected_proposal = ProposalInfo {
            author: Identity::Address(user.wallet.address().into()),
            yes_votes: 0,
            no_votes: 0,
            acceptance_percentage: 10,
            proposal_transaction: proposal_transaction.clone(),
            deadline: 14,
            executed: false,
        };

        assert_eq!(
            *event,
            CreateProposalEvent {
                proposal_info: expected_proposal.clone(),
                id: 0
            }
        );
        assert_eq!(proposal(&user.dao_voting, 0).await, expected_proposal);
    }

    #[tokio::test]
    async fn user_can_create_multiple_proposals() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        let response =
            create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        let log = response
            .get_logs_with_type::<CreateProposalEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            CreateProposalEvent {
                proposal_info: ProposalInfo {
                    author: Identity::Address(user.wallet.address().into()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 14,
                    executed: false,
                },
                id: 0
            }
        );
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

        let response =
            create_proposal(&user.dao_voting, 20, 20, proposal_transaction.clone()).await;

        let log = response
            .get_logs_with_type::<CreateProposalEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            CreateProposalEvent {
                proposal_info: ProposalInfo {
                    author: Identity::Address(user.wallet.address().into()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 20,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 26,
                    executed: false,
                },
                id: 1
            }
        );
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
    #[should_panic(expected = "DurationCannotBeZero")]
    async fn when_duration_is_zero() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&deployer.dao_voting, 10, 0, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAcceptancePercentage")]
    async fn with_zero_acceptance_percentage() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&deployer.dao_voting, 0, 10, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAcceptancePercentage")]
    async fn with_over_hundred_acceptance_percentage() {
        let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&deployer.dao_voting, 101, 10, proposal_transaction.clone()).await;
    }
}
