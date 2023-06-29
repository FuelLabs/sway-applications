use crate::utils::{
    interface::core::{
        nft::{approve, mint},
        token_distributor::{create, set_token_price},
    },
    setup::{defaults, setup},
};
use fuels::{prelude::Address, types::Identity};

mod success {

    use super::*;
    use crate::utils::interface::{
        core::{asset::mint_and_send_to_address, token_distributor::purchase},
        info::token_distributor::token_distribution,
    };

    #[tokio::test]
    async fn sets_token_price() {
        let (
            _deployer,
            owner1,
            _owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            token_distribution_struct.clone().unwrap().token_price,
            token_price
        );

        set_token_price(
            &owner1.token_distributor,
            fractional_nft_contract,
            token_price + 1,
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            token_distribution_struct.unwrap().token_price,
            token_price + 1
        );
    }

    #[tokio::test]
    async fn sets_token_price_after_purchase() {
        let (
            _deployer,
            owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, purchase_amount, asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;
        mint_and_send_to_address(
            asset_supply,
            &owner2.asset,
            Address::new(*owner2.wallet.address().hash()),
        )
        .await;
        purchase(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            token_price,
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            token_distribution_struct.clone().unwrap().token_price,
            token_price
        );

        set_token_price(
            &owner1.token_distributor,
            fractional_nft_contract,
            token_price + 1,
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            token_distribution_struct.unwrap().token_price,
            token_price + 1
        );
    }
}

mod revert {

    use super::*;
    use crate::utils::interface::core::token_distributor::end;

    #[tokio::test]
    #[should_panic(expected = "DistributionDoesNotExist")]
    async fn when_token_distribution_does_not_exist() {
        let (
            _deployer,
            owner1,
            _owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            _nft_contract,
            _asset_contract,
        ) = setup().await;
        let (_reserve_price, token_price, _token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        set_token_price(
            &owner1.token_distributor,
            fractional_nft_contract,
            token_price + 1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotTokenAdmin")]
    async fn when_not_owner() {
        let (
            _deployer,
            owner1,
            _owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            None,
            token_price,
            token_supply,
            0,
        )
        .await;

        set_token_price(
            &owner1.token_distributor,
            fractional_nft_contract,
            token_price + 1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidState")]
    async fn when_not_in_distributing_state() {
        let (
            _deployer,
            owner1,
            _owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;
        end(
            &owner1.token_distributor,
            &owner1.wallet,
            fractional_nft_contract,
            nft_contract,
        )
        .await;

        set_token_price(
            &owner1.token_distributor,
            fractional_nft_contract,
            token_price + 1,
        )
        .await;
    }
}
