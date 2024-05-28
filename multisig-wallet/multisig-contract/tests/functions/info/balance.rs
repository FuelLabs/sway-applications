mod success {

    use crate::utils::{
        interface::info::balance,
        setup::{setup_env, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK},
    };
    use fuels::{
        accounts::Account,
        prelude::{TxPolicies, BASE_ASSET_ID},
    };

    #[tokio::test]
    async fn gets_balance() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_balance = balance(&deployer.contract, BASE_ASSET_ID).await.value;

        deployer
            .wallet
            .force_transfer_to_contract(
                deployer.contract.contract_id(),
                DEFAULT_TRANSFER_AMOUNT,
                BASE_ASSET_ID,
                TxPolicies::default(),
            )
            .await
            .unwrap();

        let final_balance = balance(&deployer.contract, BASE_ASSET_ID).await.value;

        assert_eq!(initial_balance, 0);
        assert_eq!(final_balance, DEFAULT_TRANSFER_AMOUNT);
        assert!(final_balance > initial_balance);
    }
}
