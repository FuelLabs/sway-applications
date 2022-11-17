use crate::utils::{
    abi_calls::balance,
    test_helpers::{base_asset_contract_id, setup_env},
    VALID_SIGNER_PK,
};

use fuels::prelude::*;

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_balance() {
        let (_private_key, contract, deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_balance = balance(&contract, base_asset_contract_id()).await.value;

        let transfer_amount = 200;
        let _receipt = deployer_wallet
            .force_transfer_to_contract(
                contract.get_contract_id(),
                transfer_amount,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let final_balance = balance(&contract, base_asset_contract_id()).await.value;

        assert_eq!(initial_balance, 0);
        assert_eq!(final_balance, transfer_amount);
    }
}
