use crate::utils::{
    fractional_nft_abi_calls::{deposit, nft_info},
    nft_abi_calls::{approve, mint},
    test_helpers::{defaults, setup},
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn get_info_on_none() {
        let (_deployer, owner1, _owner2, _fractional_nft_contract, _nft_contract) = setup().await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(nft_struct, None);
    }

    #[tokio::test]
    async fn get_info_on_sone() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(nft_struct, None);

        deposit(
            &owner1.f_nft,
            nft_contract.clone(),
            Some(owner_identity.clone()),
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert!(nft_struct.is_some());
        assert_eq!(nft_struct.clone().unwrap().nft, nft_contract.clone());
        assert_eq!(
            nft_struct.clone().unwrap().owner,
            Some(owner_identity.clone())
        );
    }
}