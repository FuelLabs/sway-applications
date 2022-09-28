use crate::utils::{
    abi_calls::mint_amount,
    test_helpers::{build_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_mint_amount() {
        let (_owner, minter, initial_mint_amount, token_contract_id, _token_instance) =
            setup_and_initialize().await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;

        assert_eq!(mint_amount(&token_instance_alternative).await, initial_mint_amount);
    }
}
