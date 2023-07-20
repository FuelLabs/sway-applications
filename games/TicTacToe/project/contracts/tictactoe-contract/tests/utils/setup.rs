use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Contract, LoadConfiguration,
        StorageConfiguration, TxParameters, WalletsConfig,
    },
    types::Identity,
};

abigen!(Contract(
    name = "TicTacToe",
    abi = "./contracts/tictactoe-contract/out/debug/tictactoe-contract-abi.json"
));

const TICTACTOE_CONTRACT_BINARY_PATH: &str = "./out/debug/tictactoe-contract.bin";
const TICTACTOE_CONTRACT_STORAGE_PATH: &str = "./out/debug/tictactoe-contract-storage_slots.json";

pub(crate) struct Player {
    pub(crate) contract: TicTacToe<WalletUnlocked>,
    pub(crate) identity: Identity,
}

pub(crate) async fn setup() -> (Player, Player) {
    let number_of_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 100_000_000;

    let config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

    let player_one_wallet = wallets.pop().unwrap();
    let player_two_wallet = wallets.pop().unwrap();
    let contract_storage_configuration =
        StorageConfiguration::load_from(TICTACTOE_CONTRACT_STORAGE_PATH);

    let contract_configuration = LoadConfiguration::default()
        .set_storage_configuration(contract_storage_configuration.unwrap());

    let contract_id = Contract::load_from(TICTACTOE_CONTRACT_BINARY_PATH, contract_configuration)
        .unwrap()
        .deploy(&player_one_wallet, TxParameters::default())
        .await
        .unwrap();

    let player_one = Player {
        contract: TicTacToe::new(contract_id.clone(), player_one_wallet.clone()),
        identity: Identity::Address(player_one_wallet.address().into()),
    };

    let player_two = Player {
        contract: TicTacToe::new(contract_id, player_two_wallet.clone()),
        identity: Identity::Address(player_two_wallet.address().into()),
    };

    (player_one, player_two)
}
