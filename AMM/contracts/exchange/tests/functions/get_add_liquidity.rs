use crate::utils::{
    abi_calls::get_add_liquidity,
    test_helpers::setup,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_preview_add_liquidity() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;

        let forwarded: u64 = 100;
        let expected_amount: u64 = 99;

        let call_params = CallParameters::new(
            Some(forwarded),
            Some(token_asset_id.clone()),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let add_liquidity_preview = get_add_liquidity(
            &exchange_instance,
            call_params,
            tx_params,
            expected_amount,
            Bits256(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(add_liquidity_preview.lp_token_received, expected_amount);
    }
}
