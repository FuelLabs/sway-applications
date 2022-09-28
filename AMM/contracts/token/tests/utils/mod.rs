use fuels::{contract::contract::CallResponse, prelude::*, tx::ContractId};

abigen!(MyToken, "../token/out/debug/token-abi.json");

pub mod abi_calls {
    use super::*;

    pub async fn balance(contract: &MyToken) -> u64 {
        contract.balance().call().await.unwrap().value
    }

    pub async fn burn_coins(contract: &MyToken, amount: u64) -> CallResponse<()> {
        contract.burn_coins(amount).call().await.unwrap()
    }

    pub async fn initialize(
        contract: &MyToken,
        identity: Identity,
        amount: u64,
    ) -> CallResponse<()> {
        contract.initialize(identity, amount).call().await.unwrap()
    }

    pub async fn mint(contract: &MyToken) -> CallResponse<()> {
        contract
            .mint()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn mint_amount(contract: &MyToken) -> u64 {
        contract.mint_amount().call().await.unwrap().value
    }

    pub async fn mint_coins(contract: &MyToken, amount: u64) -> CallResponse<()> {
        contract.mint_coins(amount).call().await.unwrap()
    }

    pub async fn set_mint_amount(contract: &MyToken, amount: u64) -> CallResponse<()> {
        contract.set_mint_amount(amount).call().await.unwrap()
    }

    pub async fn token_balance(
        contract: &MyToken,
        call_params: CallParameters,
        asset: ContractId,
    ) -> u64 {
        contract
            .token_balance(asset)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn transfer_coins(
        contract: &MyToken,
        coins: u64,
        identity: Identity,
    ) -> CallResponse<()> {
        contract
            .transfer_coins(coins, identity)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer_token_to_output(
        contract: &MyToken,
        asset_id: ContractId,
        coins: u64,
        identity: Identity,
    ) -> CallResponse<()> {
        contract
            .transfer_token_to_output(asset_id, coins, identity)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {
    use super::*;
    use abi_calls::initialize;

    pub async fn build_contract(contract_id: Bech32ContractId, wallet: WalletUnlocked) -> MyToken {
        MyTokenBuilder::new(contract_id.to_string(), wallet).build()
    }

    pub async fn setup_and_initialize() -> (
        WalletUnlocked,
        WalletUnlocked,
        u64,
        Bech32ContractId,
        MyToken,
    ) {
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

        (
            owner.clone(),
            minter.clone(),
            mint_amount,
            token_contract_id,
            token_instance,
        )
    }
}
