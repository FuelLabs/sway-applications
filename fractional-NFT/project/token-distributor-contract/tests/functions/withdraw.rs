use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    nft_abi_calls::{approve, mint},
    test_helpers::{defaults, setup, wallet_balance},
    token_distributor_abi_calls::{create, purchase, token_distribution, withdraw},
};
use fuels::{prelude::Address, signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn withdraws_tokens() {
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
            wallet_balance(asset_contract.clone(), &owner1.wallet).await,
            0
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            (purchase_amount * token_price)
        );

        withdraw(&owner1.token_distributor, fractional_nft_contract.clone()).await;

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract.clone()).await;
        assert_eq!(
            wallet_balance(asset_contract.clone(), &owner1.wallet).await,
            purchase_amount * token_price
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            0
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
            _nft_contract,
            _asset_contract,
        ) = setup().await;
        let (_reserve_price, _token_price, _token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        withdraw(&owner1.token_distributor, fractional_nft_contract.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotTokenAdmin")]
    async fn when_not_admin() {
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
        purchase(
            purchase_amount,
            &owner2.token_distributor,
            asset_contract.clone(),
            fractional_nft_contract.clone(),
            token_price,
        )
        .await;

        withdraw(&owner2.token_distributor, fractional_nft_contract.clone()).await;
    }
}
