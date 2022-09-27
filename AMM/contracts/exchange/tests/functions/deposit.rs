use crate::utils::{
    abi_calls::{deposit, get_balance},
    test_helpers::setup,
};
use fuels::{prelude::*, tx::ContractId};
use std::str::FromStr;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_deposit_native_assets() {
        let (exchange_instance, native_contract_id, ..) = setup().await;

        let native_amount = 100;
        let call_params = CallParameters::new(Some(native_amount), None, None);
        deposit(&exchange_instance, call_params).await;

        let balance = get_balance(&exchange_instance, native_contract_id).await;
        assert_eq!(balance, native_amount);
    }

    #[tokio::test]
    async fn can_deposit_non_native_assets() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;

        let token_amount = 100;
        let call_params = CallParameters::new(Some(token_amount), Some(token_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let balance = get_balance(&exchange_instance, ContractId::new(*token_asset_id)).await;
        assert_eq!(balance, token_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_token_id_not_match() {
        let (exchange_instance, ..) = setup().await;

        let amount = 1;
        let unmatched_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000002")
                .unwrap();
        let call_params = CallParameters::new(Some(amount), Some(unmatched_id), None);
        deposit(&exchange_instance, call_params).await;
    }
}
