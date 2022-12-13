use fuels::{contract::call_response::FuelCallResponse, prelude::*, tx::ContractId};

use crate::utils::setup::{AssetInfo, Campaign, CampaignInfo, Fundraiser, Pledge};

pub async fn asset_count(contract: &Fundraiser) -> u64 {
    contract.methods().asset_count().call().await.unwrap().value
}

pub async fn asset_info_by_id(
    contract: &Fundraiser,
    asset: &ContractId,
) -> FuelCallResponse<Option<AssetInfo>> {
    contract
        .methods()
        .asset_info_by_id(*asset)
        .call()
        .await
        .unwrap()
}

pub async fn asset_info_by_count(
    contract: &Fundraiser,
    id: u64,
) -> FuelCallResponse<Option<AssetInfo>> {
    contract
        .methods()
        .asset_info_by_count(id)
        .call()
        .await
        .unwrap()
}

pub async fn campaign(
    contract: &Fundraiser,
    id: u64,
    user: Identity,
) -> FuelCallResponse<Option<Campaign>> {
    contract.methods().campaign(id, user).call().await.unwrap()
}

pub async fn campaign_info(
    contract: &Fundraiser,
    id: u64,
) -> FuelCallResponse<Option<CampaignInfo>> {
    contract.methods().campaign_info(id).call().await.unwrap()
}

pub async fn pledged(
    contract: &Fundraiser,
    id: u64,
    user: Identity,
) -> FuelCallResponse<Option<Pledge>> {
    contract.methods().pledged(id, user).call().await.unwrap()
}

pub async fn pledge_count(contract: &Fundraiser, user: Identity) -> u64 {
    contract
        .methods()
        .pledge_count(user)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn total_campaigns(contract: &Fundraiser) -> u64 {
    contract
        .methods()
        .total_campaigns()
        .call()
        .await
        .unwrap()
        .value
}

pub async fn user_campaign_count(contract: &Fundraiser, user: Identity) -> u64 {
    contract
        .methods()
        .user_campaign_count(user)
        .call()
        .await
        .unwrap()
        .value
}
