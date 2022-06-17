#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, ContractId},
};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Fundraiser, "out/debug/fundraiser-abi.json");
abigen!(Asset, "tests/artifacts/asset/out/debug/asset-abi.json");

struct Metadata {
    contract: Fundraiser,
    wallet: LocalWallet,
}

struct MetaAsset {
    contract: Asset,
    id: ContractId,
}

async fn setup() -> (Metadata, Metadata, MetaAsset) {
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

    let author = Metadata {
        contract: Fundraiser::new(id.to_string(), author_wallet.clone()),
        wallet: author_wallet,
    };

    let user = Metadata {
        contract: Fundraiser::new(id.to_string(), user_wallet.clone()),
        wallet: user_wallet,
    };

    let asset = MetaAsset {
        contract: Asset::new(asset_id.to_string(), deployer_wallet),
        id: asset_id,
    };

    (author, user, asset)
}

async fn create_campaign(
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

async fn campaign_count(contract: &Fundraiser) -> u64 {
    contract.campaign_count().call().await.unwrap().value
}

async fn total_campaigns(contract: &Fundraiser) -> u64 {
    contract.total_campaigns().call().await.unwrap().value
}

async fn campaign(contract: &Fundraiser, id: u64) -> CallResponse<Campaign> {
    contract.campaign(id).call().await.unwrap()
}

async fn campaign_info(contract: &Fundraiser, id: u64) -> CallResponse<CampaignInfo> {
    contract.campaign_info(id).call().await.unwrap()
}

async fn pledge_count(contract: &Fundraiser) -> u64 {
    contract.pledge_count().call().await.unwrap().value
}

async fn cancel_campaign(contract: &Fundraiser, id: u64) -> CallResponse<()> {
    contract.cancel_campaign(id).call().await.unwrap()
}

async fn mint(contract: &Asset, amount: u64, address: Address) -> bool {
    contract
        .mint_and_send_to_address(amount, address)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}

async fn pledge(
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

async fn unpledge(contract: &Fundraiser, id: u64, amount: u64) -> CallResponse<()> {
    contract
        .unpledge(id, amount)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

async fn claim_pledges(contract: &Fundraiser, id: u64) -> CallResponse<()> {
    contract
        .claim_pledges(id)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

mod create_campaign {

    // TODO: create 2 separate campaigns

    use super::*;

    #[tokio::test]
    async fn creates_a_campaign() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        assert_eq!(0, total_campaigns(&author.contract).await);
        assert_eq!(0, campaign_count(&author.contract).await);

        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        let info = campaign_info(&author.contract, 1).await.value;

        assert_eq!(1, total_campaigns(&author.contract).await);
        assert_eq!(1, campaign_count(&author.contract).await);
        assert_eq!(1, campaign(&author.contract, 1).await.value.id);
        assert_eq!(info.asset, asset_id);
        assert_eq!(info.author, Identity::Address(author.wallet.address()));
        assert_eq!(info.beneficiary, beneficiary);
        assert_eq!(info.cancelled, false);
        assert_eq!(info.claimed, false);
        assert_eq!(info.deadline, deadline);
        assert_eq!(info.target_amount, target_amount);
        assert_eq!(info.total_pledge, 0);
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_asset_is_base_asset() {
        let (author, user, asset) = setup().await;
        let asset_id = ContractId::from([0u8; 32]);
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        // Should panic
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_deadline_is_in_the_past() {
        let (author, user, asset) = setup().await;
        let asset_id = ContractId::from([0u8; 32]);
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 0;
        let target_amount = 512;

        // Should panic
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_target_amount_is_zero() {
        let (author, user, asset) = setup().await;
        let asset_id = ContractId::from([0u8; 32]);
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 0;

        // Should panic
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
    }
}

mod cancel_campaign {

    use super::*;

    #[tokio::test]
    async fn cancels() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;

        let info = campaign_info(&author.contract, 1).await.value;
        assert_eq!(info.cancelled, false);

        cancel_campaign(&author.contract, 1).await;

        let info = campaign_info(&author.contract, 1).await.value;
        assert_eq!(info.cancelled, true);
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_zero() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;

        // Should panic
        cancel_campaign(&author.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_greater_than_number_of_campaigns() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        // Should panic
        cancel_campaign(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_sender_is_not_author() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 4;
        let target_amount = 512;

        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;

        // Should panic
        cancel_campaign(&user.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_calling_after_deadline() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 3;
        let target_amount = 512;

        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;

        // Should panic
        cancel_campaign(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_calling_after_already_cancelling() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        cancel_campaign(&author.contract, 1).await;

        // Should panic
        cancel_campaign(&author.contract, 1).await;
    }
}

mod claim_pledges {

    use super::*;

    #[tokio::test]
    async fn claims() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(author.wallet.address());
        let deadline = 5;
        let target_amount = 512;

        mint(&asset.contract, target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, target_amount).await;
        claim_pledges(&author.contract, 1).await;

        let info = campaign_info(&author.contract, 1).await.value;
        let claimed = author
            .wallet
            .get_asset_balance(&AssetId::from(*asset.id))
            .await
            .unwrap();

        assert_eq!(target_amount, claimed);
        assert_eq!(info.claimed, true);
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_zero() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(author.wallet.address());
        let deadline = 5;
        let target_amount = 512;

        mint(&asset.contract, target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, target_amount).await;

        // Should panic
        claim_pledges(&author.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_greater_than_number_of_campaigns() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(author.wallet.address());
        let deadline = 5;
        let target_amount = 512;

        mint(&asset.contract, target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, target_amount).await;

        // Should panic
        claim_pledges(&author.contract, 100).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_sender_is_not_author() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 4;
        let target_amount = 512;

        mint(&asset.contract, target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, target_amount).await;

        // Should panic
        claim_pledges(&user.contract, 1).await;
    }

    // #[tokio::test]
    // #[should_panic(expected = "Revert(42)")]
    // async fn panics_when_calling_before_deadline() {
    //     let (author, user, asset) = setup().await;
    //     let asset_id = asset.id;
    //     let beneficiary = Identity::Address(user.wallet.address());
    //     let deadline = 5;
    //     let target_amount = 512;

    //     mint(&asset.contract, target_amount, user.wallet.address()).await;
    //     create_campaign(
    //         &author.contract,
    //         &asset_id,
    //         &beneficiary,
    //         deadline,
    //         target_amount,
    //     )
    //     .await;
    //     pledge(&user.contract, 1, &asset, target_amount).await;

    //     create_campaign(
    //         &author.contract,
    //         &asset_id,
    //         &beneficiary,
    //         deadline*2,
    //         target_amount,
    //     )
    //     .await;

    //     // Should panic
    //     claim_pledges(&author.contract, 1).await;
    // }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_target_amount_is_not_reached() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(author.wallet.address());
        let deadline = 5;
        let target_amount = 512;

        mint(&asset.contract, target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, target_amount - 1).await;

        // Should panic
        claim_pledges(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_already_claimed() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(author.wallet.address());
        let deadline = 5;
        let target_amount = 512;

        mint(&asset.contract, target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, target_amount).await;
        claim_pledges(&author.contract, 1).await;

        // Should panic
        claim_pledges(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_cancelled() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(author.wallet.address());
        let deadline = 6;
        let target_amount = 512;

        mint(&asset.contract, target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, target_amount).await;
        cancel_campaign(&author.contract, 1).await;

        // Should panic
        claim_pledges(&author.contract, 1).await;
    }
}

mod pledge {

    use super::*;
}

mod unpledge {

    use super::*;
}

mod total_campaigns {

    use super::*;

    #[tokio::test]
    async fn returns_zero() {
        let (author, user, asset) = setup().await;

        assert_eq!(0, total_campaigns(&author.contract).await);
    }

    #[tokio::test]
    async fn returns_one() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        assert_eq!(0, total_campaigns(&author.contract).await);
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        assert_eq!(1, total_campaigns(&author.contract).await);
    }
}

mod campaign_info {

    use super::*;

    #[tokio::test]
    async fn returns_info() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;

        let info = campaign_info(&author.contract, 1).await.value;

        assert_eq!(info.asset, asset_id);
        assert_eq!(info.author, Identity::Address(author.wallet.address()));
        assert_eq!(info.beneficiary, beneficiary);
        assert_eq!(info.cancelled, false);
        assert_eq!(info.claimed, false);
        assert_eq!(info.deadline, deadline);
        assert_eq!(info.target_amount, target_amount);
        assert_eq!(info.total_pledge, 0);
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_zero() {
        let (author, user, asset) = setup().await;

        // Should panic
        campaign_info(&author.contract, 0).await.value;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_greater_than_number_of_campaigns() {
        let (author, user, asset) = setup().await;

        // Should panic
        campaign_info(&author.contract, 1).await.value;
    }
}

mod campaign_count {

    use super::*;

    #[tokio::test]
    async fn returns_zero() {
        let (author, user, asset) = setup().await;

        assert_eq!(0, campaign_count(&author.contract).await);
    }

    #[tokio::test]
    async fn returns_one() {
        let (author, user, asset) = setup().await;
        let asset_id = asset.id;
        let beneficiary = Identity::Address(user.wallet.address());
        let deadline = 100;
        let target_amount = 512;

        assert_eq!(0, campaign_count(&author.contract).await);
        create_campaign(
            &author.contract,
            &asset_id,
            &beneficiary,
            deadline,
            target_amount,
        )
        .await;
        assert_eq!(1, campaign_count(&author.contract).await);
    }
}

mod campaign {

    use super::*;
}

mod pledge_count {

    use super::*;

    #[tokio::test]
    async fn returns_0() {
        let (author, user, asset) = setup().await;

        assert_eq!(0, pledge_count(&user.contract).await);
    }

    // #[tokio::test]
    // async fn returns_1() {
    //     let (author, user, asset) = setup().await;

    //     author.contract.create_campaign(ContractId::from([1u8; 32]), Identity::Address(Address::from([0u8; 32])), 112, 1776).call().await.unwrap();
    //     user.contract.pledge(1).call().await.unwrap();

    //     assert_eq!(1, user.contract.pledge_count().call().await.unwrap().value);
    // }
}

mod pledged {

    use super::*;
}
