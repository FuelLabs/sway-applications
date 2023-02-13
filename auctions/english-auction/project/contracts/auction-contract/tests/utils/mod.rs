use fuels::{
    prelude::*,
    programs::call_response::FuelCallResponse,
    tx::{ContractId, Salt},
    types::Identity,
};

abigen!(
    Contract(
        name = "EnglishAuction",
        abi = "./contracts/auction-contract/out/debug/auction-contract-abi.json"
    ),
    Contract(
        name = "Nft",
        abi = "./contracts/auction-contract/tests/artifacts/NFT/out/debug/NFT-abi.json"
    ),
    Contract(
        name = "MyAsset",
        abi = "./contracts/auction-contract/tests/artifacts/asset/out/debug/asset-abi.json"
    ),
);

pub struct Metadata {
    pub asset: MyAsset,
    pub auction: EnglishAuction,
    pub nft: Nft,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const AUCTION_CONTRACT_BINARY_PATH: &str = "./out/debug/auction-contract.bin";
    pub const AUCTION_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/auction-contract-storage_slots.json";
    pub const NATIVE_ASSET_BINARY_PATH: &str = "./tests/artifacts/asset/out/debug/asset.bin";
    pub const NFT_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT.bin";
    pub const NFT_CONTRACT_STORAGE_PATH: &str =
        "./tests/artifacts/NFT/out/debug/NFT-storage_slots.json";
}

pub mod asset_abi_calls {

    use super::*;

    pub async fn mint_and_send_to_address(
        amount: u64,
        contract: &MyAsset,
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

pub mod english_auction_abi_calls {

    use super::*;

    pub async fn auction_info(auction_id: u64, contract: &EnglishAuction) -> Option<Auction> {
        contract
            .methods()
            .auction_info(auction_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn bid(
        auction_id: u64,
        bid_asset: AuctionAsset,
        contract: &EnglishAuction,
    ) -> FuelCallResponse<()> {
        match bid_asset {
            AuctionAsset::NFTAsset(bid_asset) => contract
                .methods()
                .bid(auction_id, AuctionAsset::NFTAsset(bid_asset.clone()))
                .set_contract_ids(&[bid_asset.asset_id.into()])
                .call()
                .await
                .unwrap(),
            AuctionAsset::TokenAsset(bid_asset) => {
                let tx_params = TxParameters::new(None, Some(1_000_000), None);
                let call_params = CallParameters::new(
                    Some(bid_asset.amount),
                    Some(AssetId::from(*bid_asset.asset_id)),
                    None,
                );

                contract
                    .methods()
                    .bid(auction_id, AuctionAsset::TokenAsset(bid_asset.clone()))
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .call()
                    .await
                    .unwrap()
            }
        }
    }

    pub async fn cancel(auction_id: u64, contract: &EnglishAuction) -> FuelCallResponse<()> {
        contract.methods().cancel(auction_id).call().await.unwrap()
    }

    pub async fn create(
        bid_asset: AuctionAsset,
        contract: &EnglishAuction,
        duration: u64,
        initial_price: u64,
        reserve_price: Option<u64>,
        seller: Identity,
        sell_asset: AuctionAsset,
    ) -> u64 {
        match sell_asset {
            AuctionAsset::NFTAsset(sell_asset) => {
                contract
                    .methods()
                    .create(
                        bid_asset,
                        duration,
                        initial_price,
                        reserve_price,
                        seller,
                        AuctionAsset::NFTAsset(sell_asset.clone()),
                    )
                    .set_contract_ids(&[sell_asset.asset_id.into(), sell_asset.asset_id.into()])
                    .call()
                    .await
                    .unwrap()
                    .value
            }
            AuctionAsset::TokenAsset(sell_asset) => {
                let tx_params = TxParameters::new(None, Some(1_000_000), None);
                let call_params = CallParameters::new(
                    Some(sell_asset.amount),
                    Some(AssetId::from(*sell_asset.asset_id)),
                    None,
                );

                contract
                    .methods()
                    .create(
                        bid_asset,
                        duration,
                        initial_price,
                        reserve_price,
                        seller,
                        AuctionAsset::TokenAsset(sell_asset.clone()),
                    )
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .call()
                    .await
                    .unwrap()
                    .value
            }
        }
    }

    pub async fn deposit_balance(
        auction_id: u64,
        contract: &EnglishAuction,
        identity: Identity,
    ) -> Option<AuctionAsset> {
        contract
            .methods()
            .deposit_balance(auction_id, identity)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn withdraw(
        auction_id: u64,
        contract: &EnglishAuction,
        withdrawing_asset: AuctionAsset,
    ) -> FuelCallResponse<()> {
        match withdrawing_asset {
            AuctionAsset::NFTAsset(withdrawing_asset) => contract
                .methods()
                .withdraw(auction_id)
                .set_contract_ids(&[withdrawing_asset.asset_id.into()])
                .call()
                .await
                .unwrap(),
            AuctionAsset::TokenAsset(_withdrawing_asset) => contract
                .methods()
                .withdraw(auction_id)
                .append_variable_outputs(1)
                .call()
                .await
                .unwrap(),
        }
    }

    pub async fn total_auctions(contract: &EnglishAuction) -> u64 {
        contract
            .methods()
            .total_auctions()
            .call()
            .await
            .unwrap()
            .value
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

    pub async fn set_approval_for_all(
        approve: bool,
        contract: &Nft,
        operator: Identity,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_approval_for_all(approve, operator)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {

    use super::*;
    use paths::{
        AUCTION_CONTRACT_BINARY_PATH, AUCTION_CONTRACT_STORAGE_PATH, NATIVE_ASSET_BINARY_PATH,
        NFT_CONTRACT_BINARY_PATH, NFT_CONTRACT_STORAGE_PATH,
    };

    pub async fn create_auction_copy(
        bid_asset: AuctionAsset,
        highest_bidder: Option<Identity>,
        end_block: u64,
        initial_price: u64,
        reserve_price: Option<u64>,
        sell_asset: AuctionAsset,
        seller: Identity,
        state: State,
    ) -> Auction {
        Auction {
            bid_asset,
            highest_bidder,
            end_block,
            initial_price,
            reserve_price,
            sell_asset,
            seller,
            state,
        }
    }

    pub async fn defaults_nft() -> (u64, u64, u64, u64) {
        let sell_count = 1;
        let inital_count = 1;
        let reserve_count = 1;
        let duration = 10;

        (sell_count, inital_count, reserve_count, duration)
    }

    pub async fn defaults_token() -> (u64, u64, u64, u64) {
        let sell_amount = 10;
        let initial_price = 1;
        let reserve_price = 10;
        let duration = 10;

        (sell_amount, initial_price, reserve_price, duration)
    }

    pub async fn nft_asset(asset_id: ContractId, token_id: u64) -> AuctionAsset {
        let token = NFTAsset { asset_id, token_id };

        AuctionAsset::NFTAsset(token)
    }

    pub async fn setup() -> (
        Metadata,
        Metadata,
        Metadata,
        Metadata,
        ContractId,
        ContractId,
        ContractId,
        ContractId,
        ContractId,
    ) {
        let num_wallets = 4;
        let coins_per_wallet = 1;
        let coin_amount = 1000000;
        let config = Config {
            manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
            ..Config::local_node()
        };
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(Some(num_wallets), Some(coins_per_wallet), Some(coin_amount)),
            Some(config),
            None,
        )
        .await;

        // Get the wallets from that provider
        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();
        let wallet4 = wallets.pop().unwrap();

        let auction_id = Contract::deploy(
            AUCTION_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                AUCTION_CONTRACT_STORAGE_PATH.to_string(),
            )),
        )
        .await
        .unwrap();

        let sell_asset_id = Contract::deploy(
            NATIVE_ASSET_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let sell_nft_id = Contract::deploy(
            NFT_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let deploy_wallet = Metadata {
            asset: MyAsset::new(sell_asset_id.clone(), wallet1.clone()),
            auction: EnglishAuction::new(auction_id.clone(), wallet1.clone()),
            nft: Nft::new(sell_nft_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let seller = Metadata {
            asset: MyAsset::new(sell_asset_id.clone(), wallet2.clone()),
            auction: EnglishAuction::new(auction_id.clone(), wallet2.clone()),
            nft: Nft::new(sell_nft_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let buy_asset_id = Contract::deploy_with_parameters(
            NATIVE_ASSET_BINARY_PATH,
            &wallet3,
            TxParameters::default(),
            StorageConfiguration::default(),
            Salt::from([1u8; 32]),
        )
        .await
        .unwrap();

        let buy_nft_id = Contract::deploy_with_parameters(
            NFT_CONTRACT_BINARY_PATH,
            &wallet3,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
            Salt::from([2u8; 32]),
        )
        .await
        .unwrap();

        let buyer1 = Metadata {
            asset: MyAsset::new(buy_asset_id.clone(), wallet3.clone()),
            auction: EnglishAuction::new(auction_id.clone(), wallet3.clone()),
            nft: Nft::new(buy_nft_id.clone(), wallet3.clone()),
            wallet: wallet3,
        };

        let buyer2 = Metadata {
            asset: MyAsset::new(buy_asset_id.clone(), wallet4.clone()),
            auction: EnglishAuction::new(auction_id.clone(), wallet4.clone()),
            nft: Nft::new(buy_nft_id.clone(), wallet4.clone()),
            wallet: wallet4,
        };

        (
            deploy_wallet,
            seller,
            buyer1,
            buyer2,
            auction_id.into(),
            sell_asset_id.into(),
            sell_nft_id.into(),
            buy_asset_id.into(),
            buy_nft_id.into(),
        )
    }

    pub async fn token_asset(asset_id: ContractId, amount: u64) -> AuctionAsset {
        let token = TokenAsset { asset_id, amount };

        AuctionAsset::TokenAsset(token)
    }
}
