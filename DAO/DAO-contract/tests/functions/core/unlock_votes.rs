use crate::utils::{
    interface::core::{constructor, create_proposal, deposit, unlock_votes, vote},
    setup::{proposal_transaction, setup, Votes},
};
use fuels::{prelude::CallParameters, types::AssetId};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::{user_balance, user_votes},
        setup::UnlockVotesEvent,
    };
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn user_can_unlock_assets() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 1, 1, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount / 2
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: asset_amount / 2,
                no_votes: 0
            }
        );

        let response = unlock_votes(&user.dao_voting, 0).await;

        let log = response
            .decode_logs_with_type::<UnlockVotesEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            UnlockVotesEvent {
                id: 0,
                vote_amount: asset_amount / 2,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: 0,
                no_votes: 0
            }
        );
    }

    #[tokio::test]
    async fn user_can_unlock_assets_from_simultaneous_proposals() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 1, 3, proposal_transaction.clone()).await;
        create_proposal(&user.dao_voting, 10, 4, proposal_transaction.clone()).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        vote(&user.dao_voting, true, 0, asset_amount / 2).await;
        vote(&user.dao_voting, true, 1, asset_amount / 2).await;
        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            0
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: asset_amount / 2,
                no_votes: 0
            }
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 1).await,
            Votes {
                yes_votes: asset_amount / 2,
                no_votes: 0
            }
        );

        let response1 = unlock_votes(&user.dao_voting, 0).await;

        let log1 = response1
            .decode_logs_with_type::<UnlockVotesEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();

        assert_eq!(
            *event1,
            UnlockVotesEvent {
                id: 0,
                vote_amount: asset_amount / 2,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount / 2
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: 0,
                no_votes: 0
            }
        );

        let response2 = unlock_votes(&user.dao_voting, 1).await;

        let log2 = response2
            .decode_logs_with_type::<UnlockVotesEvent>()
            .unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event2,
            UnlockVotesEvent {
                id: 1,
                vote_amount: asset_amount / 2,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );
        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 1).await,
            Votes {
                yes_votes: 0,
                no_votes: 0
            }
        );
    }

    #[tokio::test]
    async fn user_can_unlock_assets_from_multiple_proposals() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 1, 1, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount / 2
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: asset_amount / 2,
                no_votes: 0
            }
        );

        let response1 = unlock_votes(&user.dao_voting, 0).await;

        let log1 = response1
            .decode_logs_with_type::<UnlockVotesEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();

        assert_eq!(
            *event1,
            UnlockVotesEvent {
                id: 0,
                vote_amount: asset_amount / 2,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: 0,
                no_votes: 0
            }
        );

        create_proposal(&user.dao_voting, 10, 1, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 1, asset_amount / 2).await;

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount / 2
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 1).await,
            Votes {
                yes_votes: asset_amount / 2,
                no_votes: 0
            }
        );

        let response2 = unlock_votes(&user.dao_voting, 1).await;

        let log2 = response2
            .decode_logs_with_type::<UnlockVotesEvent>()
            .unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event2,
            UnlockVotesEvent {
                id: 1,
                vote_amount: asset_amount / 2,
                user: Identity::Address(Address::from(user.wallet.address()))
            }
        );

        assert_eq!(
            user_balance(&user.dao_voting, user.wallet.address()).await,
            asset_amount
        );
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 1).await,
            Votes {
                yes_votes: 0,
                no_votes: 0
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
        unlock_votes(&user.dao_voting, 0).await;
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
        unlock_votes(&user.dao_voting, 0).await;
    }
}
