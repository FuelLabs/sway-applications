#![allow(unused_variables)]

use fuels::prelude::*;
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Multisig, "out/debug/multisig-wallet-abi.json");

async fn setup() -> (Multisig, LocalWallet, LocalWallet, LocalWallet) {
    // Create some addresses with the minimum amount of asset: 1 Million
    let (pk1, mut coins1) = setup_address_and_coins(1, 1000000);
    let (pk2, coins2) = setup_address_and_coins(1, 1000000);
    let (pk3, coins3) = setup_address_and_coins(1, 1000000);

    coins1.extend(coins2);
    coins1.extend(coins3);

    // Launch a provider with those coins
    let (provider, _) = setup_test_provider(coins1).await;

    // Get the wallets from that provider
    let wallet1 = LocalWallet::new_from_private_key(pk1, provider.clone());
    let wallet2 = LocalWallet::new_from_private_key(pk2, provider.clone());
    let wallet3 = LocalWallet::new_from_private_key(pk3, provider);

    let id = Contract::deploy(
        "./out/debug/multisig-wallet.bin",
        &wallet1,
        TxParameters::default(),
    )
    .await
    .unwrap();

    (
        Multisig::new(id.to_string(), wallet1.clone()),
        wallet1,
        wallet2,
        wallet3,
    )
}

mod constructor {

    use super::*;

    #[tokio::test]
    async fn placeholder() {
        let (multisig, wallet1, wallet2, wallet3) = setup().await;
    }
}

mod execute_transaction {

    use super::*;

    #[tokio::test]
    async fn placeholder() {
        let (multisig, wallet1, wallet2, wallet3) = setup().await;
    }
}

mod is_owner {

    use super::*;

    #[tokio::test]
    async fn placeholder() {
        let (multisig, wallet1, wallet2, wallet3) = setup().await;
    }
}

mod get_transaction_hash {

    use super::*;

    #[tokio::test]
    async fn placeholder() {
        let (multisig, wallet1, wallet2, wallet3) = setup().await;
    }
}
