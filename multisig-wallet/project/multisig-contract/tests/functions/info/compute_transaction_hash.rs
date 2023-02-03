mod success {

    use crate::utils::{
        interface::info::{compute_transaction_hash, nonce},
        setup::{setup_env, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK},
    };
    use fuels::{
        core::{abi_encoder::ABIEncoder, traits::Tokenizable},
        signers::fuel_crypto::Hasher,
        tx::Bytes32,
        types::{Bits256, ContractId, Identity, Token},
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};

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
    async fn computes_transaction_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let to = Identity::Address(deployer.wallet.address().try_into().unwrap());
        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);
        let nonce = nonce(&deployer.contract).await.value;
        ///

        // Recreate Transaction instance
        let tx = Transaction {
            contract_identifier: ContractId,
            nonce: u64,
            value: Option<u64>,
            asset_id: Option<ContractId>,
            target: Identity,
            function_selector: Option<Vec<u8>>,
            calldata: Option<Vec<u8>>,
            single_value_type_arg: Option<bool>,
            forwarded_gas: Option<u64>,
        };

        let tx_token = Token::Struct(vec![
            tx.contract_identifier.into_token(),
            tx.data.into_token(),
            tx.destination.into_token(),
            tx.nonce.into_token(),
            tx.value.into_token(),
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&vec![tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response =
            transaction_hash(&deployer.contract, to, DEFAULT_TRANSFER_AMOUNT, data, nonce)
                .await
                .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
