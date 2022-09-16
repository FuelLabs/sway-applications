use crate::utils::{
    abi_calls::{constructor, create_proposal, deposit, execute, vote},
    test_helpers::{mint, proposal_transaction, setup},
};
use fuels::{prelude::{abigen, CallParameters}, tx::AssetId};

abigen!(
    Governor,
    "../governor_contract/out/debug/governor_contract-abi.json"
);

mod success {
    use super::*;

    #[tokio::test]
    async fn user_proposal_can_execute() {
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

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&user.dao_voting, 10, 1, governor_id, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        execute(&user.dao_voting, governor_id, 0).await;

        let governor = GovernorBuilder::new(governor_id.to_string(), deployer.wallet.clone()).build();
        let (var1, var2) = governor.vars().call().await.unwrap().value;
        assert_eq!(var1, 42);
        assert_eq!(var2, true);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_invalid_proposal_id() {
        let (_gov_token, _gov_token_id, _deployer, user, _asset_amount, governor_id) = setup().await;
        execute(&user.dao_voting, governor_id, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    #[ignore]
    async fn on_already_executed_proposal() {
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

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&user.dao_voting, 10, 1, governor_id, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        execute(&user.dao_voting, governor_id, 0).await;
        execute(&user.dao_voting, governor_id, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    pub async fn on_active_proposal() {
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

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&user.dao_voting, 10, 100, governor_id, proposal_transaction.clone()).await;
        vote(&user.dao_voting, true, 0, asset_amount / 2).await;

        execute(&user.dao_voting, governor_id, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    pub async fn on_not_enough_yes_votes() {
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

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(&user.dao_voting, 10, 100, governor_id, proposal_transaction.clone()).await;
        vote(&user.dao_voting, false, 0, asset_amount / 2).await;

        execute(&user.dao_voting, governor_id, 0).await;
    }

    // TODO add test for reverting on a failed proposal call
}
