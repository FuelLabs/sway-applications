mod success {
    use crate::utils::{
        interface::info::{asset, total_assets},
        setup::setup,
    };
    use fuels::prelude::{TxParameters, BASE_ASSET_ID};

    #[tokio::test]
    async fn gets_amount() {
        let (instance, wallet) = setup().await;
        let transfer_amount = 101;
        let initial_amount = total_assets(&instance).await.value;

        let (_tx_id, _receipts) = wallet
            .force_transfer_to_contract(
                &instance.get_contract_id(),
                transfer_amount,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let final_amount = total_assets(&instance).await.value;

        assert_eq!(0, initial_amount);
        assert_eq!(transfer_amount, final_amount);
        assert_ne!(initial_amount, final_amount);
    }
}
