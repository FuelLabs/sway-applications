use fuels::{
    contract::contract::CallResponse,
    prelude::*,
};

abigen!(
    TicTacToe,
    "./project/contracts/tictactoe-contract/out/debug/tictactoe-contract-abi.json"
);

pub struct Metadata {
    pub tictactoe: TicTacToe,
    pub wallet: WalletUnlocked,
}

pub mod test_helpers {
    use super::*;

    pub async fn setup() -> (Metadata, Metadata, Metadata){
        let num_wallets = 4;
        let coins_per_wallet = 1;
        let coin_amount = 1000000;
        let config = Config {
            manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
            ..Config::local_node()
        };
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(Some(num_wallets), Some(coins_per_wallet), Some(coin_amount)),
            Some(config),
            None,
        )
        .await;

        // Get the wallets from that provider
        let deployer_p1 = wallets.pop().unwrap();
        let p2 = wallets.pop().unwrap();
        let p3 = wallets.pop().unwrap();

        let tictactoe_contract_id = Contract::deploy(
            "./out/debug/tictactoe-contract.bin",
            &deployer_p1,
            TxParameters::default(),
            StorageConfiguration::default(),
        ).await.unwrap();

        let p1meta = Metadata {
            tictactoe: TicTacToe::new(tictactoe_contract_id.clone(), deployer_p1.clone()),
            wallet: deployer_p1,
        };

        let p2meta = Metadata {
            tictactoe: TicTacToe::new(tictactoe_contract_id.clone(), p2.clone()),
            wallet: p2,
        };

        let p3meta = Metadata {
            tictactoe: TicTacToe::new(tictactoe_contract_id.clone(), p3.clone()),
            wallet: p3,
        };

        (
            p1meta,
            p2meta,
            p3meta,
        )
    }
}

pub mod abi_calls {
    use super::*;

    pub async fn new_game(
        contract: &TicTacToe, 
        player_one: &Bech32Address, 
        player_two: &Bech32Address,
    ) -> CallResponse<()> {
        contract
            .methods()
            .new_game(
                Identity::Address(player_one.into()), 
                Identity::Address(player_two.into()),
            )
            .call()
            .await
            .unwrap()
    }

    pub async fn play_move(
        contract: &TicTacToe, 
        position: u64,
    ) -> CallResponse<()> {
        contract
            .methods()
            .play_move(position)
            .call()
            .await
            .unwrap()
    }
}