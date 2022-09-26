use crate::utils::{
    abi_calls::{burn_coins, get_balance, initialize, mint_coins},
    test_helpers::{build_contract, setup},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn owner_can_burn_all_coins() {
        let (owner, .., token_instance) = setup().await;
        let mint_amount = 10000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        mint_coins(&token_instance, mint_amount).await;
        burn_coins(&token_instance, mint_amount).await;

        assert_eq!(get_balance(&token_instance).await, 0);
    }

    #[tokio::test]
    async fn owner_can_burn_coins_partially() {
        let (owner, .., token_instance) = setup().await;
        let mint_amount = 10000;
        let burn_amount = 5000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        mint_coins(&token_instance, mint_amount).await;
        burn_coins(&token_instance, burn_amount).await;

        assert_eq!(
            get_balance(&token_instance).await,
            mint_amount - burn_amount
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_burn() {
        let (owner, .., not_owner, token_contract_id, token_instance) = setup().await;
        let mint_amount = 10000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        mint_coins(&token_instance, mint_amount).await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), not_owner.clone()).await;

        burn_coins(&token_instance_alternative, mint_amount).await;
    }
}
