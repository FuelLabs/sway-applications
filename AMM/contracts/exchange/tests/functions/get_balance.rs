use crate::utils::{
    abi_calls::{deposit, get_balance},
    test_helpers::setup,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_balance() {
        let (exchange_instance, native_contract_id, ..) = setup().await;

        let native_amount = 100;

        let call_params = CallParameters::new(Some(native_amount), None, None);
        deposit(&exchange_instance, call_params).await;

        let balance = get_balance(&exchange_instance, native_contract_id).await;
        assert_eq!(balance, 100);
    }
}
