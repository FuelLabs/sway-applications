use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Contract, StorageConfiguration,
        TxParameters, WalletUnlocked, WalletsConfig,
    },
    programs::call_response::FuelCallResponse,
    types::Identity,
};

abigen!(Contract(
    name = "Nft",
    abi = "./contracts/NFT-contract/out/debug/NFT-contract-abi.json"
));

pub(crate) struct Metadata {
    pub(crate) contract: Nft,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) mod paths {
    pub(crate) const NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/NFT-contract.bin";
    pub(crate) const NFT_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/NFT-contract-storage_slots.json";
}

pub(crate) mod abi_calls {

    use super::*;

    pub(crate) async fn admin(contract: &Nft) -> Option<Identity> {
        contract.methods().admin().call().await.unwrap().value
    }

    pub(crate) async fn approve(
        approved: Option<Identity>,
        contract: &Nft,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .approve(approved.clone(), token_id)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn approved(contract: &Nft, token_id: u64) -> Option<Identity> {
        contract
            .methods()
            .approved(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn balance_of(contract: &Nft, wallet: Identity) -> u64 {
        contract
            .methods()
            .balance_of(wallet.clone())
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn burn(contract: &Nft, token_id: u64) -> FuelCallResponse<()> {
        contract.methods().burn(token_id).call().await.unwrap()
    }

    pub(crate) async fn constructor(
        admin: Option<Identity>,
        contract: &Nft,
        token_supply: Option<u64>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .constructor(admin.clone(), token_supply)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn is_approved_for_all(
        contract: &Nft,
        operator: Identity,
        owner: Identity,
    ) -> bool {
        contract
            .methods()
            .is_approved_for_all(operator.clone(), owner.clone())
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn max_supply(contract: &Nft) -> Option<u64> {
        contract.methods().max_supply().call().await.unwrap().value
    }

    pub(crate) async fn mint(amount: u64, contract: &Nft, owner: Identity) -> FuelCallResponse<()> {
        contract
            .methods()
            .mint(amount, owner.clone())
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn owner_of(contract: &Nft, token_id: u64) -> Option<Identity> {
        contract
            .methods()
            .owner_of(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn set_approval_for_all(
        approve: bool,
        contract: &Nft,
        operator: Identity,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_approval_for_all(approve, operator.clone())
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn set_admin(
        contract: &Nft,
        minter: Option<Identity>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_admin(minter.clone())
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn token_metadata(contract: &Nft, token_id: u64) -> Option<TokenMetadata> {
        contract
            .methods()
            .token_metadata(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn tokens_minted(contract: &Nft) -> u64 {
        contract
            .methods()
            .tokens_minted()
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn transfer(
        contract: &Nft,
        to: Identity,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .transfer(to.clone(), token_id)
            .call()
            .await
            .unwrap()
    }
}

pub(crate) mod test_helpers {

    use super::*;
    use paths::{NFT_CONTRACT_BINARY_PATH, NFT_CONTRACT_STORAGE_PATH};

    pub(crate) async fn setup() -> (Metadata, Metadata, Metadata) {
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

        let nft_id = Contract::deploy(
            NFT_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let deploy_wallet = Metadata {
            contract: Nft::new(nft_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let owner1 = Metadata {
            contract: Nft::new(nft_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let owner2 = Metadata {
            contract: Nft::new(nft_id, wallet3.clone()),
            wallet: wallet3,
        };

        (deploy_wallet, owner1, owner2)
    }
}
