mod utils;

use fuels::{
    prelude::{CallParameters, Contract, TxParameters},
    signers::Signer,
    tx::{AssetId, Salt},
};

use utils::{
    abi_calls::{
        balance, constructor, create_proposal, deposit, execute, governance_token_id, proposal,
        unlock_votes, user_balance, user_votes, vote, withdraw, proposal_count,
    },
    test_helpers::{mint, proposal_transaction, setup},
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;

            assert_eq!(
                proposal(&user, 0).await,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction,
                    deadline: 13,
                    executed: false,
                }
            );
        }

        #[tokio::test]
        async fn user_can_create_multiple_proposals() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;
            assert_eq!(
                proposal(&user, 0).await,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 13,
                    executed: false,
                }
            );

            create_proposal(&user, 20, 20, proposal_transaction.clone()).await;
            assert_eq!(
                proposal(&user, 1).await,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
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
        #[should_panic]
        async fn panics_with_incorrect_deadline() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&deployer, 10, 0, proposal_transaction.clone()).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_zero_acceptance_percentage() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&deployer, 0, 10, proposal_transaction.clone()).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_over_hundred_acceptance_percentage() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
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

            assert_eq!(balance(&user).await, 0);

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

            // Make sure that deposit did not erroneously work with 0
            assert!(asset_amount != 0);

            assert_eq!(balance(&user).await, asset_amount);

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

            assert_eq!(balance(&user).await, asset_amount);

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                asset_amount
            );

            withdraw(&user, asset_amount).await;

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                0
            );

            assert_eq!(balance(&user).await, 0);
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_withdraw_zero() {
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
            withdraw(&user, 0).await;
        }

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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;

            vote(&user, true, 0, asset_amount / 4).await;
            vote(&user, false, 0, asset_amount / 4).await;

            assert_eq!(
                proposal(&user, 0).await,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: asset_amount / 4,
                    no_votes: asset_amount / 4,
                    acceptance_percentage: 10,
                    proposal_transaction,
                    deadline: 15,
                    executed: false,
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

        #[tokio::test]
        async fn user_can_vote_on_multiple_proposals() {
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;

            vote(&user, true, 0, asset_amount / 4).await;
            vote(&user, false, 0, asset_amount / 4).await;

            assert_eq!(
                proposal(&user, 0).await,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: asset_amount / 4,
                    no_votes: asset_amount / 4,
                    acceptance_percentage: 10,
                    proposal_transaction: proposal_transaction.clone(),
                    deadline: 15,
                    executed: false
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

            create_proposal(&user, 20, 20, proposal_transaction.clone()).await;
            vote(&user, true, 1, 2).await;

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                4
            );
            assert_eq!(
                user_votes(&user, Identity::Address(user.wallet.address()), 1).await,
                2
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_invalid_proposal_id() {
            let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
            vote(&user, true, 0, 10).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_on_zero_vote_amount() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;
            vote(&user, true, 0, 0).await;
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

            let proposal_transaction = proposal_transaction(gov_token_id);
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
        async fn panics_on_vote_amount_greater_than_balance() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;
            vote(&user, true, 10, asset_amount).await;
        }
    }
}

mod execute {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        #[ignore]
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 1, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 2).await;

            execute(&user, 0).await;

            // TODO actually test execution of an arbitrary transaction
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_invalid_proposal_id() {
            let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
            execute(&user, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        #[ignore]
        async fn panics_on_already_executed_proposal() {
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 1, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 2).await;

            execute(&user, 0).await;
            execute(&user, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        pub async fn panics_on_active_proposal() {
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 100, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 2).await;

            execute(&user, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        pub async fn panics_on_not_enough_yes_votes() {
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 100, proposal_transaction.clone()).await;
            vote(&user, false, 0, asset_amount / 2).await;

            execute(&user, 0).await;
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 1, 1, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 2).await;

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                asset_amount / 2
            );
            assert_eq!(
                user_votes(&user, Identity::Address(user.wallet.address()), 0).await,
                asset_amount / 2
            );

            unlock_votes(&user, 0).await;

            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                asset_amount
            );
            assert_eq!(
                user_votes(&user, Identity::Address(user.wallet.address()), 0).await,
                0
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_invalid_proposal_id() {
            let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
            unlock_votes(&user, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        pub async fn panics_on_active_proposal() {
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

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 100, proposal_transaction.clone()).await;
            vote(&user, true, 0, asset_amount / 2).await;
            unlock_votes(&user, 0).await;
        }
    }
}

mod balance {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        pub async fn user_can_check_contract_balance() {
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
            assert_eq!(balance(&user).await, 0);
            deposit(&user, tx_params, call_params).await;
            assert_eq!(balance(&user).await, asset_amount);
        }
    }
}

mod user_balance {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        pub async fn user_can_check_user_balance() {
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
            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                0
            );
            deposit(&user, tx_params, call_params).await;
            assert_eq!(
                user_balance(&user, Identity::Address(user.wallet.address())).await,
                asset_amount
            );
        }
    }
}

mod user_votes {
    use super::*;

    mod sucess {
        use super::*;

        #[tokio::test]
        pub async fn user_can_check_user_votes() {
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
            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction).await;
            assert_eq!(
                user_votes(&user, Identity::Address(user.wallet.address()), 0).await,
                0
            );
            vote(&user, true, 0, asset_amount).await;
            assert_eq!(
                user_votes(&user, Identity::Address(user.wallet.address()), 0).await,
                asset_amount
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        pub async fn panics_on_invalid_proposal_id() {
            let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
            user_votes(&user, Identity::Address(user.wallet.address()), 0).await;
        }
    }
}

mod proposal {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        pub async fn user_can_get_proposal() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;

            assert_eq!(
                proposal(&user, 0).await,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction,
                    deadline: 13,
                    executed: false,
                }
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_invalid_proposal_id() {
            let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
            proposal(&user, 0).await;
        }
    }
}

mod governance_token_id {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        pub async fn user_can_get_governance_token_id() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;
            assert_eq!(governance_token_id(&deployer).await, gov_token_id);
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        pub async fn panics_on_not_inialized() {
            let (_gov_token, _gov_token_id, deployer, _user, _asset_amount) = setup().await;
            governance_token_id(&deployer).await;
        }
    }
}

mod proposal_count {
    use super::*;

    mod success {
        use super::*; 
        
        #[tokio::test]
        async fn use_can_get_proposal_count() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let proposal_transaction = proposal_transaction(gov_token_id);
            create_proposal(&user, 10, 10, proposal_transaction.clone()).await;

            assert_eq!(
                proposal_count(&user).await,
                1
            );
        }
    }
}
