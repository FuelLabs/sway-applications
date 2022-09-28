use crate::utils::{abi_calls::pool_info, test_helpers::setup};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_pool_info() {
        let (exchange_instance, _native_contract_id, ..) = setup().await;

        let pool_info = pool_info(&exchange_instance).await;
        assert_eq!(pool_info.eth_reserve, 0);
        assert_eq!(pool_info.token_reserve, 0);
        assert_eq!(pool_info.lp_token_supply, 0);
    }
}
