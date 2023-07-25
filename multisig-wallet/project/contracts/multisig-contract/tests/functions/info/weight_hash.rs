mod success {

    use crate::utils::{
        interface::info::{nonce, weight_hash},
        setup::{default_users, setup_env, User, VALID_SIGNER_PK},
    };
    use fuels::{
        accounts::fuel_crypto::Hasher,
        core::{codec::ABIEncoder, traits::Tokenizable},
        prelude::ContractId,
        tx::Bytes32,
        types::{Bits256, Token},
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};

    struct Weight {
        contract_identifier: ContractId,
        data: Option<Bits256>,
        nonce: u64,
        user: User,
    }

    #[tokio::test]
    async fn gets_weight_hash() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);
        let nonce = nonce(&deployer.contract).await.value;
        let user = default_users().pop().unwrap();

        // Recreate Weight instance
        let tx = Weight {
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            data: Some(data),
            nonce,
            user: user.clone(),
        };

        let tx_token = Token::Struct(vec![
            tx.contract_identifier.into_token(),
            tx.data.into_token(),
            tx.nonce.into_token(),
            tx.user.into_token(),
        ]);

        let encoded_tx_struct = ABIEncoder::encode(&[tx_token]).unwrap().resolve(0);
        let expected_hash = Hasher::hash(encoded_tx_struct);

        let response = weight_hash(&deployer.contract, Some(data), nonce, user)
            .await
            .value;

        assert_eq!(Bits256(expected_hash.into()), response);
    }
}
