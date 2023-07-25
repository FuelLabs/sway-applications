mod success {

    use crate::utils::{
        interface::info::{nonce, threshold, threshold_hash},
        setup::{setup_env, VALID_SIGNER_PK},
    };
    use fuels::{
        accounts::fuel_crypto::Hasher,
        core::{codec::ABIEncoder, traits::Tokenizable},
        prelude::ContractId,
        tx::Bytes32,
        types::{Bits256, Token},
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};

    struct Threshold {
        contract_identifier: ContractId,
        data: Option<Bits256>,
        nonce: u64,
        threshold: u64,
    }

    #[tokio::test]
    async fn gets_threshold_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);
        let nonce = nonce(&deployer.contract).await.value;
        let threshold = threshold(&deployer.contract).await.value;

        // Recreate Threshold instance
        let tx = Threshold {
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            data: Some(data),
            nonce,
            threshold,
        };

        let tx_token = Token::Struct(vec![
            tx.contract_identifier.into_token(),
            tx.data.into_token(),
            tx.nonce.into_token(),
            tx.threshold.into_token(),
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&[tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = threshold_hash(&deployer.contract, Some(data), nonce, threshold)
            .await
            .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
