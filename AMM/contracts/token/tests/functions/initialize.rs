use crate::utils::{
    abi_calls::{initialize, mint_amount},
    test_helpers::build_contract,
    Identity, MyTokenBuilder,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn address_can_initialize_contract() {
        let owner = launch_provider_and_get_wallet().await;

        let token_contract_id = Contract::deploy(
            "../token/out/debug/token.bin",
            &owner,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let token_instance =
            MyTokenBuilder::new(token_contract_id.to_string(), owner.clone()).build();

        let initial_mint_amount = 10000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            initial_mint_amount,
        )
        .await;

        assert_eq!(mint_amount(&token_instance).await, initial_mint_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_reinitialization() {
        let initial_amount = 1000000000;
        let num_wallets = 2;
        let num_coins = 1;
        let config = WalletsConfig::new(Some(num_wallets), Some(num_coins), Some(initial_amount));
        let mut wallets = launch_custom_provider_and_get_wallets(config, None).await;
        let owner = wallets.pop().unwrap();
        let minter = wallets.pop().unwrap();

        let token_contract_id = Contract::deploy(
            "../token/out/debug/token.bin",
            &owner,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();
        let token_instance =
            MyTokenBuilder::new(token_contract_id.to_string(), owner.clone()).build();

        let mint_amount = 10000;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;

        initialize(
            &token_instance_alternative,
            Identity::Address(Address::from(minter.address())),
            mint_amount,
        )
        .await;
    }
}
