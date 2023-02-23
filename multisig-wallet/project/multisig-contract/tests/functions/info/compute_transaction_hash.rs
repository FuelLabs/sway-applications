mod success {

    use crate::{calldata, fn_selector};

    use crate::utils::{
        interface::info::{compute_transaction_hash, nonce},
        setup::{base_asset_contract_id, setup_env, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK},
    };
    use fuels::{
        core::abi_encoder::ABIEncoder,
        prelude::ContractId,
        signers::fuel_crypto::Hasher,
        types::{traits::Tokenizable, Bits256, Identity, Token},
    };

    struct Transaction {
        contract_identifier: ContractId,
        nonce: u64,
        value: Option<u64>,
        asset_id: Option<ContractId>,
        target: Identity,
        function_selector: Option<Vec<u8>>,
        calldata: Option<Vec<u8>>,
        single_value_type_arg: Option<bool>,
        forwarded_gas: Option<u64>,
    }

    #[tokio::test]
    async fn computes_transfer_transaction_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        // Recreate Transaction instance
        let tx = Transaction {
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            nonce: nonce(&deployer.contract).await.value,
            value: Some(DEFAULT_TRANSFER_AMOUNT),
            asset_id: Some(base_asset_contract_id()),
            target: Identity::Address(deployer.wallet.address().try_into().unwrap()),
            function_selector: None,
            calldata: None,
            single_value_type_arg: None,
            forwarded_gas: None,
        };

        let tx_token = Token::Struct(vec![
            tx.contract_identifier.into_token(),
            tx.nonce.into_token(),
            tx.value.into_token(),
            tx.asset_id.into_token(),
            tx.target.clone().into_token(),
            tx.function_selector.clone().into_token(),
            tx.calldata.clone().into_token(),
            tx.single_value_type_arg.into_token(),
            tx.forwarded_gas.into_token(),
        ]);
        let encoded_tx_struct = ABIEncoder::encode(&vec![tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = compute_transaction_hash(
            &deployer.contract,
            tx.contract_identifier,
            tx.nonce,
            tx.value,
            tx.asset_id,
            tx.target,
            tx.function_selector,
            tx.calldata,
            tx.single_value_type_arg,
            tx.forwarded_gas,
        )
        .await
        .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }

    #[tokio::test]
    async fn computes_call_transaction_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        // Recreate Transaction instance
        let tx = Transaction {
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            nonce: nonce(&deployer.contract).await.value,
            value: Some(DEFAULT_TRANSFER_AMOUNT),
            asset_id: Some(base_asset_contract_id()),
            target: Identity::Address(deployer.wallet.address().try_into().unwrap()),
            function_selector: Some(fn_selector!(example_function(Identity, u64, bool))),
            calldata: Some(calldata!(
                Identity::Address(deployer.wallet.address().try_into().unwrap()),
                DEFAULT_TRANSFER_AMOUNT,
                true
            )),
            single_value_type_arg: Some(false),
            forwarded_gas: Some(10_000_000),
        };

        // Manually encode in order to accurately match encoding of the `Bytes` type in the contract's `Transaction` type
        let mut encoded_tx_struct = Vec::new();
        encoded_tx_struct.append(
            &mut ABIEncoder::encode(&vec![tx.contract_identifier.into_token()])
                .unwrap()
                .resolve(0),
        );
        encoded_tx_struct.append(
            &mut ABIEncoder::encode(&vec![tx.nonce.into_token()])
                .unwrap()
                .resolve(0),
        );
        encoded_tx_struct.append(
            &mut ABIEncoder::encode(&vec![tx.value.into_token()])
                .unwrap()
                .resolve(0),
        );
        encoded_tx_struct.append(
            &mut ABIEncoder::encode(&vec![tx.asset_id.into_token()])
                .unwrap()
                .resolve(0),
        );
        encoded_tx_struct.append(
            &mut ABIEncoder::encode(&vec![tx.target.clone().into_token()])
                .unwrap()
                .resolve(0),
        );

        let mut encoded_function_selector = match tx.function_selector.clone() {
            Option::Some(mut vec) => {
                let mut enum_tag = 1u64.to_be_bytes().to_vec();
                enum_tag.append(&mut vec);
                enum_tag
            }
            Option::None => [0u8; 32].to_vec(),
        };
        encoded_tx_struct.append(&mut encoded_function_selector);

        let mut encoded_calldata = match tx.calldata.clone() {
            Option::Some(mut vec) => {
                let mut enum_tag = 1u64.to_be_bytes().to_vec();
                enum_tag.append(&mut vec);
                enum_tag
            }
            Option::None => [0u8; 32].to_vec(),
        };
        encoded_tx_struct.append(&mut encoded_calldata);

        encoded_tx_struct.append(
            &mut ABIEncoder::encode(&vec![tx.single_value_type_arg.into_token()])
                .unwrap()
                .resolve(0),
        );
        encoded_tx_struct.append(
            &mut ABIEncoder::encode(&vec![tx.forwarded_gas.into_token()])
                .unwrap()
                .resolve(0),
        );

        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = compute_transaction_hash(
            &deployer.contract,
            tx.contract_identifier,
            tx.nonce,
            tx.value,
            tx.asset_id,
            tx.target,
            tx.function_selector,
            tx.calldata,
            tx.single_value_type_arg,
            tx.forwarded_gas,
        )
        .await
        .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
