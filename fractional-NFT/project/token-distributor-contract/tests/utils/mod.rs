use fuels::{contract::contract::CallResponse, prelude::*};

// Load abi from json
abigen!(
    TokenDistributor,
    "./project/token-distributor-contract/out/debug/token-distibutor-contract-abi.json"
);
abigen!(
    FractionalNFT,
    "./project/fractional-NFT-contract/out/debug/fractional-NFT-contract-abi.json"
);
abigen!(
    Nft,
    "./project/token-distributor-contract/tests/artifacts/NFT/out/debug/NFT-abi.json"
);

pub struct Metadata {
    pub f_nft: FractionalNFT,
    pub nft: Nft,
    pub token_distributor: TokenDistributor,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const FRACTIONAL_NFT_CONTRACT_BINARY_PATH: &str = "../fractional-NFT-contract/out/debug/fractional-NFT-contract.bin";
    pub const FRACTIONAL_NFT_CONTRACT_STORAGE_PATH: &str =
        "../fractional-NFT-contract/out/debug/fractional-NFT-contract-storage_slots.json";
    pub const NFT_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT.bin";
    pub const NFT_CONTRACT_STORAGE_PATH: &str =
        "./tests/artifacts/NFT/out/debug/NFT-storage_slots.json";
    pub const TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH: &str = "./out/debug/token-distributor-contract.bin";
    pub const TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/token-distributor-contract-storage_slots.json";
}

pub mod token_distibutor_abi_calls {

    use super::*;

    pub async fn cancel(
        contract: &TokenDistributor,
        f_nft: ContractId,
        nft: ContractId,
    ) -> CallResponse<()> {
        contract
            .methods()
            .cancel(f_nft.clone())
            .set_contracts(&[Bech32ContractId::from(f_nft.clone()), Bech32ContractId::from(nft.clone())])
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn close(
        contract: &TokenDistributor,
        f_nft: ContractId,
        nft: ContractId,
    ) -> CallResponse<()> {
        contract
            .methods()
            .close(f_nft.clone())
            .set_contracts(&[Bech32ContractId::from(f_nft.clone()), Bech32ContractId::from(nft.clone())])
            .call()
            .await
            .unwrap()
    }

    pub async fn create(
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        nft: ContractId,
        owner: Option<Identity>,
        reserve_price: Option<u64>,
        token_price: u64,
        token_supply: u64,
        token_id: u64,
    ) -> CallResponse<()> {
        contract
            .methods()
            .create(external_asset.clone(), f_nft.clone(), nft.clone(), owner.clone(), reserve_price.clone(), token_price, token_supply, token_id)
            .set_contracts(&[Bech32ContractId::from(f_nft.clone()), Bech32ContractId::from(nft.clone())])
            .append_variable_outputs(1)
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
    ) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(amount * price),
            Some(AssetId::from(*external_asset)),
            None,
        );

        contract
            .methods()
            .purchase(amount, f_nft.clone())
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn purchase_reserve(
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        owner: Option<Identity>,
        reserve: Option<u64>
    ) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(amount),
            Some(AssetId::from(*external_asset)),
            None,
        );

        contract
            .methods()
            .purchase_reserve(f_nft.clone(), owner, reserve)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn request_return(
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        token_price: u64,
    ) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(amount),
            Some(AssetId::from(*external_asset)),
            None,
        );

        contract
            .methods()
            .request_return(f_nft.clone(), token_price)
            .tx_params(tx_params)
            .call_params(call_params)
            .set_contracts(&[Bech32ContractId::from(f_nft.clone())])
            .call()
            .await
            .unwrap()
    }

    pub async fn sell(
        amount: u64,
        contract: &TokenDistributor,
        f_nft: ContractId,
    ) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(amount),
            Some(AssetId::from(*f_nft)),
            None,
        );

        contract
            .methods()
            .sell(f_nft.clone())
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn token_distribution(contract: &TokenDistributor, f_nft: ContractId,) -> Option<TokenDistribution> {
        contract.methods().token_distribution(f_nft.clone()).call().await.unwrap().value
    }

    pub async fn withdraw(
        contract: &TokenDistributor,
        f_nft: ContractId,
    ) -> CallResponse<()> {
        contract
            .methods()
            .withdraw(f_nft.clone())
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
    ) -> CallResponse<()> {
        contract
            .methods()
            .approve(approved, token_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn mint(amount: u64, contract: &Nft, owner: Identity) -> CallResponse<()> {
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
    use paths::{FRACTIONAL_NFT_CONTRACT_BINARY_PATH, FRACTIONAL_NFT_CONTRACT_STORAGE_PATH, NFT_CONTRACT_BINARY_PATH, NFT_CONTRACT_STORAGE_PATH,TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH,TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH};

    pub async fn defaults() -> u64 {
        let supply = 10;
        supply
    }

    pub async fn setup() -> (Metadata, Metadata, Metadata, ContractId, ContractId, ContractId) {
        let num_wallets = 3;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(
                Some(num_wallets),
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

        let deploy_wallet = Metadata {
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet1.clone()),
            nft: Nft::new(nft_id.clone(), wallet1.clone()),
            token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let owner1 = Metadata {
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet2.clone()),
            nft: Nft::new(nft_id.clone(), wallet2.clone()),
            token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let owner2 = Metadata {
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet3.clone()),
            nft: Nft::new(nft_id.clone(), wallet3.clone()),
            token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet3.clone()),
            wallet: wallet3,
        };

        (deploy_wallet, owner1, owner2, token_distributor_id.into(), f_nft_id.into(), nft_id.into())
    }
}
