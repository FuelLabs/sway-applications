use fuels::prelude::*;

abigen!(Multisig, "out/debug/multisig-wallet-abi.json");

async fn setup() -> (Multisig, WalletUnlocked, WalletUnlocked, WalletUnlocked) {
    let num_wallets = 3;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/multisig-wallet.bin",
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/multisig-wallet-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    (
        Multisig::new(id.clone(), wallet1.clone()),
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
