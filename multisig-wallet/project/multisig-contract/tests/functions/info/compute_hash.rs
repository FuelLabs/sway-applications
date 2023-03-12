mod success {

    use crate::utils::{
        interface::info::{compute_hash, nonce, threshold},
        setup::{setup_env, Threshold, TypeToHash, VALID_SIGNER_PK},
    };
    use fuels::{
        core::abi_encoder::ABIEncoder,
        signers::fuel_crypto::Hasher,
        types::{traits::Tokenizable, Bits256, Token},
    };

    #[tokio::test]
    async fn gets_threshold_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let nonce = nonce(&deployer.contract).await.value;
        let threshold = threshold(&deployer.contract).await.value;

        // Recreate Threshold instance
        let threshold = Threshold {
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            nonce,
            threshold,
        };

        let threshold_token = Token::Struct(vec![
            threshold.contract_identifier.into_token(),
            threshold.nonce.into_token(),
            threshold.threshold.into_token(),
        ]);

        let encoded_threshold_struct = ABIEncoder::encode(&vec![threshold_token])
            .unwrap()
            .resolve(0);
        let expected_hash = Hasher::hash(encoded_threshold_struct);

        let response = compute_hash(&deployer.contract, TypeToHash::Threshold(threshold))
            .await
            .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
