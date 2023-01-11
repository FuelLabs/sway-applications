use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    nft_abi_calls::{approve, mint},
    test_helpers::{defaults, setup},
    token_distributor_abi_calls::{buyback, create, purchase, token_distribution},
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
    async fn starts_buyback() {
        let (
            deployer,
            owner1,
            owner2,
            token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, purchase_amount, asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());
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

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract.clone()),
                    AssetId::from(*asset_contract.clone())
                )
                .await
                .unwrap(),
            purchase_amount * token_price
        );
        assert!(matches!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Distributed()
        ));

        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract.clone()),
                    AssetId::from(*asset_contract.clone())
                )
                .await
                .unwrap(),
            (purchase_amount * token_price) * 2
        );
        assert!(matches!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Buyback()
        ));
    }

    #[tokio::test]
    async fn starts_return_at_different_price() {
        let (
            deployer,
            owner1,
            owner2,
            token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, purchase_amount, asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());
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

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract.clone()),
                    AssetId::from(*asset_contract.clone())
                )
                .await
                .unwrap(),
            purchase_amount * token_price
        );
        assert!(matches!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Distributed()
        ));

        buyback(
            purchase_amount * (token_price + 1),
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price + 1,
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract.clone()),
                    AssetId::from(*asset_contract.clone())
                )
                .await
                .unwrap(),
            (purchase_amount * token_price) + (purchase_amount * (token_price + 1))
        );
        assert!(matches!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Buyback()
        ));
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
            _nft_contract,
            asset_contract,
        ) = setup().await;
        let (_reserve_price, token_price, _token_supply, purchase_amount, asset_supply) =
            defaults().await;

        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
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
            None,
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
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidState")]
    async fn when_not_distributed() {
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

        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
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
            (purchase_amount - 1) * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;
    }
}
