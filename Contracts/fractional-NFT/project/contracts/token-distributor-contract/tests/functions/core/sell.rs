use crate::utils::{
    interface::core::{
        asset::mint_and_send_to_address,
        nft::{approve, mint},
        token_distributor::{buyback, create, purchase, sell},
    },
    setup::{defaults, setup},
};
use fuels::{
    prelude::{Address, Bech32ContractId},
    tx::AssetId,
    types::Identity,
};

mod success {

    use super::*;
    use crate::utils::setup::wallet_balance;

    #[tokio::test]
    async fn sells() {
        let (
            deployer,
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
        let provider = deployer.wallet.provider().unwrap();

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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
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
        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            token_price,
        )
        .await;

        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply - (purchase_amount * token_price)
        );
        assert_eq!(
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            purchase_amount
        );
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(fractional_nft_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            0
        );

        sell(
            purchase_amount,
            &owner2.token_distributor,
            fractional_nft_contract,
        )
        .await;

        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply
        );
        assert_eq!(
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            0
        );
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(fractional_nft_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            purchase_amount
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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
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
        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            token_price,
        )
        .await;

        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply - (purchase_amount * token_price)
        );
        assert_eq!(
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            purchase_amount
        );

        sell(
            purchase_amount - 1,
            &owner2.token_distributor,
            fractional_nft_contract,
        )
        .await;

        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply - token_price
        );
        assert_eq!(
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            1
        );
    }
}

mod revert {

    use super::*;
    use fuels::prelude::{CallParameters, TxParameters};

    #[tokio::test]
    #[should_panic(expected = "DistributionDoesNotExist")]
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

        sell(purchase_amount, &owner2.token_distributor, asset_contract).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidState")]
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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
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

        sell(
            purchase_amount,
            &owner2.token_distributor,
            fractional_nft_contract,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAssetTransfer")]
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
        mint_and_send_to_address(
            asset_supply,
            &owner1.asset,
            Address::new(*owner1.wallet.address().hash()),
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
        buyback(
            purchase_amount * token_price,
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            token_price,
        )
        .await;

        let tx_params = TxParameters::new(0, 2_000_000, 0);
        let call_params = CallParameters::new(
            asset_supply - (purchase_amount * token_price),
            AssetId::from(*asset_contract),
            1_000_000,
        );

        owner2
            .token_distributor
            .methods()
            .sell(fractional_nft_contract)
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(1)
            .set_contract_ids(&[Bech32ContractId::from(fractional_nft_contract)])
            .call()
            .await
            .unwrap();
    }
}
