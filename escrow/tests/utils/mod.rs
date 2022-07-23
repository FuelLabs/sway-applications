use fuels::{
    contract::contract::CallResponse, 
    prelude::*,
};
// use fuels::tx::{AssetId, ContractId, Salt};

abigen!(Escrow, "out/debug/escrow-abi.json");
abigen!(MyAsset, "tests/artifacts/asset/out/debug/asset-abi.json");

pub struct User {
    pub contract: Escrow,
    pub wallet: LocalWallet,
}

pub struct Defaults {
    pub asset: MyAsset,
    pub asset_id: ContractId,
    pub asset_amount: u64,
    pub deadline: u64,
}

pub mod test_helpers {

    use super::*;

    pub async fn create_arbiter(address: Address, asset: ContractId, fee_amount: u64) -> Arbiter {
        Arbiter { address: Identity::Address(address), asset, fee_amount }
    }

    pub async fn create_asset(amount:u64, id: ContractId) -> Asset {
        Asset { amount, id }
    }

    pub async fn mint(contract: &MyAsset, address: Address, amount: u64) {
        contract
            .mint_and_send_to_address(amount, address)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }

    pub async fn setup() -> (User, User, User, Defaults) {
        let num_wallets = 4;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;
    
        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );
    
        let mut wallets = launch_provider_and_get_wallets(config).await;
    
        let deployer_wallet = wallets.pop().unwrap();
        let arbiter_wallet = wallets.pop().unwrap();
        let buyer_wallet = wallets.pop().unwrap();
        let seller_wallet = wallets.pop().unwrap();
    
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

        let asset = MyAsset::new(asset_id.to_string(), deployer_wallet.clone());

        let arbiter = User {
            contract: Escrow::new(escrow_id.to_string(), arbiter_wallet.clone()),
            wallet: arbiter_wallet,
        };
    
        let buyer = User {
            contract: Escrow::new(escrow_id.to_string(), buyer_wallet.clone()),
            wallet: buyer_wallet,
        };
    
        let seller = User {
            contract: Escrow::new(escrow_id.to_string(), seller_wallet.clone()),
            wallet: seller_wallet,
        };

        let defaults = Defaults { asset, asset_id, asset_amount: 100, deadline: 100 };
    
        (arbiter, buyer, seller, defaults)
    }

    pub async fn create_asset_with_salt(salt: [u8; 32], wallet: LocalWallet) -> (ContractId, MyAsset) {
        let asset_id = Contract::deploy_with_salt(
            "./tests/artifacts/asset/out/debug/asset.bin",
            &wallet,
            TxParameters::default(),
            Salt::from(salt),
        )
        .await
        .unwrap();

        (asset_id, MyAsset::new(asset_id.to_string(), wallet.clone()))
    }

    pub async fn asset_amount(wallet: &LocalWallet, asset: &ContractId) -> u64 {
        wallet
            .clone()
            .get_asset_balance(&AssetId::from(**asset))
            .await
            .unwrap()
    }

}

pub mod abi_calls {

    use super::*;

    pub async fn accept_arbiter(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.accept_arbiter(identifier).append_variable_outputs(1).call().await.unwrap()
    }

    pub async fn create_escrow(contract: &Escrow, amount: u64, arbiter: &Arbiter, asset: &ContractId, assets: Vec<Asset>, buyer: Identity, deadline: u64) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(**asset)), Some(100_000));

        contract.create_escrow(arbiter.clone(), assets, buyer, deadline).tx_params(tx_params).call_params(call_params).call().await.unwrap()
    }

    pub async fn deposit(amount: u64, asset: &ContractId, contract: &Escrow, identifier: u64) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(**asset)), Some(100_000));

        contract.deposit(identifier).tx_params(tx_params).call_params(call_params).call().await.unwrap()
    }

    pub async fn dispute(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.dispute(identifier).call().await.unwrap()
    }

    pub async fn propose_arbiter(contract: &Escrow, arbiter: Arbiter, identifier: u64) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(arbiter.fee_amount), Some(AssetId::from(*arbiter.asset)), Some(100_000));

        contract.propose_arbiter(arbiter, identifier).tx_params(tx_params).call_params(call_params).append_variable_outputs(1).call().await.unwrap()
    }

    pub async fn resolve_dispute(contract: &Escrow, identifier: u64, payment_amount: u64, user: Identity) -> CallResponse<()> {
        contract.resolve_dispute(identifier, payment_amount, user).append_variable_outputs(4).call().await.unwrap()
    }

    pub async fn return_deposit(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.return_deposit(identifier).append_variable_outputs(3).call().await.unwrap()
    }

    pub async fn take_payment(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.take_payment(identifier).append_variable_outputs(3).call().await.unwrap()
    }

    pub async fn transfer_to_seller(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.transfer_to_seller(identifier).append_variable_outputs(3).call().await.unwrap()
    }

}
