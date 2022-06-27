#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::Add;

use fuels::{
    prelude::*,
    tx::{AssetId, ContractId},
};

// Load abi from json
abigen!(DaoVoting, "out/debug/dao-voting-abi.json");
abigen!(
    GovToken,
    "tests/artifacts/gov_token/out/debug/gov_token-abi.json"
);

struct Metadata {
    dao_voting: DaoVoting,
    gov_token: Option<GovToken>,
    wallet: LocalWallet,
}

async fn setup() -> (GovToken, ContractId, Metadata, Metadata, u64) {
    let num_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;
    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_provider_and_get_wallets(config).await;
    let deployer_wallet = wallets.pop().unwrap();
    let user_wallet = wallets.pop().unwrap();

    let dao_voting_id = Contract::deploy(
        "./out/debug/dao-voting.bin",
        &deployer_wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let gov_token_id = Contract::deploy(
        "./tests/artifacts/gov_token/out/debug/gov_token.bin",
        &deployer_wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let gov_token = GovToken::new(gov_token_id.to_string(), deployer_wallet.clone());

    let deployer = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id.to_string(), deployer_wallet.clone()),
        gov_token: Some(GovToken::new(
            gov_token_id.to_string(),
            deployer_wallet.clone(),
        )),
        wallet: deployer_wallet,
    };

    let user = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id.to_string(), user_wallet.clone()),
        gov_token: None,
        wallet: user_wallet,
    };

    let asset_amount: u64 = 10;

    (gov_token, gov_token_id, deployer, user, asset_amount)
}

fn get_call_data(recipient: Address, asset_id: ContractId) -> daovoting_mod::Proposal {
    let mem_address = daovoting_mod::MemoryAddress {
        contract_id: asset_id,
        function_selector: 0,
        function_data: 0,
    };

    let call_data = daovoting_mod::Proposal {
        memory_address: mem_address,
        num_coins_to_forward: 0,
        asset_id_of_coins_to_forward: asset_id,
        amount_of_gas_to_forward: 20000,
    };

    call_data
}

async fn initialize() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id)
        .call()
        .await
        .unwrap()
        .value
}

mod initialize {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn initializes() {
            initialize().await;
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_when_reinitialized() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;
        }
    }
}

mod add_proposal {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_add_proposal() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                daovoting_mod::ProposalInfo {
                    yes_votes: 0,
                    no_votes: 0,
                    acceptance_percentage: 10,
                    call_data: call_data,
                    end_height: 13,
                }
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_incorrect_proposal_id() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            user.dao_voting.proposal(0).call().await.unwrap();
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_with_incorrect_voting_period() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

            deployer
                .dao_voting
                .create_proposal(10, 101, call_data)
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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

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

            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);

            assert_eq!(
                user.dao_voting
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
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
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

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

            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

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

            deployer
                .dao_voting
                .constructor(gov_token_id)
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
        }
    }
}

mod vote {
    use super::*;

    mod success {
        use super::*;

        #[tokio::test]
        async fn user_can_vote() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

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

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(0, asset_amount / 4, true)
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(0, asset_amount / 4, false)
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                daovoting_mod::ProposalInfo {
                    yes_votes: asset_amount / 4,
                    no_votes: asset_amount / 4,
                    acceptance_percentage: 10,
                    call_data: call_data,
                    end_height: 15,
                }
            );
        }
    }

    mod revert {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_on_not_enough_votes() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data)
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(0, asset_amount / 4, true)
                .call()
                .await
                .unwrap()
                .value;
        }

        #[tokio::test]
        #[should_panic]
        async fn panics_on_expired_proposal() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

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

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

            user.dao_voting
                .create_proposal(1, 10, call_data)
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
                .vote(0, asset_amount / 4, true)
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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap();

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

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

            user.dao_voting
                .create_proposal(10, 10, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(0, asset_amount / 2, true)
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                daovoting_mod::ProposalInfo {
                    yes_votes: 5,
                    no_votes: 0,
                    call_data: call_data,
                    end_height: 15,
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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

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

            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);

            assert_eq!(
                user.dao_voting
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
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
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
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
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
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
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

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

            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap()
                .value;

            assert_eq!(deployer.dao_voting.balance().call().await.unwrap().value, 0);

            assert_eq!(
                user.dao_voting
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
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
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
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
        async fn user_can_convert_votes_to_tokens() {
            let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
            deployer
                .dao_voting
                .constructor(gov_token_id)
                .call()
                .await
                .unwrap();

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

            let call_data = get_call_data(user.wallet.address(), gov_token_id);

            user.dao_voting
                .create_proposal(1, 10, call_data.clone())
                .call()
                .await
                .unwrap()
                .value;

            user.dao_voting
                .vote(0, asset_amount / 2, true)
                .call()
                .await
                .unwrap()
                .value;

            let proposal = user.dao_voting.proposal(0).call().await.unwrap().value;

            assert_eq!(
                proposal,
                daovoting_mod::ProposalInfo {
                    yes_votes: 5,
                    no_votes: 0,
                    call_data: call_data,
                    end_height: 6,
                    acceptance_percentage: 10
                }
            );

            user.dao_voting
                .convert_votes_to_tokens(0)
                .call()
                .await
                .unwrap();

            assert_eq!(
                user.dao_voting
                    .user_balance(daovoting_mod::Identity::Address(user.wallet.address()))
                    .call()
                    .await
                    .unwrap()
                    .value,
                asset_amount
            );
        }
    }
}
