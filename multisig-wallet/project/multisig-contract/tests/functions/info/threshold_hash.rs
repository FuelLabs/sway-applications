mod success {

    use crate::utils::{
        interface::info::{nonce, threshold, threshold_hash},
        setup::{setup_env, VALID_SIGNER_PK},
    };
    use fuels::{
        contract::abi_encoder::ABIEncoder,
        prelude::{Bits256, ContractId, Token},
        signers::fuel_crypto::Hasher,
        tx::Bytes32,
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};

    struct Threshold {
        contract_identifier: ContractId,
        data: Option<Bits256>,
        nonce: u64,
        threshold: u64,
    }

    #[ignore]
    #[tokio::test]
    async fn gets_transaction_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);
        let nonce = nonce(&deployer.contract).await.value;
        let threshold = threshold(&deployer.contract).await.value;

        // Recreate Threshold instance
        let tx = Threshold {
            contract_identifier: deployer.contract.get_contract_id().try_into().unwrap(),
            data: Some(data),
            nonce,
            threshold,
        };

        // Set tokens for encoding the Threshold instance with ABIEncoder
        let contract_identifier_token = Token::Struct(vec![Token::B256(
            tx.contract_identifier.try_into().unwrap(),
        )]);
        let data_token = Token::B256(tx.data.unwrap().0);
        let nonce_token = Token::U64(tx.nonce);
        let threshold_token = Token::U64(tx.threshold);

        let tx_token = Token::Struct(vec![
            contract_identifier_token,
            data_token,
            nonce_token,
            threshold_token,
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&vec![tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = threshold_hash(&deployer.contract, Some(data), nonce, threshold)
            .await
            .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
