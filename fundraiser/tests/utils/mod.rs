use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, ContractId, Salt},
};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Fundraiser, "out/debug/fundraiser-abi.json");
abigen!(Asset, "tests/artifacts/asset/out/debug/asset-abi.json");

pub struct DefaultParameters {
    pub asset_id: ContractId,
    pub beneficiary: Identity,
    pub deadline: u64,
    pub target_amount: u64,
}

pub struct Metadata {
    pub contract: Fundraiser,
    pub wallet: LocalWallet,
}

pub struct MetaAsset {
    pub contract: Asset,
    pub id: ContractId,
}

pub mod abi_calls {

    use super::*;

    pub async fn create_campaign(
        contract: &Fundraiser,
        asset: &ContractId,
        beneficiary: &Identity,
        deadline: u64,
        target_amount: u64,
    ) -> CallResponse<()> {
        contract
            .create_campaign(asset.clone(), beneficiary.clone(), deadline, target_amount)
            .call()
            .await
            .unwrap()
    }

    pub async fn cancel_campaign(contract: &Fundraiser, id: u64) -> CallResponse<()> {
        contract.cancel_campaign(id).call().await.unwrap()
    }

    pub async fn claim_pledges(contract: &Fundraiser, id: u64) -> CallResponse<()> {
        contract
            .claim_pledges(id)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn pledge(
        contract: &Fundraiser,
        id: u64,
        asset: &MetaAsset,
        amount: u64,
    ) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset.id)));

        contract
            .pledge(id)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn unpledge(contract: &Fundraiser, id: u64, amount: u64) -> CallResponse<()> {
        contract
            .unpledge(id, amount)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn total_campaigns(contract: &Fundraiser) -> u64 {
        contract.total_campaigns().call().await.unwrap().value
    }

    pub async fn campaign_info(contract: &Fundraiser, id: u64) -> CallResponse<CampaignInfo> {
        contract.campaign_info(id).call().await.unwrap()
    }

    pub async fn user_campaign_count(contract: &Fundraiser, user: Identity) -> u64 {
        contract
            .user_campaign_count(user)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn campaign(
        contract: &Fundraiser,
        id: u64,
        user: Identity,
    ) -> CallResponse<Campaign> {
        contract.campaign(id, user).call().await.unwrap()
    }

    pub async fn pledge_count(contract: &Fundraiser, user: Identity) -> u64 {
        contract.pledge_count(user).call().await.unwrap().value
    }

    pub async fn pledged(contract: &Fundraiser, id: u64, user: Identity) -> CallResponse<Pledge> {
        contract.pledged(id, user).call().await.unwrap()
    }

    pub async fn asset_count(contract: &Fundraiser) -> u64 {
        contract.asset_count().call().await.unwrap().value
    }

    pub async fn asset_info_by_id(
        contract: &Fundraiser,
        asset: &ContractId,
    ) -> CallResponse<AssetInfo> {
        contract.asset_info_by_id(*asset).call().await.unwrap()
    }

    pub async fn asset_info_by_count(contract: &Fundraiser, id: u64) -> CallResponse<AssetInfo> {
        contract.asset_info_by_count(id).call().await.unwrap()
    }
}

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (Metadata, Metadata, MetaAsset, MetaAsset, DefaultParameters) {
        let num_wallets = 3;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );

        let mut wallets = launch_provider_and_get_wallets(config).await;

        let deployer_wallet = wallets.pop().unwrap();
        let author_wallet = wallets.pop().unwrap();
        let user_wallet = wallets.pop().unwrap();

        let id = Contract::deploy(
            "./out/debug/fundraiser.bin",
            &deployer_wallet,
            TxParameters::default(),
        )
        .await
        .unwrap();

        let asset_id = Contract::deploy(
            "./tests/artifacts/asset/out/debug/asset.bin",
            &deployer_wallet,
            TxParameters::default(),
        )
        .await
        .unwrap();

        let asset2_id = Contract::deploy_with_salt(
            "./tests/artifacts/asset/out/debug/asset.bin",
            &deployer_wallet,
            TxParameters::default(),
            Salt::from([1u8; 32]),
        )
        .await
        .unwrap();

        let author = Metadata {
            contract: Fundraiser::new(id.to_string(), author_wallet.clone()),
            wallet: author_wallet,
        };

        let user = Metadata {
            contract: Fundraiser::new(id.to_string(), user_wallet.clone()),
            wallet: user_wallet.clone(),
        };

        let asset = MetaAsset {
            contract: Asset::new(asset_id.to_string(), deployer_wallet.clone()),
            id: asset_id,
        };

        let asset2 = MetaAsset {
            contract: Asset::new(asset2_id.to_string(), deployer_wallet),
            id: asset2_id,
        };

        let defaults = DefaultParameters {
            asset_id,
            beneficiary: Identity::Address(user_wallet.address()),
            deadline: 100,
            target_amount: 512,
        };

        (author, user, asset, asset2, defaults)
    }

    pub async fn mint(contract: &Asset, amount: u64, address: Address) -> bool {
        contract
            .mint_and_send_to_address(amount, address)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value
    }
}
