use crate::utils::{
    abi_calls::{get_mint_amount, initialize},
    test_helpers::{build_contract, setup},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_mint_amount() {
        let (owner, not_owner, .., token_contract_id, token_instance) = setup().await;
        let mint_amount = 10000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), not_owner.clone()).await;

        assert_eq!(
            get_mint_amount(&token_instance_alternative).await,
            mint_amount
        );
    }
}
