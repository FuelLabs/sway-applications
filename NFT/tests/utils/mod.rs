use fuels::{contract::contract::CallResponse, prelude::*};
use fuels_abigen_macro::abigen;

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

    pub async fn nft_identity_option(wallet: &Metadata) -> nft_mod::Option {
        let identity = Identity::Address(wallet.wallet.address());
        nft_mod::Option::Some(identity)
    }
}

pub mod abi_calls {

    use super::*;

    pub async fn constructor(
        deploy_wallet: &Metadata,
        owner: Option,
        access_control: bool,
        token_supply: u64,
    ) -> CallResponse<()> {
        deploy_wallet
            .nft
            .constructor(
                owner,
                access_control,
                token_supply,
            )
            .call()
            .await
            .unwrap()
    }

    pub async fn mint(mint_wallet: &Metadata, owner: &Metadata, amount: u64) -> CallResponse<()> {
        mint_wallet
            .nft
            .mint(nft_mod::Identity::Address(owner.wallet.address()), amount)
            .call()
            .await
            .unwrap()
    }

    pub async fn burn(call_wallet: &Metadata, token_id: u64) -> CallResponse<()> {
        call_wallet.nft.burn(token_id).call().await.unwrap()
    }

    pub async fn transfer_from(
        call_wallet: &Metadata,
        from: &Metadata,
        to: &Metadata,
        token_id: u64,
    ) -> CallResponse<()> {
        call_wallet
            .nft
            .transfer_from(
                nft_mod::Identity::Address(from.wallet.address()),
                nft_mod::Identity::Address(to.wallet.address()),
                token_id,
            )
            .call()
            .await
            .unwrap()
    }

    pub async fn approve(
        call_wallet: &Metadata,
        approved: &Metadata,
        token_id: u64,
        approve: bool,
    ) -> CallResponse<()> {
        match approve {
            true => call_wallet
                .nft
                .approve(
                    nft_mod::Option::Some(nft_mod::Identity::Address(approved.wallet.address())),
                    token_id,
                )
                .call()
                .await
                .unwrap(),
            false => call_wallet
                .nft
                .approve(nft_mod::Option::None(), token_id)
                .call()
                .await
                .unwrap(),
        }
    }

    pub async fn set_approval_for_all(
        call_wallet: &Metadata,
        owner: &Metadata,
        operator: &Metadata,
        approve: bool,
    ) -> CallResponse<()> {
        call_wallet
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner.wallet.address()),
                nft_mod::Identity::Address(operator.wallet.address()),
                approve,
            )
            .call()
            .await
            .unwrap()
    }

    pub async fn set_admin(call_wallet: &Metadata, minter: &Metadata) -> CallResponse<()> {
        call_wallet
            .nft
            .set_admin(nft_mod::Option::Some(nft_mod::Identity::Address(
                minter.wallet.address(),
            )))
            .call()
            .await
            .unwrap()
    }

    pub async fn approved(call_wallet: &Metadata, token_id: u64) -> nft_mod::Option {
        call_wallet
            .nft
            .approved(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn balance_of(call_wallet: &Metadata, wallet: &Metadata) -> u64 {
        call_wallet
            .nft
            .balance_of(nft_mod::Identity::Address(wallet.wallet.address()))
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn is_approved_for_all(
        call_wallet: &Metadata,
        owner: &Metadata,
        operator: &Metadata,
    ) -> bool {
        call_wallet
            .nft
            .is_approved_for_all(
                nft_mod::Identity::Address(owner.wallet.address()),
                nft_mod::Identity::Address(operator.wallet.address()),
            )
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn owner_of(call_wallet: &Metadata, token_id: u64) -> nft_mod::Option {
        call_wallet
            .nft
            .owner_of(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn total_supply(call_wallet: &Metadata) -> u64 {
        call_wallet.nft.total_supply().call().await.unwrap().value
    }
}
