use crate::utils::{
    abi_calls::{balance, mint_coins},
    test_helpers::{build_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_balance() {
        let (_owner, minter, mint_amount, token_contract_id, token_instance) =
            setup_and_initialize().await;

        mint_coins(&token_instance, mint_amount).await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;

        assert_eq!(balance(&token_instance_alternative).await, mint_amount);
    }
}
