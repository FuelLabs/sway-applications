use crate::utils::{
    interface::core::{constructor, create_proposal},
    setup::{proposal_transaction, setup},
};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::proposal,
        setup::{CreateProposalEvent, ProposalInfo},
    };
    use fuels::types::Identity;

    #[tokio::test]
    async fn user_can_create_proposal() {
        let (gov_asset_id, _other_asset_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        let response =
            create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        let log = response
            .decode_logs_with_type::<CreateProposalEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let expected_proposal = ProposalInfo {
            author: Identity::Address(user.wallet.address().into()),
            yes_votes: 0,
            no_votes: 0,
            acceptance_percentage: 10,
            proposal_transaction: proposal_transaction.clone(),
            deadline: 13,
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
        let (gov_asset_id, _other_asset_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        let response =
            create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        let log = response
            .decode_logs_with_type::<CreateProposalEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            CreateProposalEvent {
                proposal_info: ProposalInfo {
                    author: Identity::Address(user.wallet.address().into()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 13,
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
                deadline: 13,
                executed: false,
            }
        );

        let response =
            create_proposal(&user.dao_voting, 20, 20, proposal_transaction.clone()).await;

        let log = response
            .decode_logs_with_type::<CreateProposalEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            CreateProposalEvent {
                proposal_info: ProposalInfo {
                    author: Identity::Address(user.wallet.address().into()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 20,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 25,
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
                deadline: 25,
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
        let (gov_asset_id, _other_asset_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&deployer.dao_voting, 10, 0, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAcceptancePercentage")]
    async fn with_zero_acceptance_percentage() {
        let (gov_asset_id, _other_asset_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&deployer.dao_voting, 0, 10, proposal_transaction.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAcceptancePercentage")]
    async fn with_over_hundred_acceptance_percentage() {
        let (gov_asset_id, _other_asset_id, deployer, _user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&deployer.dao_voting, 101, 10, proposal_transaction.clone()).await;
    }
}
