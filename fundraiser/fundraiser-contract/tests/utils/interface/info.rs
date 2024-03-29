use crate::utils::setup::{AssetInfo, Campaign, CampaignInfo, Fundraiser, Pledge};
use fuels::{
    prelude::{AssetId, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::Identity,
};

pub(crate) async fn asset_count(contract: &Fundraiser<WalletUnlocked>) -> u64 {
    contract.methods().asset_count().call().await.unwrap().value
}

pub(crate) async fn asset_info_by_id(
    contract: &Fundraiser<WalletUnlocked>,
    asset: &AssetId,
) -> FuelCallResponse<Option<AssetInfo>> {
    contract
        .methods()
        .asset_info_by_id(*asset)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn asset_info_by_count(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
) -> FuelCallResponse<Option<AssetInfo>> {
    contract
        .methods()
        .asset_info_by_count(id)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn campaign(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
    user: Identity,
) -> FuelCallResponse<Option<Campaign>> {
    contract.methods().campaign(id, user).call().await.unwrap()
}

pub(crate) async fn campaign_info(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
) -> FuelCallResponse<Option<CampaignInfo>> {
    contract.methods().campaign_info(id).call().await.unwrap()
}

pub(crate) async fn pledged(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
    user: Identity,
) -> FuelCallResponse<Option<Pledge>> {
    contract.methods().pledged(id, user).call().await.unwrap()
}

pub(crate) async fn pledge_count(contract: &Fundraiser<WalletUnlocked>, user: Identity) -> u64 {
    contract
        .methods()
        .pledge_count(user)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn total_campaigns(contract: &Fundraiser<WalletUnlocked>) -> u64 {
    contract
        .methods()
        .total_campaigns()
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn user_campaign_count(
    contract: &Fundraiser<WalletUnlocked>,
    user: Identity,
) -> u64 {
    contract
        .methods()
        .user_campaign_count(user)
        .call()
        .await
        .unwrap()
        .value
}
