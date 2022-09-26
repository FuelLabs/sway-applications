use crate::utils::{
    abi_calls::{get_mint_amount, initialize, set_mint_amount},
    test_helpers::{build_contract, setup},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_set_mint_amount() {
        let (owner, .., token_instance) = setup().await;
        let initial_mint_amount = 10000;
        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            initial_mint_amount,
        )
        .await;
        let new_mint_amount = 1;
        set_mint_amount(&token_instance, new_mint_amount).await;
        let mint_amount = get_mint_amount(&token_instance).await;
        assert_eq!(mint_amount, new_mint_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_set_mint_amount() {
        let (owner, minter, .., token_contract_id, token_instance) = setup().await;
        let initial_mint_amount = 10000;
        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            initial_mint_amount,
        )
        .await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;
        let new_mint_amount = 1;
        set_mint_amount(&token_instance_alternative, new_mint_amount).await;
    }
}
