use fuels::{contract::contract::CallResponse, prelude::*};

abigen!(Nft, "out/debug/NFT-abi.json");

pub struct Metadata {
    pub contract: Nft,
    pub wallet: WalletUnlocked,
}

pub mod abi_calls {

    use super::*;

    pub async fn admin(contract: &Nft) -> Identity {
        contract.methods().admin().call().await.unwrap().value
    }

    pub async fn approve(approved: &Identity, contract: &Nft, token_id: u64) -> CallResponse<()> {
        contract
            .methods()
            .approve(approved.clone(), token_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn approved(contract: &Nft, token_id: u64) -> Identity {
        contract
            .methods()
            .approved(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn balance_of(contract: &Nft, wallet: &Identity) -> u64 {
        contract
            .methods()
            .balance_of(wallet.clone())
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn burn(contract: &Nft, token_id: u64) -> CallResponse<()> {
        contract.methods().burn(token_id).call().await.unwrap()
    }

    pub async fn constructor(
        access_control: bool,
        contract: &Nft,
        owner: &Identity,
        token_supply: u64,
    ) -> CallResponse<()> {
        contract
            .methods()
            .constructor(access_control, owner.clone(), token_supply)
            .call()
            .await
            .unwrap()
    }

    pub async fn is_approved_for_all(
        contract: &Nft,
        operator: &Identity,
        owner: &Identity,
    ) -> bool {
        contract
            .methods()
            .is_approved_for_all(operator.clone(), owner.clone())
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn max_supply(contract: &Nft) -> u64 {
        contract.methods().max_supply().call().await.unwrap().value
    }

    pub async fn mint(amount: u64, contract: &Nft, owner: &Identity) -> CallResponse<()> {
        contract
            .methods()
            .mint(amount, owner.clone())
            .call()
            .await
            .unwrap()
    }

    pub async fn meta_data(contract: &Nft, token_id: u64) -> TokenMetaData {
        contract
            .methods()
            .meta_data(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn owner_of(contract: &Nft, token_id: u64) -> Identity {
        contract
            .methods()
            .owner_of(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn set_approval_for_all(
        approve: bool,
        contract: &Nft,
        operator: &Identity,
    ) -> CallResponse<()> {
        contract
            .methods()
            .set_approval_for_all(approve, operator.clone())
            .call()
            .await
            .unwrap()
    }

    pub async fn set_admin(contract: &Nft, minter: &Identity) -> CallResponse<()> {
        contract
            .methods()
            .set_admin(minter.clone())
            .call()
            .await
            .unwrap()
    }

    pub async fn total_supply(contract: &Nft) -> u64 {
        contract
            .methods()
            .total_supply()
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn transfer_from(
        contract: &Nft,
        from: &Identity,
        to: &Identity,
        token_id: u64,
    ) -> CallResponse<()> {
        contract
            .methods()
            .transfer_from(from.clone(), to.clone(), token_id)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {

    use super::*;

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
        )
        .await;

        // Get the wallets from that provider
        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();

        let nft_id = Contract::deploy(
            "./out/debug/NFT.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./out/debug/NFT-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        let deploy_wallet = Metadata {
            contract: Nft::new(nft_id.to_string(), wallet1.clone()),
            wallet: wallet1.clone(),
        };

        let owner1 = Metadata {
            contract: Nft::new(nft_id.to_string(), wallet2.clone()),
            wallet: wallet2.clone(),
        };

        let owner2 = Metadata {
            contract: Nft::new(nft_id.to_string(), wallet3.clone()),
            wallet: wallet3.clone(),
        };

        (deploy_wallet, owner1, owner2)
    }
}
