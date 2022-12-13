use fuels::{contract::contract::CallResponse, prelude::*};

// Load abi from json
abigen!(
    FractionalNFT,
    "./project/fractional-NFT-contract/out/debug/fractional-NFT-contract-abi.json"
);

pub struct Metadata {
    pub contract: FractionalNFT,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const FRACTIONAL_NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/fractional-NFT-contract.bin";
    pub const FRACTIONAL_NFT_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/fractional-NFT-contract-storage_slots.json";
}

pub mod abi_calls {

    use super::*;

    pub async fn deposit(
        contract: &FractionalNFT,
        nft: ContractId,
        owner: Identity,
        supply: u64,
        token_id: u64,
    ) -> CallResponse<()> {
        contract
            .methods()
            .deposit(nft.clone(), owner.clone(), supply, token_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn nft(contract: &FractionalNFT) -> (Option<ContractId>, u64) {
        contract.methods().nft().call().await.unwrap().value
    }

    pub async fn owner(contract: &FractionalNFT) -> Option<Identity> {
        contract.methods().owner().call().await.unwrap().value
    }

    pub async fn set_owner(
        contract: &FractionalNFT,
        new_owner: Option<Identity>,
    ) -> CallResponse<()> {
        contract
            .methods()
            .set_owner(new_owner.clone())
            .call()
            .await
            .unwrap()
    }

    pub async fn supply(contract: &FractionalNFT) -> u64 {
        contract.methods().supply().call().await.unwrap().value
    }

    pub async fn withdraw(contract: &FractionalNFT) -> CallResponse<()> {
        contract.methods().withdraw().call().await.unwrap()
    }
}

pub mod test_helpers {

    use super::*;
    use paths::{FRACTIONAL_NFT_CONTRACT_BINARY_PATH, FRACTIONAL_NFT_CONTRACT_STORAGE_PATH};

    pub async fn setup() -> (Metadata, Metadata, Metadata) {
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

        let deploy_wallet = Metadata {
            contract: FractionalNFT::new(f_nft_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let owner1 = Metadata {
            contract: FractionalNFT::new(f_nft_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let owner2 = Metadata {
            contract: FractionalNFT::new(f_nft_id, wallet3.clone()),
            wallet: wallet3,
        };

        (deploy_wallet, owner1, owner2)
    }
}
