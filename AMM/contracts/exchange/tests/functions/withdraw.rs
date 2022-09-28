use crate::utils::{
    abi_calls::{balance, deposit, withdraw},
    test_helpers::setup,
};
use fuels::{prelude::*, tx::ContractId};
use std::str::FromStr;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_withdraw_native_assets() {
        let (exchange_instance, native_contract_id, ..) = setup().await;

        let native_amount = 100;

        let call_params = CallParameters::new(Some(native_amount), None, None);
        deposit(&exchange_instance, call_params).await;
        withdraw(
            &exchange_instance,
            native_amount,
            native_contract_id.clone(),
        )
        .await;

        let balance = balance(&exchange_instance, native_contract_id).await;
        assert_eq!(balance, 0);
    }

    #[tokio::test]
    async fn can_withdraw_non_native_assets() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;

        let token_amount = 100;
        let token_contract_id = ContractId::new(*token_asset_id);

        let call_params = CallParameters::new(Some(token_amount), Some(token_asset_id), None);
        deposit(&exchange_instance, call_params).await;
        withdraw(&exchange_instance, token_amount, token_contract_id.clone()).await;

        let balance = balance(&exchange_instance, token_contract_id).await;
        assert_eq!(balance, 0);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_token_id_not_match() {
        let (exchange_instance, ..) = setup().await;

        let native_amount = 100;

        let call_params = CallParameters::new(Some(native_amount), None, None);
        deposit(&exchange_instance, call_params).await;

        let unmatched_id = ContractId::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000000002",
        )
        .unwrap();

        withdraw(&exchange_instance, 0, unmatched_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_user_no_balance_in_contract() {
        let (exchange_instance, native_contract_id, ..) = setup().await;

        let native_amount = 100;

        withdraw(&exchange_instance, native_amount, native_contract_id).await;
    }
}
