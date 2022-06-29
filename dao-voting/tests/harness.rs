mod utils;

use fuels::{prelude::*, tx::AssetId};

use utils::{
    abi_calls::{constructor},
    test_helpers::{get_call_data, setup},
    GovToken, Identity, ProposalInfo,
};

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
        use crate::utils::Identity;

        use super::*;

        #[tokio::test]
        async fn user_can_create_proposal() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let call_data = get_call_data(gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    proposal_transaction: call_data,
                    deadline: 13,
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
        async fn panics_with_incorrect_voting_period() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let call_data = get_call_data(gov_token_id);

            deployer
                .dao_voting
                .create_proposal(0, 10, call_data)
                .call()
                .await
                .unwrap()
                .value;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_zero_acceptance_percentage() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let call_data = get_call_data(gov_token_id);

            deployer
                .dao_voting
                .create_proposal(10, 0, call_data)
                .call()
                .await
                .unwrap()
                .value;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_over_hundred_acceptance_percentage() {
            let (_gov_token, gov_token_id, deployer, _user, _asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let call_data = get_call_data(gov_token_id);

            deployer
                .dao_voting
                .create_proposal(101, 10, call_data)
                .call()
                .await
                .unwrap()
                .value;
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

            assert!(
                deployer
                    .gov_token
                    .as_ref()
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            constructor(&deployer, gov_token_id).await;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                0
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;

            assert_eq!(
                deployer.dao_voting.balance().call().await.unwrap().value,
                asset_amount
            );

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                asset_amount
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_with_incorrect_amount() {
            let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;

            assert!(
                deployer
                    .gov_token
                    .as_ref()
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            constructor(&deployer, gov_token_id).await;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);
            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params =
                CallParameters::new(Some(0), Some(AssetId::from(*gov_token_id)), Some(100_000));
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_when_not_initialized() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

            assert!(
                deployer
                    .gov_token
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;
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

            assert!(
                another_asset
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            constructor(&deployer, gov_token_id).await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;
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

            assert!(
                deployer
                    .gov_token
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(true, 0, asset_amount / 4)
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(false, 0, asset_amount / 4)
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: asset_amount / 4,
                    no_votes: asset_amount / 4,
                    acceptance_percentage: 10,
                    proposal_transaction: call_data,
                    deadline: 15,
                }
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_not_enough_votes() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            let call_data = get_call_data(gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data)
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(true, 0, asset_amount / 4)
                .call()
                .await
                .unwrap()
                .value;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_on_expired_proposal() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            assert!(
                deployer
                    .gov_token
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            let call_data = get_call_data(gov_token_id);

            user.dao_voting
                .create_proposal(1, 1, call_data)
                .call()
                .await
                .unwrap()
                .value;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(true, 0, asset_amount / 4)
                .call()
                .await
                .unwrap()
                .value;
        }
    }
}

mod execute_proposal {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_proposal_can_execute() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            assert!(
                deployer
                    .gov_token
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(true, 0, asset_amount / 2)
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 5,
                    no_votes: 0,
                    proposal_transaction: call_data,
                    deadline: 15,
                    acceptance_percentage: 10
                }
            );

            // TODO actually test execution of an arbitrary transaction
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

            assert!(
                deployer
                    .gov_token
                    .as_ref()
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            constructor(&deployer, gov_token_id).await;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                0
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;

            assert_eq!(
                deployer.dao_voting.balance().call().await.unwrap().value,
                asset_amount
            );

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                asset_amount
            );

            user.dao_voting
                .withdraw(asset_amount)
                .append_variable_outputs(1)
                .call()
                .await
                .unwrap()
                .value;

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
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

            assert!(
                deployer
                    .gov_token
                    .as_ref()
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            constructor(&deployer, gov_token_id).await;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                0
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;

            assert_eq!(
                deployer.dao_voting.balance().call().await.unwrap().value,
                asset_amount
            );

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                asset_amount
            );

            user.dao_voting
                .withdraw(asset_amount * 100)
                .append_variable_outputs(1)
                .call()
                .await
                .unwrap()
                .value;
        }
    }
}

mod convert_votes {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_unlock_tokens() {
            let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            constructor(&deployer, gov_token_id).await;

            assert!(
                deployer
                    .gov_token
                    .unwrap()
                    .mint_and_send_to_address(100, user.wallet.address())
                    .append_variable_outputs(1)
                    .call()
                    .await
                    .unwrap()
                    .value
            );

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(
                Some(asset_amount),
                Some(AssetId::from(*gov_token_id)),
                Some(100_000),
            );
            user.dao_voting
                .deposit()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(gov_token_id);

            user.dao_voting
                .create_proposal(1, 1, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(true, 0, asset_amount / 2)
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                ProposalInfo {
                    author: Identity::Address(user.wallet.address()),
                    yes_votes: 5,
                    no_votes: 0,
                    proposal_transaction: call_data,
                    deadline: 6,
                    acceptance_percentage: 1
                }
            );

            user.dao_voting.unlock_votes(0).call().await.unwrap();

            assert_eq!(
                user.dao_voting
                    .user_balance(Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                asset_amount
            );
        }
    }
}
