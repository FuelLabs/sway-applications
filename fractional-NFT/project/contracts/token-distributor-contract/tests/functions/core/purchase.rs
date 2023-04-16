use crate::utils::{
    interface::core::{
        asset::mint_and_send_to_address,
        nft::{approve, mint},
        token_distributor::{create, end, purchase},
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

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            0
        );
        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            0
        );

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
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            purchase_amount
        );
        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply - (purchase_amount * token_price)
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            (purchase_amount * token_price)
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
        let (reserve_price, token_price, token_supply, _purchase_amount, asset_supply) =
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

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            0
        );
        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            0
        );

        purchase(
            token_supply,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            token_price,
        )
        .await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            wallet_balance(fractional_nft_contract, &owner2.wallet).await,
            token_supply
        );
        assert_eq!(
            wallet_balance(asset_contract, &owner2.wallet).await,
            asset_supply - (token_supply * token_price)
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            (token_supply * token_price)
        );
    }
}

mod revert {

    use super::*;
    use fuels::{
        prelude::{CallParameters, TxParameters},
        tx::AssetId,
    };

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
        let (_reserve_price, token_price, _token_supply, purchase_amount, asset_supply) =
            defaults().await;

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
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidState")]
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

        purchase(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            token_price,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotEnoughTokensAvailable")]
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
        mint_and_send_to_address(
            (token_supply + 1) * token_price,
            &owner2.asset,
            Address::new(*owner2.wallet.address().hash()),
        )
        .await;

        purchase(
            token_supply + 1,
            &owner2.token_distributor,
            asset_contract,
            fractional_nft_contract,
            token_price,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAssetTransfer")]
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
        let (reserve_price, token_price, token_supply, purchase_amount, _asset_supply) =
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
            (token_supply * token_price) + 1,
            &owner2.asset,
            Address::new(*owner2.wallet.address().hash()),
        )
        .await;

        let tx_params = TxParameters::new(0, 2_000_000, 0);
        let call_params = CallParameters::new(
            (purchase_amount * token_price) + 1,
            AssetId::from(*asset_contract),
            1_000_000,
        );

        owner2
            .token_distributor
            .methods()
            .purchase(purchase_amount, fractional_nft_contract)
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }
}
