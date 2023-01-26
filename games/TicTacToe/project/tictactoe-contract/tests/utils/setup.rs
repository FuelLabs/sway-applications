use fuels::prelude::*;

abigen!(
    TicTacToe,
    "./project/tictactoe-contract/out/debug/TicTacToe-abi.json"
);

const TICTACTOE_CONTRACT_BINARY_PATH: &str = "./out/debug/TicTacToe.bin";
const TICTACTOE_CONTRACT_STORAGE_PATH: &str = "./out/debug/TicTacToe-storage_slots.json";

pub struct Player {
    pub contract: TicTacToe,
    pub identity: Identity,
}

async fn identity(address: &Bech32Address) -> Identity {
    Identity::Address(address.into())
}

pub async fn setup() -> (Player, Player) {
    let num_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 100_000_000;

    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

    let player_one_wallet = wallets.pop().unwrap();
    let player_two_wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        TICTACTOE_CONTRACT_BINARY_PATH,
        &player_one_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(TICTACTOE_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let player_one = Player {
        contract: TicTacToe::new(id.clone(), player_one_wallet.clone()),
        identity: identity(player_one_wallet.address()).await,
    };

    let player_two = Player {
        contract: TicTacToe::new(id, player_two_wallet.clone()),
        identity: identity(player_two_wallet.address()).await,
    };

    (player_one, player_two)
}
