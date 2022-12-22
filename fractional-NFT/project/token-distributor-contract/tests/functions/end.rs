use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    fractional_nft_abi_calls::nft_info,
    nft_abi_calls::{approve, mint, owner_of},
    test_helpers::{defaults, setup},
    token_distributor_abi_calls::{
        buyback, create, end, purchase, purchase_admin, sell, token_distribution,
    },
    token_distributor_mod::DistributionState,
};
use fuels::{
    prelude::{Address, Bech32ContractId, Identity},
    signers::Signer,
    tx::AssetId,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn ends() {
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
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
        )
        .await;
        purchase(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;
        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;
        sell(
            purchase_amount,
            &owner2.token_distributor,
            fractional_nft_contract.clone(),
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Buyback()
        );

        end(
            &owner1.token_distributor,
            fractional_nft_contract.clone(),
            nft_contract.clone(),
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(owner_of(&owner1.nft, 0).await, Some(owner_identity.clone()));
        assert_eq!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Ended()
        );
    }

    #[tokio::test]
    async fn cancels_distribution() {
        let (
            deployer,
            owner1,
            _owner2,
            token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());
        let token_distributor_identity = Identity::ContractId(token_distributor_contract.into());
        let provider = deployer.wallet.get_provider().unwrap();

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
            Some(reserve_price),
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract.clone()),
                    AssetId::from(*fractional_nft_contract.clone())
                )
                .await
                .unwrap(),
            token_supply
        );
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(nft_struct.clone().unwrap().asset_id, nft_contract.clone());
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(token_distributor_identity.clone())
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Started()
        );

        end(
            &owner1.token_distributor,
            fractional_nft_contract.clone(),
            nft_contract.clone(),
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract.clone()),
                    AssetId::from(*fractional_nft_contract.clone())
                )
                .await
                .unwrap(),
            0
        );
        assert_eq!(owner_of(&owner1.nft, 0).await, Some(owner_identity.clone()));
        assert_eq!(nft_struct.clone().unwrap().asset_id, nft_contract.clone());
        assert_eq!(nft_struct.clone().unwrap().admin, None);
        assert_eq!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Ended()
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
            owner1,
            _owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            _asset_contract,
        ) = setup().await;
        let (_reserve_price, _token_price, _token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        end(
            &owner1.token_distributor,
            fractional_nft_contract.clone(),
            nft_contract.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotTokenAdmin")]
    async fn when_no_owner() {
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
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
        )
        .await;
        purchase(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;
        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;
        sell(
            purchase_amount,
            &owner2.token_distributor,
            fractional_nft_contract.clone(),
        )
        .await;
        purchase_admin(
            None,
            reserve_price,
            &owner2.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            Some(reserve_price * 2),
        )
        .await;

        end(
            &owner1.token_distributor,
            fractional_nft_contract.clone(),
            nft_contract.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidState")]
    async fn when_not_in_returning_state() {
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
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
        )
        .await;
        purchase(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;

        end(
            &owner1.token_distributor,
            fractional_nft_contract.clone(),
            nft_contract.clone(),
        )
        .await;
    }

    #[tokio::test]
    // TODO: This test expects an error from another contract, upstream error messages not yet supported
    // #[should_panic(expected = "SupplyNotReturned")]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_not_all_assets_returned() {
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
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
        )
        .await;
        purchase(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;
        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;
        sell(
            purchase_amount - 1,
            &owner2.token_distributor,
            fractional_nft_contract.clone(),
        )
        .await;

        end(
            &owner1.token_distributor,
            fractional_nft_contract.clone(),
            nft_contract.clone(),
        )
        .await;
    }
}
