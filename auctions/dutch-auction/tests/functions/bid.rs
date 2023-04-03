pub mod passing {
    use fuels::types::{Identity, ContractId};
    use crate::utils::{create_auction, get_contract_instance, bid};

    #[tokio::test]
    async fn bid_is_correct() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2,  // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),            
        ).await;

        bid(&instance, 1, 100).await;

        assert_eq!(auctions_won(&instance, Identity::Address(wallet.address().into())).await, vec![1]);
    }
}