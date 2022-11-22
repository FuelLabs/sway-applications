use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, ContractId, Salt},
};

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
    pub wallet: WalletUnlocked,
}

pub struct MetaAsset {
    pub contract: Asset,
    pub id: ContractId,
}

pub mod abi_calls {

    use super::*;

    pub async fn asset_count(contract: &Fundraiser) -> u64 {
        contract.methods().asset_count().call().await.unwrap().value
    }

    pub async fn asset_info_by_id(
        contract: &Fundraiser,
        asset: &ContractId,
    ) -> CallResponse<AssetInfo> {
        contract
            .methods()
            .asset_info_by_id(*asset)
            .call()
            .await
            .unwrap()
    }

    pub async fn asset_info_by_count(contract: &Fundraiser, id: u64) -> CallResponse<AssetInfo> {
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
    ) -> CallResponse<Campaign> {
        contract.methods().campaign(id, user).call().await.unwrap()
    }

    pub async fn campaign_info(contract: &Fundraiser, id: u64) -> CallResponse<CampaignInfo> {
        contract.methods().campaign_info(id).call().await.unwrap()
    }

    pub async fn cancel_campaign(contract: &Fundraiser, id: u64) -> CallResponse<()> {
        contract.methods().cancel_campaign(id).call().await.unwrap()
    }

    pub async fn claim_pledges(contract: &Fundraiser, id: u64) -> CallResponse<()> {
        contract
            .methods()
            .claim_pledges(id)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn create_campaign(
        contract: &Fundraiser,
        asset: &ContractId,
        beneficiary: &Identity,
        deadline: u64,
        target_amount: u64,
    ) -> CallResponse<()> {
        contract
            .methods()
            .create_campaign(asset.clone(), beneficiary.clone(), deadline, target_amount)
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
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset.id)), None);

        contract
            .methods()
            .pledge(id)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn pledged(contract: &Fundraiser, id: u64, user: Identity) -> CallResponse<Pledge> {
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

    pub async fn unpledge(contract: &Fundraiser, id: u64, amount: u64) -> CallResponse<()> {
        contract
            .methods()
            .unpledge(id, amount)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
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
}

pub mod test_helpers {

    use super::*;

    pub async fn identity(address: &Bech32Address) -> Identity {
        Identity::Address(address.into())
    }

    pub async fn mint(contract: &Asset, amount: u64, address: &Bech32Address) -> bool {
        contract
            .methods()
            .mint_and_send_to_address(amount, address.into())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn setup() -> (Metadata, Metadata, MetaAsset, MetaAsset, DefaultParameters) {
        let num_wallets = 3;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );

        let mut wallets = launch_custom_provider_and_get_wallets(config, None).await;

        let deployer_wallet = wallets.pop().unwrap();
        let author_wallet = wallets.pop().unwrap();
        let user_wallet = wallets.pop().unwrap();

        let id = Contract::deploy(
            paths::CONTRACT_BINARY,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                paths::CONTRACT_STORAGE.to_string(),
            )),
        )
        .await
        .unwrap();

        let asset_id = Contract::deploy(
            paths::ASSET_BINARY,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                paths::ASSET_STORAGE.to_string(),
            )),
        )
        .await
        .unwrap();

        let asset2_id = Contract::deploy_with_parameters(
            paths::ASSET_BINARY,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                paths::ASSET_STORAGE.to_string(),
            )),
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
            id: asset_id.clone().into(),
        };

        let asset2 = MetaAsset {
            contract: Asset::new(asset2_id.to_string(), deployer_wallet),
            id: asset2_id.into(),
        };

        let defaults = DefaultParameters {
            asset_id: asset_id.into(),
            beneficiary: Identity::Address(user_wallet.address().into()),
            deadline: 100,
            target_amount: 512,
        };

        (author, user, asset, asset2, defaults)
    }
}

pub mod paths {
    pub const CONTRACT_BINARY: &str = "./out/debug/fundraiser.bin";
    pub const CONTRACT_STORAGE: &str = "./out/debug/fundraiser-storage_slots.json";
    pub const ASSET_BINARY: &str = "./tests/artifacts/asset/out/debug/asset.bin";
    pub const ASSET_STORAGE: &str = "./tests/artifacts/asset/out/debug/asset-storage_slots.json";
}
