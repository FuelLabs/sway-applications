use crate::utils::{
    interface::core::{
        asset::mint_and_send_to_address,
        nft::{approve, mint},
        token_distributor::{create, end, purchase, purchase_admin},
    },
    setup::{defaults, setup},
};
use fuels::{prelude::Address, types::Identity};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::token_distributor::token_distribution, setup::wallet_balance,
    };

    #[tokio::test]
    async fn purchases_ownership() {
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
        let owner2_identity = Identity::Address(owner2.wallet.address().into());
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
        assert_eq!(wallet_balance(asset_contract, &owner1.wallet).await, 0);
        assert_eq!(
            token_distribution_struct.clone().unwrap().admin,
            Some(owner_identity.clone())
        );

        purchase_admin(
            Some(owner2_identity.clone()),
            reserve_price,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            Some(reserve_price * 2),
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            wallet_balance(asset_contract, &owner1.wallet).await,
            reserve_price
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().admin,
            Some(owner2_identity.clone())
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "DistributionDoesNotExist")]
    async fn when_token_distribution_does_not_exist() {
        let (
            _deployer,
            _owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            _nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, _token_price, _token_supply, _purchase_amount, asset_supply) =
            defaults().await;

        let owner2_identity = Identity::Address(owner2.wallet.address().into());
        mint_and_send_to_address(
            asset_supply,
            &owner2.asset,
            Address::new(*owner2.wallet.address().hash()),
        )
        .await;

        purchase_admin(
            Some(owner2_identity.clone()),
            reserve_price,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            Some(reserve_price * 2),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "NoReserveAvailable")]
    async fn when_no_reserve() {
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
        let owner2_identity = Identity::Address(owner2.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            None,
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

        purchase_admin(
            Some(owner2_identity.clone()),
            reserve_price,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            Some(reserve_price * 2),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidState")]
    async fn when_closed() {
        let (
            _deployer,
            owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let owner2_identity = Identity::Address(owner2.wallet.address().into());
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
        mint_and_send_to_address(
            asset_supply,
            &owner2.asset,
            Address::new(*owner2.wallet.address().hash()),
        )
        .await;

        purchase_admin(
            Some(owner2_identity.clone()),
            reserve_price,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            Some(reserve_price * 2),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAssetTransfer")]
    async fn when_incorrect_amount_provided() {
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
        let owner2_identity = Identity::Address(owner2.wallet.address().into());
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

        purchase_admin(
            Some(owner2_identity.clone()),
            reserve_price - 1,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            Some(reserve_price * 2),
        )
        .await;
    }
}
