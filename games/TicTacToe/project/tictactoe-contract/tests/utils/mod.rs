use fuels::{contract::contract::FuelCallResponse, prelude::*};

abigen!(TicTacToe, "out/debug/TicTacToe-abi.json");

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (TicTacToe, WalletUnlocked, WalletUnlocked) {
        let num_wallets = 2;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );

        let mut wallets = launch_provider_and_get_wallets(config).await;

        let player_one_wallet = wallets.pop().unwrap();
        let player_two_wallet = wallets.pop().unwrap();

        let id = Contract::deploy(
            "./out/debug/TicTacToe.bin",
            &player_one_wallet,
            TxParameters::default(),
        )
        .await
        .unwrap();

        let instance = TicTacToe::new(id.to_string(), player_one_wallet.clone());

        (instance, player_one_wallet, player_two_wallet)
    }
}

pub mod abi_calls {

    use super::*;

    pub async fn new_game(
        contract: &TicTacToe,
        player_one: Identity,
        player_two: Identity,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .new_game(player_one, player_two)
            .call()
            .await
            .unwrap()
    }

    pub async fn r#move(contract: &TicTacToe) -> FuelCallResponse<()> {
        contract.methods().r#move().call().await.unwrap()
    }
}
