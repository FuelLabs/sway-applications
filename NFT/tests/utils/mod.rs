use fuels::{contract::contract::CallResponse, prelude::*};

// Load abi from json
abigen!(Nft, "out/debug/NFT-abi.json");

pub struct Metadata {
    pub nft: Nft,
    pub wallet: LocalWallet,
}

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (Metadata, Metadata, Metadata) {
        // Setup 3 test wallets
        let mut wallets = launch_provider_and_get_wallets(WalletsConfig {
            num_wallets: 3,
            coins_per_wallet: 1,
            coin_amount: 1000000,
        })
        .await;

        // Get the wallets from that provider
        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();

        let nft_id = Contract::deploy("./out/debug/NFT.bin", &wallet1, TxParameters::default())
            .await
            .unwrap();

        let deploy_wallet = Metadata {
            nft: Nft::new(nft_id.to_string(), wallet1.clone()),
            wallet: wallet1.clone(),
        };

        let owner1 = Metadata {
            nft: Nft::new(nft_id.to_string(), wallet2.clone()),
            wallet: wallet2.clone(),
        };

        let owner2 = Metadata {
            nft: Nft::new(nft_id.to_string(), wallet3.clone()),
            wallet: wallet3.clone(),
        };

        (deploy_wallet, owner1, owner2)
    }
}

pub mod abi_calls {

    use super::*;

    pub async fn approve(contract: &Nft, approved: &Option, token_id: u64) -> CallResponse<()> {
        contract
            .approve(approved.clone(), token_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn approved(contract: &Nft, token_id: u64) -> Option {
        contract.approved(token_id).call().await.unwrap().value
    }

    pub async fn balance_of(contract: &Nft, wallet: &Identity) -> u64 {
        contract
            .balance_of(wallet.clone())
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn burn(contract: &Nft, token_id: u64) -> CallResponse<()> {
        contract.burn(token_id).call().await.unwrap()
    }

    pub async fn constructor(
        contract: &Nft,
        owner: &Option,
        access_control: bool,
        token_supply: u64,
    ) -> CallResponse<()> {
        contract
            .constructor(owner.clone(), access_control, token_supply)
            .call()
            .await
            .unwrap()
    }

    pub async fn is_approved_for_all(
        contract: &Nft,
        owner: &Identity,
        operator: &Identity,
    ) -> bool {
        contract
            .is_approved_for_all(owner.clone(), operator.clone())
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn mint(contract: &Nft, owner: &Identity, amount: u64) -> CallResponse<()> {
        contract.mint(owner.clone(), amount).call().await.unwrap()
    }

    pub async fn owner_of(contract: &Nft, token_id: u64) -> Option {
        contract.owner_of(token_id).call().await.unwrap().value
    }

    pub async fn set_approval_for_all(
        contract: &Nft,
        owner: &Identity,
        operator: &Identity,
        approve: bool,
    ) -> CallResponse<()> {
        contract
            .set_approval_for_all(owner.clone(), operator.clone(), approve)
            .call()
            .await
            .unwrap()
    }

    pub async fn set_admin(contract: &Nft, minter: &Option) -> CallResponse<()> {
        contract.set_admin(minter.clone()).call().await.unwrap()
    }

    pub async fn total_supply(contract: &Nft) -> u64 {
        contract.total_supply().call().await.unwrap().value
    }

    pub async fn transfer_from(
        contract: &Nft,
        from: &Identity,
        to: &Identity,
        token_id: u64,
    ) -> CallResponse<()> {
        contract
            .transfer_from(from.clone(), to.clone(), token_id)
            .call()
            .await
            .unwrap()
    }
}
