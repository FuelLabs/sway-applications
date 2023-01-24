use fuels::{prelude::*, programs::call_response::FuelCallResponse, types::Identity};

// Load abi from json
abigen!(
    Contract(
        name = "FractionalNFT",
        abi = "./project/fractional-NFT-contract/out/debug/fractional-NFT-contract-abi.json"
    ),
    Contract(
        name = "Nft",
        abi = "./project/fractional-NFT-contract/tests/artifacts/NFT/out/debug/NFT-1-abi.json"
    )
);

pub struct Metadata {
    pub f_nft: FractionalNFT,
    pub nft: Nft,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const FRACTIONAL_NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/fractional-NFT-contract.bin";
    pub const FRACTIONAL_NFT_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/fractional-NFT-contract-storage_slots.json";
    pub const NFT_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT-1.bin";
    pub const NFT_CONTRACT_STORAGE_PATH: &str =
        "./tests/artifacts/NFT/out/debug/NFT-1-storage_slots.json";
}

pub mod fractional_nft_abi_calls {

    use super::*;

    pub async fn deposit(
        admin: Option<Identity>,
        contract: &FractionalNFT,
        nft: ContractId,
        supply: u64,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .deposit(admin, nft, supply, token_id)
            .set_contract_ids(&[Bech32ContractId::from(nft)])
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn nft_info(contract: &FractionalNFT) -> Option<NFTInfo> {
        contract.methods().nft_info().call().await.unwrap().value
    }

    pub async fn set_admin(
        contract: &FractionalNFT,
        new_admin: Option<Identity>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_admin(new_admin)
            .call()
            .await
            .unwrap()
    }

    pub async fn supply(contract: &FractionalNFT) -> u64 {
        contract.methods().supply().call().await.unwrap().value
    }

    pub async fn withdraw(
        contract: &FractionalNFT,
        nft: ContractId,
        to: Identity,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(to)
            .set_contract_ids(&[Bech32ContractId::from(nft)])
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
        FRACTIONAL_NFT_CONTRACT_BINARY_PATH, FRACTIONAL_NFT_CONTRACT_STORAGE_PATH,
        NFT_CONTRACT_BINARY_PATH, NFT_CONTRACT_STORAGE_PATH,
    };

    pub async fn defaults() -> u64 {
        10 // supply
    }

    pub async fn setup() -> (Metadata, Metadata, Metadata, ContractId, ContractId) {
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
            wallet: wallet1,
        };

        let owner1 = Metadata {
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet2.clone()),
            nft: Nft::new(nft_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let owner2 = Metadata {
            f_nft: FractionalNFT::new(f_nft_id.clone(), wallet3.clone()),
            nft: Nft::new(nft_id.clone(), wallet3.clone()),
            wallet: wallet3,
        };

        (
            deploy_wallet,
            owner1,
            owner2,
            f_nft_id.into(),
            nft_id.into(),
        )
    }
}
