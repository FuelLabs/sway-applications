use crate::utils::{
    interface::core::{create_escrow, deposit},
    setup::{create_arbiter, create_asset, mint, setup},
};
use fuels::tx::ContractId;

mod success {

    use super::*;
    use crate::utils::{
        interface::info::escrows,
        setup::{asset_amount, Buyer, DepositEvent, EscrowInfo, Seller, State},
    };
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn deposits() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;

        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            EscrowInfo {
                arbiter: arbiter_obj.clone(),
                asset_count: 2,
                buyer: Buyer {
                    address: Identity::Address(Address::from(buyer.wallet.address())),
                    asset: None,
                    deposited_amount: 0,
                },
                deadline: defaults.deadline,
                disputed: false,
                first_asset_index: 0,
                seller: Seller {
                    address: Identity::Address(Address::from(seller.wallet.address())),
                },
                state: State::Pending,
            }
        );

        let response = deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        let log = response.get_logs_with_type::<DepositEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            DepositEvent {
                asset: defaults.asset_id,
                identifier: 0
            }
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            EscrowInfo {
                arbiter: arbiter_obj,
                asset_count: 2,
                buyer: Buyer {
                    address: Identity::Address(Address::from(buyer.wallet.address())),
                    asset: Some(defaults.asset_id),
                    deposited_amount: defaults.asset_amount,
                },
                deadline: defaults.deadline,
                disputed: false,
                first_asset_index: 0,
                seller: Seller {
                    address: Identity::Address(Address::from(seller.wallet.address())),
                },
                state: State::Pending,
            }
        );
    }

    #[tokio::test]
    async fn deposits_to_two_escrows() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;

        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            EscrowInfo {
                arbiter: arbiter_obj.clone(),
                asset_count: 1,
                buyer: Buyer {
                    address: Identity::Address(Address::from(buyer.wallet.address())),
                    asset: None,
                    deposited_amount: 0,
                },
                deadline: defaults.deadline,
                disputed: false,
                first_asset_index: 0,
                seller: Seller {
                    address: Identity::Address(Address::from(seller.wallet.address())),
                },
                state: State::Pending,
            }
        );

        assert_eq!(
            escrows(&seller.contract, 1).await.unwrap(),
            EscrowInfo {
                arbiter: arbiter_obj.clone(),
                asset_count: 3,
                buyer: Buyer {
                    address: Identity::Address(Address::from(buyer.wallet.address())),
                    asset: None,
                    deposited_amount: 0,
                },
                deadline: defaults.deadline,
                disputed: false,
                first_asset_index: 1,
                seller: Seller {
                    address: Identity::Address(Address::from(seller.wallet.address())),
                },
                state: State::Pending,
            }
        );

        let response = deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        let log = response.get_logs_with_type::<DepositEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            DepositEvent {
                asset: defaults.asset_id,
                identifier: 0
            }
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            EscrowInfo {
                arbiter: arbiter_obj.clone(),
                asset_count: 1,
                buyer: Buyer {
                    address: Identity::Address(Address::from(buyer.wallet.address())),
                    asset: Some(defaults.asset_id),
                    deposited_amount: defaults.asset_amount,
                },
                deadline: defaults.deadline,
                disputed: false,
                first_asset_index: 0,
                seller: Seller {
                    address: Identity::Address(Address::from(seller.wallet.address())),
                },
                state: State::Pending,
            }
        );

        let response = deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            1,
        )
        .await;
        let log = response.get_logs_with_type::<DepositEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            DepositEvent {
                asset: defaults.asset_id,
                identifier: 1
            }
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(
            escrows(&seller.contract, 1).await.unwrap(),
            EscrowInfo {
                arbiter: arbiter_obj,
                asset_count: 3,
                buyer: Buyer {
                    address: Identity::Address(Address::from(buyer.wallet.address())),
                    asset: Some(defaults.asset_id),
                    deposited_amount: defaults.asset_amount,
                },
                deadline: defaults.deadline,
                disputed: false,
                first_asset_index: 1,
                seller: Seller {
                    address: Identity::Address(Address::from(seller.wallet.address())),
                },
                state: State::Pending,
            }
        );
    }
}

mod revert {

    use super::*;
    use crate::utils::{interface::core::transfer_to_seller, setup::create_asset_with_salt};

    #[tokio::test]
    #[should_panic(expected = "EscrowExpired")]
    async fn when_deadline_is_reached() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            6,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "StateNotPending")]
    async fn when_escrow_is_not_pending() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        transfer_to_seller(&buyer.contract, 0).await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_buyer() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &seller.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "AlreadyDeposited")]
    async fn when_depositing_more_than_once() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetAmount")]
    async fn when_incorrect_asset_amount_is_sent() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        deposit(
            defaults.asset_amount - 1,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetSent")]
    async fn when_incorrect_asset_is_sent() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
        let (id, salted_asset) = create_asset_with_salt([1u8; 32], buyer.wallet.clone()).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount * 2,
            &salted_asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &ContractId::from(*id),
            &buyer.contract,
            0,
        )
        .await;
    }
}
