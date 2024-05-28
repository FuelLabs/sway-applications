mod success {
    use crate::utils::{
        interface::info::{compute_hash, nonce, threshold},
        setup::{
            default_users, setup_env, ContractCallParams, Threshold, Transaction,
            TransactionParameters, TransferParams, TypeToHash, Weight, DEFAULT_TRANSFER_AMOUNT,
            VALID_SIGNER_PK,
        },
    };
    use fuels::{
        accounts::fuel_crypto::Hasher,
        core::{codec::ABIEncoder, constants::BASE_ASSET_ID, traits::Tokenizable},
        prelude::Bytes,
        types::{Bits256, Identity, Token},
    };

    #[tokio::test]
    async fn gets_threshold_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let nonce = nonce(&deployer.contract).await.value;
        let threshold = threshold(&deployer.contract).await.value;

        let threshold_instance = Threshold {
            contract_identifier: deployer.contract.contract_id().into(),
            nonce,
            threshold,
        };

        let threshold_instance_token = Token::Struct(vec![
            threshold_instance.contract_identifier.into_token(),
            threshold_instance.nonce.into_token(),
            threshold_instance.threshold.into_token(),
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&[threshold_instance_token])
            .unwrap()
            .resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(threshold_instance),
        )
        .await
        .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }

    #[tokio::test]
    async fn gets_transfer_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let nonce = nonce(&deployer.contract).await.value;
        let target = Identity::Address(deployer.wallet.address().into());
        let transaction_parameters = TransactionParameters::Transfer(TransferParams {
            asset_id: BASE_ASSET_ID,
            value: Some(DEFAULT_TRANSFER_AMOUNT),
        });

        let transaction_instance = Transaction {
            contract_identifier: deployer.contract.contract_id().into(),
            nonce,
            target,
            transaction_parameters,
        };

        let transaction_instance_token = Token::Struct(vec![
            transaction_instance.contract_identifier.into_token(),
            transaction_instance.nonce.into_token(),
            transaction_instance.target.clone().into_token(),
            transaction_instance
                .transaction_parameters
                .clone()
                .into_token(),
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&[transaction_instance_token])
            .unwrap()
            .resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        dbg!(Bits256(expected_hash.into()));

        let response = compute_hash(
            &deployer.contract,
            TypeToHash::Transaction(transaction_instance),
        )
        .await
        .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }

    #[ignore = "Encoding of nested structs with Bytes fields in the RustSDK doesn't match the encoding in Sway. This is not an issue for the use of compute_hash when making a contract call; as the user would not be creating the hash in the SDK"]
    #[tokio::test]
    async fn gets_call_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let nonce = nonce(&deployer.contract).await.value;
        let target = Identity::Address(deployer.wallet.address().into());
        let transaction_parameters = TransactionParameters::Call(ContractCallParams {
            calldata: Bytes([1u8; 32].to_vec()),
            forwarded_gas: 100,
            function_selector: Bytes([1u8; 32].to_vec()),
            single_value_type_arg: false,
            transfer_params: TransferParams {
                asset_id: BASE_ASSET_ID,
                value: Some(DEFAULT_TRANSFER_AMOUNT),
            },
        });

        let transaction_instance = Transaction {
            contract_identifier: deployer.contract.contract_id().into(),
            nonce,
            target,
            transaction_parameters,
        };

        let transaction_instance_token = Token::Struct(vec![
            transaction_instance.contract_identifier.into_token(),
            transaction_instance.nonce.into_token(),
            transaction_instance.target.clone().into_token(),
            transaction_instance
                .transaction_parameters
                .clone()
                .into_token(), //This causes test to fail: tokenizing TransactionParameters::Call, does not encode the same as Sway
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&[transaction_instance_token])
            .unwrap()
            .resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = compute_hash(
            &deployer.contract,
            TypeToHash::Transaction(transaction_instance),
        )
        .await
        .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }

    #[tokio::test]
    async fn gets_weight_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let nonce = nonce(&deployer.contract).await.value;
        let user = default_users().pop().unwrap();

        let weight_instance = Weight {
            contract_identifier: deployer.contract.contract_id().into(),
            nonce,
            user: user.clone(),
        };

        let weight_instance_token = Token::Struct(vec![
            weight_instance.contract_identifier.into_token(),
            weight_instance.nonce.into_token(),
            weight_instance.user.clone().into_token(),
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&[weight_instance_token])
            .unwrap()
            .resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = compute_hash(&deployer.contract, TypeToHash::Weight(weight_instance))
            .await
            .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
