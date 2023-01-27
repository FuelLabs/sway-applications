mod success {

    use crate::utils::{
        interface::info::{nonce, weight_hash},
        setup::{default_users, setup_env, User, VALID_SIGNER_PK},
    };
    use fuels::{
        contract::abi_encoder::ABIEncoder,
        prelude::{Bits256, ContractId, Token},
        signers::fuel_crypto::Hasher,
        tx::Bytes32,
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};

    struct Weight {
        contract_identifier: ContractId,
        data: Option<Bits256>,
        nonce: u64,
        users: Vec<User>,
    }

    #[ignore]
    #[tokio::test]
    async fn gets_transaction_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);
        let nonce = nonce(&deployer.contract).await.value;
        let users = default_users();

        // Recreate Weight instance
        let tx = Weight {
            contract_identifier: deployer.contract.get_contract_id().try_into().unwrap(),
            data: Some(data),
            nonce,
            users: users.clone(),
        };

        // Set tokens for encoding the Weight instance with ABIEncoder
        let contract_identifier_token = Token::Struct(vec![Token::B256(
            tx.contract_identifier.try_into().unwrap(),
        )]);
        let data_token = Token::B256(tx.data.unwrap().0);
        let nonce_token = Token::U64(tx.nonce);
        let users_token = Token::Vector(vec![
            Token::Struct(vec![
                Token::B256(tx.users.get(0).unwrap().address.0),
                Token::U64(tx.users.get(0).unwrap().weight),
            ]),
            Token::Struct(vec![
                Token::B256(tx.users.get(1).unwrap().address.0),
                Token::U64(tx.users.get(1).unwrap().weight),
            ]),
        ]);

        let tx_token = Token::Struct(vec![
            contract_identifier_token,
            data_token,
            nonce_token,
            users_token,
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&vec![tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = weight_hash(&deployer.contract, Some(data), nonce, users)
            .await
            .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
