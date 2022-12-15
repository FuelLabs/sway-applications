use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    nft_abi_calls::{approve, mint},
    test_helpers::{defaults, setup},
    token_distributor_abi_calls::{create, purchase, request_return, sell},
};
use fuels::{
    prelude::{Address, Bech32ContractId, CallParameters, Identity, TxParameters},
    signers::Signer,
    tx::AssetId,
};

mod succes {

    use super::*;

    #[tokio::test]
    async fn sells() {
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
            Some(owner_identity.clone()),
            Some(reserve_price),
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
        request_return(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;

        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply - (purchase_amount * token_price)
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            purchase_amount
        );

        sell(
            purchase_amount,
            &owner2.token_distributor,
            fractional_nft_contract.clone(),
        )
        .await;

        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn sells_some() {
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
            Some(owner_identity.clone()),
            Some(reserve_price),
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
        request_return(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;

        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply - (purchase_amount * token_price)
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            purchase_amount
        );

        sell(
            purchase_amount - 1,
            &owner2.token_distributor,
            fractional_nft_contract.clone(),
        )
        .await;

        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*asset_contract))
                .await
                .unwrap(),
            asset_supply - token_price
        );
        assert_eq!(
            owner2
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            1
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
            _fractional_nft_contract,
            _nft_contract,
            asset_contract,
        ) = setup().await;
        let (_reserve_price, _token_price, _token_supply, purchase_amount, asset_supply) =
            defaults().await;

        mint_and_send_to_address(
            asset_supply,
            &owner2.asset,
            Address::new(*owner2.wallet.address().hash()),
        )
        .await;

        sell(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_not_accepting_returns() {
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
            Some(owner_identity.clone()),
            Some(reserve_price),
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

        sell(
            purchase_amount,
            &owner2.token_distributor,
            fractional_nft_contract.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_incorrect_asset_type() {
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
            Some(owner_identity.clone()),
            Some(reserve_price),
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
        request_return(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(asset_supply - (purchase_amount * token_price)),
            Some(AssetId::from(*asset_contract)),
            None,
        );

        owner2
            .token_distributor
            .methods()
            .sell(fractional_nft_contract.clone())
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .set_contracts(&[Bech32ContractId::from(fractional_nft_contract.clone())])
            .call()
            .await
            .unwrap();
    }
}
