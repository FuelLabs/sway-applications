use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    nft_abi_calls::{approve, mint},
    test_helpers::{defaults, setup},
    token_distributor_abi_calls::{cancel, create, purchase},
};
use fuels::{
    prelude::{Address, CallParameters, Identity, TxParameters},
    signers::Signer,
    tx::AssetId,
};

mod succes {

    use super::*;

    #[tokio::test]
    async fn purchases_tokens() {
        let (
            _deployer,
            owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, purchase_amount, asset_supply) = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
            Some(owner_identity.clone()),
            Some(reserve_price),
            token_price,
            token_supply,
            0,
        )
        .await;
        mint_and_send_to_address(asset_supply, &owner2.asset, Address::new(*owner2.wallet.address().hash())).await;
        
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            0
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply
        );
        
        purchase(purchase_amount, &owner2.token_distributor, asset_contract.clone(), fractional_nft_contract.clone(), token_price).await;

        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            purchase_amount
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply - (purchase_amount * token_price)
        );
    }

    #[tokio::test]
    async fn purchases_all_tokens() {
        let (
            _deployer,
            owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, asset_supply) = defaults().await;

        let purchase_amount = token_supply;
        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
            Some(owner_identity.clone()),
            Some(reserve_price),
            token_price,
            token_supply,
            0,
        )
        .await;
        mint_and_send_to_address(asset_supply, &owner2.asset, Address::new(*owner2.wallet.address().hash())).await;
        
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            0
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply
        );
        
        purchase(purchase_amount, &owner2.token_distributor, asset_contract.clone(), fractional_nft_contract.clone(), token_price).await;

        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            purchase_amount
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply - (purchase_amount * token_price)
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
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
        let (_reserve_price, token_price, _token_supply, purchase_amount, asset_supply) = defaults().await;

        mint_and_send_to_address(asset_supply, &owner2.asset, Address::new(*owner2.wallet.address().hash())).await;
        
        purchase(purchase_amount, &owner2.token_distributor, asset_contract.clone(), fractional_nft_contract.clone(), token_price).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_token_distribution_closed() {
        let (
            _deployer,
            owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, purchase_amount, asset_supply) = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
            Some(owner_identity.clone()),
            Some(reserve_price),
            token_price,
            token_supply,
            0,
        )
        .await;
        
        cancel(&owner1.token_distributor, fractional_nft_contract.clone(), nft_contract.clone()).await;
        mint_and_send_to_address(asset_supply, &owner2.asset, Address::new(*owner2.wallet.address().hash())).await;
        
        purchase(purchase_amount, &owner2.token_distributor, asset_contract.clone(), fractional_nft_contract.clone(), token_price).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_not_enough_tokens_to_buy() {
        let (
            _deployer,
            owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
            Some(owner_identity.clone()),
            Some(reserve_price),
            token_price,
            token_supply,
            0,
        )
        .await;
        mint_and_send_to_address((token_supply + 1) * token_price, &owner2.asset, Address::new(*owner2.wallet.address().hash())).await;
        
        purchase(token_supply + 1, &owner2.token_distributor, asset_contract.clone(), fractional_nft_contract.clone(), token_price).await;
    }

    
    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_not_providing_correct_amounts() {
        let (
            _deployer,
            owner1,
            owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, purchase_amount, _asset_supply) = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        create(
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            nft_contract.clone(),
            Some(owner_identity.clone()),
            Some(reserve_price),
            token_price,
            token_supply,
            0,
        )
        .await;
        mint_and_send_to_address((token_supply * token_price) + 1, &owner2.asset, Address::new(*owner2.wallet.address().hash())).await;
        
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some((purchase_amount * token_price) + 1),
            Some(AssetId::from(*asset_contract.clone())),
            None,
        );

        owner2
            .token_distributor
            .methods()
            .purchase(purchase_amount, fractional_nft_contract.clone())
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }
}
