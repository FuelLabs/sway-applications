use crate::utils::{
    abi_calls::{approved, balance_of, constructor, max_supply, mint, owner_of, tokens_minted},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;
    use crate::utils::MintEvent;

    #[tokio::test]
    async fn mints() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
        assert_eq!(owner_of(&owner1.contract, 0).await, None);

        let response = mint(1, &owner1.contract, minter.clone()).await;
        let log = response.get_logs_with_type::<MintEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            MintEvent {
                owner: minter.clone(),
                token_id: 0,
            }
        );
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 1);
        assert_eq!(owner_of(&owner1.contract, 0).await, Some(minter.clone()));
        assert_eq!(approved(&owner1.contract, 0).await, None);
        assert_eq!(tokens_minted(&owner1.contract).await, 1);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
    }

    #[tokio::test]
    async fn mints_with_access() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        let minter = Identity::Address(owner1.wallet.address().into());
        let admin = Some(minter.clone());
        constructor(admin.clone(), &deploy_wallet.contract, Some(1)).await;

        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
        assert_eq!(owner_of(&owner1.contract, 0).await, None);

        let response = mint(1, &owner1.contract, minter.clone()).await;
        let log = response.get_logs_with_type::<MintEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            MintEvent {
                owner: minter.clone(),
                token_id: 0,
            }
        );
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 1);
        assert_eq!(owner_of(&owner1.contract, 0).await, Some(minter.clone()));
        assert_eq!(approved(&owner1.contract, 0).await, None);
        assert_eq!(tokens_minted(&owner1.contract).await, 1);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
    }

    #[tokio::test]
    async fn mints_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(4)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        assert_eq!(max_supply(&owner1.contract).await, Some(4));
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
        assert_eq!(owner_of(&owner1.contract, 0).await, None);
        assert_eq!(owner_of(&owner1.contract, 1).await, None);
        assert_eq!(owner_of(&owner1.contract, 2).await, None);
        assert_eq!(owner_of(&owner1.contract, 3).await, None);

        let response = mint(4, &owner1.contract, minter.clone()).await;
        let log = response.get_logs_with_type::<MintEvent>().unwrap();

        assert_eq!(
            log,
            vec![
                MintEvent {
                    owner: minter.clone(),
                    token_id: 0,
                },
                MintEvent {
                    owner: minter.clone(),
                    token_id: 1,
                },
                MintEvent {
                    owner: minter.clone(),
                    token_id: 2,
                },
                MintEvent {
                    owner: minter.clone(),
                    token_id: 3,
                }
            ]
        );
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 4);
        assert_eq!(tokens_minted(&owner1.contract).await, 4);
        assert_eq!(max_supply(&owner1.contract).await, Some(4));

        assert_eq!(owner_of(&owner1.contract, 0).await, Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 1).await, Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 2).await, Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 3).await, Some(minter.clone()));

        assert_eq!(approved(&owner1.contract, 0).await, None);
        assert_eq!(approved(&owner1.contract, 1).await, None);
        assert_eq!(approved(&owner1.contract, 2).await, None);
        assert_eq!(approved(&owner1.contract, 3).await, None);
    }

    #[tokio::test]
    async fn mint_amount_is_zero() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(tokens_minted(&owner1.contract).await, 0);

        mint(0, &owner1.contract, minter.clone()).await;

        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
    }

    #[tokio::test]
    async fn mints_when_no_token_supply_set() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, None).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
        assert_eq!(owner_of(&owner1.contract, 0).await, None);

        let response = mint(1, &owner1.contract, minter.clone()).await;
        let log = response.get_logs_with_type::<MintEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            MintEvent {
                owner: minter.clone(),
                token_id: 0,
            }
        );
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 1);
        assert_eq!(owner_of(&owner1.contract, 0).await, Some(minter.clone()));
        assert_eq!(approved(&owner1.contract, 0).await, None);
        assert_eq!(tokens_minted(&owner1.contract).await, 1);
        assert_eq!(max_supply(&owner1.contract).await, None);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "MaxTokensMinted")]
    async fn when_minting_more_tokens_than_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(
            max_supply(&owner1.contract).await.unwrap() + 1,
            &owner1.contract,
            minter.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "SenderNotAdmin")]
    async fn when_minter_does_not_have_access() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        let minter = Identity::Address(owner2.wallet.address().into());
        let admin = Some(minter.clone());
        constructor(admin.clone(), &deploy_wallet.contract, Some(1)).await;

        mint(1, &owner1.contract, minter.clone()).await;
    }
}
