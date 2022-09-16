use crate::utils::{
    abi_calls::{constructor, create_proposal, deposit, proposal, user_balance, user_votes, vote},
    test_helpers::{mint, proposal_transaction, setup},
    Identity, ProposalInfo, Votes,
};
use fuels::{prelude::CallParameters, tx::AssetId};

mod success {
    use super::*;

    #[tokio::test]
    async fn user_can_vote() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            &deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, false);
        create_proposal(
            &user.dao_voting,
            10,
            10,
            governor_id,
            proposal_transaction.clone(),
        )
        .await;

        vote(&user.dao_voting, true, 0, asset_amount / 4).await;
        vote(&user.dao_voting, false, 0, asset_amount / 4).await;

        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: asset_amount / 4,
                no_votes: asset_amount / 4,
                acceptance_percentage: 10,
                id: governor_id,
                proposal_transaction,
                deadline: 17,
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
        let (_gov_token, gov_token_id, deployer, user, asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            &deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, false);
        create_proposal(
            &user.dao_voting,
            10,
            10,
            governor_id,
            proposal_transaction.clone(),
        )
        .await;

        vote(&user.dao_voting, true, 0, asset_amount / 4).await;
        vote(&user.dao_voting, false, 0, asset_amount / 4).await;

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

        create_proposal(
            &user.dao_voting,
            20,
            20,
            governor_id,
            proposal_transaction.clone(),
        )
        .await;

        vote(&user.dao_voting, true, 1, asset_amount / 4).await;

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
    #[should_panic(expected = "Revert(42)")]
    async fn on_invalid_proposal_id() {
        let (_gov_token, _gov_token_id, _deployer, user, _asset_amount, _) = setup().await;
        vote(&user.dao_voting, true, 0, 10).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_zero_vote_amount() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, false);
        create_proposal(
            &user.dao_voting,
            10,
            10,
            governor_id,
            proposal_transaction.clone(),
        )
        .await;
        vote(&user.dao_voting, true, 0, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_expired_proposal() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            &deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, false);
        create_proposal(
            &user.dao_voting,
            1,
            1,
            governor_id,
            proposal_transaction.clone(),
        )
        .await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;
        vote(&user.dao_voting, true, 0, asset_amount / 4).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_vote_amount_greater_than_balance() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, false);
        create_proposal(
            &user.dao_voting,
            10,
            10,
            governor_id,
            proposal_transaction.clone(),
        )
        .await;
        vote(&user.dao_voting, true, 10, asset_amount).await;
    }
}
