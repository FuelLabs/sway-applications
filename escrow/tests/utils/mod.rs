use fuels::prelude::*;
use fuels::tx::{AssetId, ContractId, Salt};

abigen!(Escrow, "out/debug/escrow-abi.json");
abigen!(MyAsset, "tests/artifacts/asset/out/debug/asset-abi.json");

struct MetaAsset {
    amount: u64,
    id: [u8; 32],
}

struct Metadata {
    escrow: Escrow,
    asset: Option<MyAsset>,
    wallet: LocalWallet,
}

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (Metadata, Metadata, Metadata, ContractId, u64) {
        let num_wallets = 3;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;
    
        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );
    
        let mut wallets = launch_provider_and_get_wallets(config).await;
    
        let deployer_wallet = wallets.pop().unwrap();
        let user1_wallet = wallets.pop().unwrap();
        let user2_wallet = wallets.pop().unwrap();
    
        let escrow_id = Contract::deploy(
            "./out/debug/escrow.bin",
            &deployer_wallet,
            TxParameters::default(),
        )
        .await
        .unwrap();
    
        let asset_id = Contract::deploy(
            "./tests/artifacts/asset/out/debug/asset.bin",
            &deployer_wallet,
            TxParameters::default(),
        )
        .await
        .unwrap();
    
        let deployer = Metadata {
            escrow: Escrow::new(escrow_id.to_string(), deployer_wallet.clone()),
            asset: Some(MyAsset::new(asset_id.to_string(), deployer_wallet.clone())),
            wallet: deployer_wallet,
        };
    
        let user1 = Metadata {
            escrow: Escrow::new(escrow_id.to_string(), user1_wallet.clone()),
            asset: None,
            wallet: user1_wallet,
        };
    
        let user2 = Metadata {
            escrow: Escrow::new(escrow_id.to_string(), user2_wallet.clone()),
            asset: None,
            wallet: user2_wallet,
        };
    
        let asset_amount: u64 = 100;
    
        (deployer, user1, user2, asset_id, asset_amount)
    }

}

pub mod abi_calls {

    use super::*;

    pub async fn accept_arbiter(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.accept_arbiter(identifier).append_variable_outputs(1).call().await.unwrap()
    }

    pub async fn create_escrow(contract: &Escrow, arbiter: Arbiter, assets: Vec<Asset>, buyer: Identity, deadline: u64) -> CallResponse<()> {
        contract.create_escrow(arbiter, assets, buyer, deadline).call().await.unwrap()
    }

    pub async fn deposit(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        // TxParams, CallParams refactor into here?
        contract.deposit(identifier).call().await.unwrap()
    }

    pub async fn dispute(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.dispute(identifier).call().await.unwrap()
    }

    pub async fn propose_arbiter(contract: &Escrow, arbiter: Arbiter, identifier: u64) -> CallResponse<()> {
        contract.propose_arbiter(arbiter, identifier).append_variable_outputs(1).call().await.unwrap()
    }

    pub async fn resolve_dispute(contract: &Escrow, identifier: u64, payment_amount: u64, user: Identity) -> CallResponse<()> {
        contract.resolve_dispute(identifier, payment_amount, user).append_variable_outputs(3).call().await.unwrap()
    }

    pub async fn return_deposit(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.return_deposit(identifier).append_variable_outputs(2).call().await.unwrap()
    }

    pub async fn take_payment(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.take_payment(identifier).call().append_variable_outputs(1).await.unwrap()
    }

    pub async fn transfer_to_seller(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.transfer_to_seller(identifier).call().append_variable_outputs(1).await.unwrap()
    }

}
