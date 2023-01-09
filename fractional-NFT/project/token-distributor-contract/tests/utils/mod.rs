use fuels::{contract::call_response::FuelCallResponse, prelude::*};

// Load abi from json
abigen!(
    TokenDistributor,
    "./project/token-distributor-contract/out/debug/token-distributor-contract-abi.json"
);
abigen!(
    FractionalNFT,
    "./project/fractional-NFT-contract/out/debug/fractional-NFT-contract-abi.json"
);
abigen!(
    Nft,
    "./project/token-distributor-contract/tests/artifacts/NFT/out/debug/NFT-abi.json"
);
abigen!(
    Asset,
    "./project/token-distributor-contract/tests/artifacts/asset/out/debug/asset-abi.json"
);

pub struct Metadata {
    pub asset: Asset,
    pub f_nft: FractionalNFT,
    pub nft: Nft,
    pub token_distributor: TokenDistributor,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const ASSET_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/asset/out/debug/asset.bin";
    pub const FRACTIONAL_NFT_CONTRACT_BINARY_PATH: &str =
        "../fractional-NFT-contract/out/debug/fractional-NFT-contract.bin";
    pub const FRACTIONAL_NFT_CONTRACT_STORAGE_PATH: &str =
        "../fractional-NFT-contract/out/debug/fractional-NFT-contract-storage_slots.json";
    pub const NFT_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT.bin";
    pub const NFT_CONTRACT_STORAGE_PATH: &str =
        "./tests/artifacts/NFT/out/debug/NFT-storage_slots.json";
    pub const TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH: &str =
        "./out/debug/token-distributor-contract.bin";
    pub const TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/token-distributor-contract-storage_slots.json";
}

pub mod asset_abi_calls {

    use super::*;

    pub async fn mint_and_send_to_address(
        amount: u64,
        contract: &Asset,
        recipient: Address,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .mint_and_send_to_address(amount, recipient)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub mod fractional_nft_abi_calls {

    use super::*;

    pub async fn nft_info(contract: &FractionalNFT) -> Option<NFTInfo> {
        contract.methods().nft_info().call().await.unwrap().value
    }
}

pub mod token_distributor_abi_calls {

    use super::*;

    pub async fn buyback(
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        token_price: u64,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params =
            CallParameters::new(Some(amount), Some(AssetId::from(*external_asset)), None);

        contract
            .methods()
            .buyback(f_nft.clone(), token_price)
            .tx_params(tx_params)
            .call_params(call_params)
            .set_contracts(&[Bech32ContractId::from(f_nft)])
            .call()
            .await
            .unwrap()
    }

    pub async fn create(
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        nft: ContractId,
        reserve_price: Option<u64>,
        token_owner: Option<Identity>,
        token_price: u64,
        token_supply: u64,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .create(
                nft.clone(),
                external_asset,
                f_nft.clone(),
                reserve_price,
                token_owner,
                token_price,
                token_supply,
                token_id,
            )
            .set_contracts(&[Bech32ContractId::from(f_nft), Bech32ContractId::from(nft)])
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn end(
        contract: &TokenDistributor,
        f_nft: ContractId,
        nft: ContractId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .end(f_nft.clone())
            .set_contracts(&[Bech32ContractId::from(f_nft), Bech32ContractId::from(nft)])
            .call()
            .await
            .unwrap()
    }

    pub async fn purchase(
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        price: u64,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(amount * price),
            Some(AssetId::from(*external_asset)),
            None,
        );

        contract
            .methods()
            .purchase(amount, f_nft)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn purchase_admin(
        admin: Option<Identity>,
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        reserve: Option<u64>,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params =
            CallParameters::new(Some(amount), Some(AssetId::from(*external_asset)), None);

        contract
            .methods()
            .purchase_admin(admin, f_nft, reserve)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn sell(
        amount: u64,
        contract: &TokenDistributor,
        f_nft: ContractId,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*f_nft)), None);

        contract
            .methods()
            .sell(f_nft.clone())
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .set_contracts(&[Bech32ContractId::from(f_nft)])
            .call()
            .await
            .unwrap()
    }

    pub async fn set_reserve(
        contract: &TokenDistributor,
        f_nft: ContractId,
        reserve: Option<u64>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_reserve(f_nft, reserve)
            .call()
            .await
            .unwrap()
    }

    pub async fn set_token_price(
        contract: &TokenDistributor,
        f_nft: ContractId,
        token_price: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_token_price(f_nft, token_price)
            .call()
            .await
            .unwrap()
    }

    pub async fn token_distribution(
        contract: &TokenDistributor,
        f_nft: ContractId,
    ) -> Option<TokenDistribution> {
        contract
            .methods()
            .token_distribution(f_nft)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn withdraw(contract: &TokenDistributor, f_nft: ContractId) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(f_nft)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub mod nft_abi_calls {

    use super::*;

    pub async fn approve(
        approved: Option<Identity>,
        contract: &Nft,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .approve(approved, token_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn mint(amount: u64, contract: &Nft, owner: Identity) -> FuelCallResponse<()> {
        contract.methods().mint(amount, owner).call().await.unwrap()
    }

    pub async fn owner_of(contract: &Nft, token_id: u64) -> Option<Identity> {
        contract
            .methods()
            .owner_of(token_id)
            .call()
            .await
            .unwrap()
            .value
    }
}

pub mod test_helpers {

    use super::*;
    use paths::{
        ASSET_CONTRACT_BINARY_PATH, FRACTIONAL_NFT_CONTRACT_BINARY_PATH,
        FRACTIONAL_NFT_CONTRACT_STORAGE_PATH, NFT_CONTRACT_BINARY_PATH, NFT_CONTRACT_STORAGE_PATH,
        TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH, TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH,
    };

    pub async fn defaults() -> (u64, u64, u64, u64, u64) {
        let asset_supply = 100;
        let price = 1;
        let purchase_amount = 2;
        let supply = 10;
        let reserve = 10;
        (price, reserve, supply, purchase_amount, asset_supply)
    }

    pub async fn setup() -> (
        Metadata,
        Metadata,
        Metadata,
        ContractId,
        ContractId,
        ContractId,
        ContractId,
    ) {
        let number_of_wallets = 3;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(
                Some(number_of_wallets),
                Some(coins_per_wallet),
                Some(amount_per_coin),
            ),
            None,
            None,
        )
        .await;

        // Get the wallets from that provider
        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();

        let token_distributor_id = Contract::deploy(
            TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH.to_string(),
            )),
        )
        .await
        .unwrap();

        let f_nft_id = Contract::deploy(
            FRACTIONAL_NFT_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                FRACTIONAL_NFT_CONTRACT_STORAGE_PATH.to_string(),
            )),
        )
        .await
        .unwrap();

        let nft_id = Contract::deploy(
            NFT_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let asset_id = Contract::deploy(
            ASSET_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let deploy_wallet = Metadata {
            asset: Asset::new(asset_id.clone(), wallet1.clone()),
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet1.clone()),
            nft: Nft::new(nft_id.clone(), wallet1.clone()),
            token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let owner1 = Metadata {
            asset: Asset::new(asset_id.clone(), wallet2.clone()),
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet2.clone()),
            nft: Nft::new(nft_id.clone(), wallet2.clone()),
            token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let owner2 = Metadata {
            asset: Asset::new(asset_id.clone(), wallet3.clone()),
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet3.clone()),
            nft: Nft::new(nft_id.clone(), wallet3.clone()),
            token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet3.clone()),
            wallet: wallet3,
        };

        (
            deploy_wallet,
            owner1,
            owner2,
            token_distributor_id.into(),
            f_nft_id.into(),
            nft_id.into(),
            asset_id.into(),
        )
    }

    pub async fn wallet_balance(asset_contract: ContractId, wallet: &WalletUnlocked) -> u64 {
        wallet
            .get_asset_balance(&AssetId::new(*asset_contract))
            .await
            .unwrap()
    }
}
