#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Fundraiser, "out/debug/fundraiser-abi.json");

struct Metadata {
    contract: Fundraiser,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata) {
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

    let author = Metadata {
        contract: Fundraiser::new(id.to_string(), author_wallet.clone()),
        wallet: author_wallet,
    };

    let user = Metadata {
        contract: Fundraiser::new(id.to_string(), user_wallet.clone()),
        wallet: user_wallet,
    };

    (author, user)
}

mod create_campaign {

    use super::*;

    #[tokio::test]
    async fn creates_a_campaign() {
        let (author, user) = setup().await;

        assert_eq!(
            0,
            author.contract.campaign_count().call().await.unwrap().value
        );

        author
            .contract
            .create_campaign(
                ContractId::from([1u8; 32]),
                Identity::Address(Address::from([0u8; 32])),
                3,
                512,
            )
            .call()
            .await
            .unwrap();

        assert_eq!(
            1,
            author.contract.campaign_count().call().await.unwrap().value
        );
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_asset_is_base_asset() {
        let (author, user) = setup().await;

        // Should panic
        author
            .contract
            .create_campaign(
                ContractId::from([0u8; 32]),
                Identity::Address(Address::from([0u8; 32])),
                2,
                1776,
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_deadline_is_in_the_past() {
        // TODO: retrieve the block height and subtract 1 from it
        let (author, user) = setup().await;

        // Should panic
        author
            .contract
            .create_campaign(
                ContractId::from([0u8; 32]),
                Identity::Address(Address::from([0u8; 32])),
                1,
                1776,
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_target_amount_is_zero() {
        // TODO: retrieve the block height and add 1 to it
        let (author, user) = setup().await;

        // Should panic
        author
            .contract
            .create_campaign(
                ContractId::from([0u8; 32]),
                Identity::Address(Address::from([0u8; 32])),
                2,
                0,
            )
            .call()
            .await
            .unwrap();
    }
}

mod total_campaigns {

    use super::*;

    #[tokio::test]
    async fn returns_0() {
        let (author, user) = setup().await;

        assert_eq!(
            0,
            author
                .contract
                .total_campaigns()
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    async fn returns_1() {
        let (author, user) = setup().await;

        author
            .contract
            .create_campaign(
                ContractId::from([1u8; 32]),
                Identity::Address(Address::from([0u8; 32])),
                112,
                1776,
            )
            .call()
            .await
            .unwrap();

        assert_eq!(
            1,
            author
                .contract
                .total_campaigns()
                .call()
                .await
                .unwrap()
                .value
        );
    }
}

mod campaign_info {

    use super::*;

    #[tokio::test]
    async fn returns_info() {
        let (author, user) = setup().await;

        author
            .contract
            .create_campaign(
                ContractId::from([1u8; 32]),
                Identity::Address(Address::from([0u8; 32])),
                2,
                512,
            )
            .call()
            .await
            .unwrap();

        let campaign = author.contract.campaign_info(1).call().await.unwrap().value;

        assert_eq!(campaign.author, Identity::Address(author.wallet.address()));
        assert_eq!(campaign.asset, ContractId::from([1u8; 32]));
        assert_eq!(campaign.cancelled, false);
        assert_eq!(campaign.claimed, false);
        assert_eq!(campaign.deadline, 2);
        assert_eq!(campaign.target_amount, 512);
        assert_eq!(campaign.total_pledge, 0);
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_zero() {
        let (author, user) = setup().await;

        // Should panic
        author.contract.campaign_info(0).call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_id_is_greater_than_number_of_campaigns() {
        let (author, user) = setup().await;

        // Should panic
        author.contract.campaign_info(1).call().await.unwrap();
    }
}

mod campaign_count {

    use super::*;

    #[tokio::test]
    async fn returns_0() {
        let (author, user) = setup().await;

        assert_eq!(
            0,
            author.contract.campaign_count().call().await.unwrap().value
        );
    }

    #[tokio::test]
    async fn returns_1() {
        let (author, user) = setup().await;

        author
            .contract
            .create_campaign(
                ContractId::from([1u8; 32]),
                Identity::Address(Address::from([0u8; 32])),
                112,
                1776,
            )
            .call()
            .await
            .unwrap();

        assert_eq!(
            1,
            author.contract.campaign_count().call().await.unwrap().value
        );
    }
}

mod pledge_count {

    use super::*;

    #[tokio::test]
    async fn returns_0() {
        let (author, user) = setup().await;

        assert_eq!(0, user.contract.pledge_count().call().await.unwrap().value);
    }

    // #[tokio::test]
    // async fn returns_1() {
    //     let (author, user) = setup().await;

    //     author.contract.create_campaign(ContractId::from([1u8; 32]), Identity::Address(Address::from([0u8; 32])), 112, 1776).call().await.unwrap();
    //     user.contract.pledge(1).call().await.unwrap();

    //     assert_eq!(1, user.contract.pledge_count().call().await.unwrap().value);
    // }
}
