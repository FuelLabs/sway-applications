mod success {

    use crate::utils::{
        interface::info::{nonce, transaction_hash},
        setup::{setup_env, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK},
    };
    use fuels::{
        accounts::fuel_crypto::Hasher,
        core::{codec::ABIEncoder, traits::Tokenizable},
        prelude::ContractId,
        tx::Bytes32,
        types::{Bits256, Identity, Token},
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};

    struct Transaction {
        contract_identifier: ContractId,
        data: Bits256,
        destination: Identity,
        nonce: u64,
        value: u64,
    }

    #[tokio::test]
    async fn gets_transaction_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let to = Identity::Address(deployer.wallet.address().try_into().unwrap());
        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);
        let nonce = nonce(&deployer.contract).await.value;

        // Recreate Transaction instance
        let tx = Transaction {
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            data,
            destination: to.clone(),
            nonce,
            value: DEFAULT_TRANSFER_AMOUNT,
        };

        let tx_token = Token::Struct(vec![
            tx.contract_identifier.into_token(),
            tx.data.into_token(),
            tx.destination.into_token(),
            tx.nonce.into_token(),
            tx.value.into_token(),
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&[tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response =
            transaction_hash(&deployer.contract, to, DEFAULT_TRANSFER_AMOUNT, data, nonce)
                .await
                .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
