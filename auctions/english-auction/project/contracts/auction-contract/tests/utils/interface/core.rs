use crate::utils::setup::{AuctionAsset, EnglishAuction, Nft};
use fuels::{
    prelude::{AssetId, CallParameters, TxParameters, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::Identity,
};

pub(crate) mod auction {
    use super::*;

    pub(crate) async fn bid(
        auction_id: u64,
        bid_asset: AuctionAsset,
        contract: &EnglishAuction<WalletUnlocked>,
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
                let tx_params = TxParameters::new(0, 2_000_000, 0);
                let call_params = CallParameters::new(
                    bid_asset.amount,
                    AssetId::from(*bid_asset.asset_id),
                    1_000_000,
                );

                contract
                    .methods()
                    .bid(auction_id, AuctionAsset::TokenAsset(bid_asset.clone()))
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .unwrap()
                    .call()
                    .await
                    .unwrap()
            }
        }
    }

    pub(crate) async fn cancel(
        auction_id: u64,
        contract: &EnglishAuction<WalletUnlocked>,
    ) -> FuelCallResponse<()> {
        contract.methods().cancel(auction_id).call().await.unwrap()
    }

    pub(crate) async fn create(
        bid_asset: AuctionAsset,
        contract: &EnglishAuction<WalletUnlocked>,
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
                let tx_params = TxParameters::new(0, 2_000_000, 0);
                let call_params = CallParameters::new(
                    sell_asset.amount,
                    AssetId::from(*sell_asset.asset_id),
                    1_000_000,
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
                    .unwrap()
                    .call()
                    .await
                    .unwrap()
                    .value
            }
        }
    }

    pub(crate) async fn withdraw(
        auction_id: u64,
        contract: &EnglishAuction<WalletUnlocked>,
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
}

pub(crate) mod nft {
    use super::*;

    pub(crate) async fn approve(
        approved: Option<Identity>,
        contract: &Nft<WalletUnlocked>,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .approve(approved, token_id)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn mint(
        amount: u64,
        contract: &Nft<WalletUnlocked>,
        owner: Identity,
    ) -> FuelCallResponse<()> {
        contract.methods().mint(amount, owner).call().await.unwrap()
    }

    pub(crate) async fn owner_of(
        contract: &Nft<WalletUnlocked>,
        token_id: u64,
    ) -> Option<Identity> {
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
        contract: &Nft<WalletUnlocked>,
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
