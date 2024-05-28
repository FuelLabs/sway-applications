use crate::utils::{
    interface::core::{constructor, create_proposal, deposit, vote},
    setup::{proposal_transaction, setup},
};
use fuels::{prelude::CallParameters, types::AssetId};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::{proposal, user_balance, user_votes},
        setup::{ProposalInfo, VoteEvent, Votes},
    };
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn user_can_vote() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        let response1 = vote(&user.dao_voting, true, 0, asset_amount / 4).await;
        let response2 = vote(&user.dao_voting, false, 0, asset_amount / 4).await;

        let log1 = response1.decode_logs_with_type::<VoteEvent>().unwrap();
        let log2 = response2.decode_logs_with_type::<VoteEvent>().unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event1,
            VoteEvent {
                id: 0,
                vote_amount: asset_amount / 4,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );
        assert_eq!(
            *event2,
            VoteEvent {
                id: 0,
                vote_amount: asset_amount / 4,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: asset_amount / 4,
                no_votes: asset_amount / 4,
                acceptance_percentage: 10,
                proposal_transaction,
                deadline: 14,
                executed: false,
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            6
        );

        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                no_votes: 2,
                yes_votes: 2,
            }
        );
    }

    #[tokio::test]
    async fn user_can_vote_on_multiple_proposals() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        let response1 = vote(&user.dao_voting, true, 0, asset_amount / 4).await;
        let response2 = vote(&user.dao_voting, false, 0, asset_amount / 4).await;

        let log1 = response1.decode_logs_with_type::<VoteEvent>().unwrap();
        let log2 = response2.decode_logs_with_type::<VoteEvent>().unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event1,
            VoteEvent {
                id: 0,
                vote_amount: asset_amount / 4,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );
        assert_eq!(
            *event2,
            VoteEvent {
                id: 0,
                vote_amount: asset_amount / 4,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            6
        );

        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                no_votes: 2,
                yes_votes: 2,
            }
        );

        create_proposal(&user.dao_voting, 20, 20, proposal_transaction.clone()).await;

        let response3 = vote(&user.dao_voting, true, 1, asset_amount / 4).await;

        let log3 = response3.decode_logs_with_type::<VoteEvent>().unwrap();
        let event3 = log3.first().unwrap();

        assert_eq!(
            *event3,
            VoteEvent {
                id: 1,
                vote_amount: asset_amount / 4,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            4
        );

        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 1).await,
            Votes {
                yes_votes: 2,
                no_votes: 0,
            }
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidId")]
    async fn on_invalid_proposal_id() {
        let (_gov_asset, _gov_asset_id, _deployer, user, _asset_amount) = setup().await;
        vote(&user.dao_voting, true, 0, 10).await;
    }

    #[tokio::test]
    #[should_panic(expected = "VoteAmountCannotBeZero")]
    async fn on_zero_vote_amount() {
        let (gov_asset_id, _other_asset_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ProposalExpired")]
    async fn on_expired_proposal() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 1, 1, proposal_transaction.clone()).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
        vote(&user.dao_voting, true, 0, asset_amount / 4).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn on_vote_amount_greater_than_balance() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount).await;
    }
}
