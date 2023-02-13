use crate::utils::{
    abi_calls::{
        constructor, create_proposal, deposit, unlock_votes, user_balance, user_votes, vote,
    },
    test_helpers::{mint, proposal_transaction, setup},
    Votes,
};
use fuels::{prelude::CallParameters, tx::AssetId};

mod success {
    use super::*;

    #[tokio::test]
    async fn user_can_unlock_tokens() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
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

        let proposal_transaction = proposal_transaction(gov_token_id);
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

        unlock_votes(&user.dao_voting, 0).await;

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
    async fn user_can_unlock_tokens_from_simultaneous_proposals() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&user.dao_voting, 1, 3, proposal_transaction.clone()).await;
        create_proposal(&user.dao_voting, 10, 4, proposal_transaction.clone()).await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
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

        unlock_votes(&user.dao_voting, 0).await;
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
        unlock_votes(&user.dao_voting, 1).await;
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
    async fn user_can_unlock_tokens_from_multiple_proposals() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
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

        let proposal_transaction = proposal_transaction(gov_token_id);
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

        unlock_votes(&user.dao_voting, 0).await;

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

        unlock_votes(&user.dao_voting, 1).await;

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
        let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
        unlock_votes(&user.dao_voting, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ProposalStillActive")]
    pub async fn on_active_proposal() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
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

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&user.dao_voting, 10, 100, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;
        unlock_votes(&user.dao_voting, 0).await;
    }
}
