mod utils;

use fuels::{prelude::*, tx::AssetId};

use utils::{
    abi_calls::{
        constructor, create_proposal, deposit, unlock_votes, user_balance, user_votes, vote,
        withdraw,
    },
    test_helpers::{mint, proposal, setup},
    GovToken, Identity, ProposalInfo,
};

// TODO: Until the SDK supports block manipulation changing tests may break them because of the
//       specifically selected block deadlines so your test might be correct but the deadline is
//       messing up the test
// - votes
//     - panics_on_expired_proposal (need SDK to manipulate block height)
//
// When logging is deserialized in the SDK, check logs are correct

mod constructor {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn constructs() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_when_reinitialized() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;
            constructor(&deployer, gov_token_id).await;
        }
    }
}

mod create_proposal {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_create_proposal() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;

            assert_eq!(
                user.dao_voting.proposal(0).call().await.unwrap().value,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction,
                    deadline: 13,
                }
            );
        }

        #[tokio::test]
        async fn user_can_create_multiple_proposals() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;
            assert_eq!(
                user.dao_voting.proposal(0).call().await.unwrap().value,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 13,
                }
            );

            create_proposal(&user, 20, 20, proposal_transaction.clone()).await;
            assert_eq!(
                user.dao_voting.proposal(1).call().await.unwrap().value,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 20,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 25,
                }
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_incorrect_proposal_id() {
            let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
            user.dao_voting.proposal(0).call().await.unwrap();
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_incorrect_deadline() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&deployer, 0, 10, proposal_transaction.clone()).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_zero_acceptance_percentage() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&deployer, 10, 0, proposal_transaction.clone()).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_over_hundred_acceptance_percentage() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&deployer, 101, 10, proposal_transaction.clone()).await;
        }
    }
}

mod deposit {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_deposit() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            constructor(&deployer, gov_token_id).await;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                0
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;

            assert_eq!(
                deployer.dao_voting.balance().call().await.unwrap().value,
                asset_amount
            );

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                asset_amount
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_when_not_initialized() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;
            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_incorrect_asset() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

            let another_asset_id = Contract::deploy_with_salt(
                "./tests/artifacts/asset/out/debug/asset.bin",
                &deployer.wallet,
                TxParameters::default(),
                Salt::from([1u8; 32]),
            )
            .await
            .unwrap();

            let another_asset =
                GovToken::new(another_asset_id.to_string(), deployer.wallet.clone());

            mint(&another_asset, 100, user.wallet.address()).await;

            constructor(&deployer, gov_token_id).await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*another_asset_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_on_zero_deposit() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            constructor(&deployer, gov_token_id).await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params =
                CallParameters::new(Some(0), Some(AssetId::from(*gov_token_id)), Some(100_000));
            deposit(&user, tx_params, call_params).await;
        }
    }
}

mod withdraw {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_withdraw() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            constructor(&deployer, gov_token_id).await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;

            assert_eq!(
                deployer.dao_voting.balance().call().await.unwrap().value,
                asset_amount
            );

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                asset_amount
            );

            withdraw(&user, asset_amount).await;

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                0
            );

            assert_eq!(user.dao_voting.balance().call().await.unwrap().value, 0);
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_not_enough_assets() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            constructor(&deployer, gov_token_id).await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;
            withdraw(&user, asset_amount * 100).await;
        }
    }
}

mod vote {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_vote() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;

            vote(&user, true, 0, asset_amount / 4).await;
            vote(&user, false, 0, asset_amount / 4).await;

            assert_eq!(
                user.dao_voting.proposal(0).call().await.unwrap().value,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: asset_amount / 4,
                    no_votes: asset_amount / 4,
                    acceptance_percentage: 10,
                    proposal_transaction,
                    deadline: 15,
                }
            );

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                6
            );

            assert_eq!(
                user_votes(&user, Identity::Address(user.wallet.address()), 0).await,
                4
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_invalid_vote_amount() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 4).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_on_expired_proposal() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 1, 1, proposal_transaction.clone()).await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;
            vote(&user, true, 0, asset_amount / 4).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_on_not_enough_balance() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;
            vote(&user, true, 10, asset_amount / 4).await;
        }
    }
}

mod execute {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_proposal_can_execute() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 2).await;

            assert_eq!(
                user.dao_voting.proposal(0).call().await.unwrap().value,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 5,
                    no_votes: 0,
                    proposal_transaction,
                    deadline: 15,
                    acceptance_percentage: 10
                }
            );

            // TODO actually test execution of an arbitrary transaction
        }
    }
}

mod unlock_votes {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_unlock_tokens() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            mint(
                &deployer.gov_token.as_ref().unwrap(),
                100,
                user.wallet.address(),
            )
            .await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            deposit(&user, tx_params, call_params).await;

            let proposal_transaction = proposal(gov_token_id);
            create_proposal(&user, 1, 1, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 2).await;

            assert_eq!(
                user.dao_voting.proposal(0).call().await.unwrap().value,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 5,
                    no_votes: 0,
                    proposal_transaction,
                    deadline: 6,
                    acceptance_percentage: 1
                }
            );

            unlock_votes(&user, 0).await;

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                asset_amount
            );
        }
    }
}
