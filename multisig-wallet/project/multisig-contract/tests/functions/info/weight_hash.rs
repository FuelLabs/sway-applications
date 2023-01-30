// TODO: test is disabled because hashing a vector is problematic in the SDK (because of offsets)
//       and thus the hash produced is not the same
//       https://github.com/FuelLabs/fuels-rs/issues/775#issuecomment-1408751296

mod success {

    use crate::utils::{
        interface::info::{nonce, weight_hash},
        setup::{default_users, setup_env, User, VALID_SIGNER_PK},
    };
    use fuels::{
        contract::abi_encoder::ABIEncoder,
        core::Tokenizable,
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

        let tx_token = Token::Struct(vec![
            tx.contract_identifier.into_token(),
            tx.data.into_token(),
            tx.nonce.into_token(),
            tx.users.into_token(),
        ]);

        // TODO: the problem here is that the `resolve(0)` assumes a memory offset which we cannot
        //       know until runtime because we are using a Vec
        let encoded_tx_struct = ABIEncoder::encode(&vec![tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        // Set tokens for encoding the Weight instance with ABIEncoder
        // let contract_identifier_token = Token::Struct(vec![Token::B256(
        //     tx.contract_identifier.try_into().unwrap(),
        // )]);

        // The data would need to be wrapped in an optional enum but we don't need to do that
        // manually

        // let data_token = Token::B256(tx.data.unwrap().0);
        // let nonce_token = Token::U64(tx.nonce);
        // let users_token = Token::Vector(vec![
        //     Token::Struct(vec![
        //         Token::B256(tx.users.get(0).unwrap().address.0),
        //         Token::U64(tx.users.get(0).unwrap().weight),
        //     ]),
        //     Token::Struct(vec![
        //         Token::B256(tx.users.get(1).unwrap().address.0),
        //         Token::U64(tx.users.get(1).unwrap().weight),
        //     ]),
        // ]);

        // let tx_token = Token::Struct(vec![
        //     contract_identifier_token,
        //     data_token,
        //     nonce_token,
        //     users_token,
        // ]);

        let response = weight_hash(&deployer.contract, Some(data), nonce, users)
            .await
            .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
