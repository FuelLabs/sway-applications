#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod utils;

use fuels::{signers::Signer, tx::{AssetId, ContractId}, prelude::{CallParameters, TxParameters}};

use utils::{
    abi_calls::{
        accept_arbiter, create_escrow, deposit, dispute, propose_arbiter, resolve_dispute, 
        return_deposit, take_payment, transfer_to_seller
    },
    test_helpers::{create_arbiter, create_asset, create_asset_with_salt, mint, setup},
    Identity,
};

mod accept_arbiter {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn accepts_proposal() {
            // TODO: when getters are implemented this should change the arbiter and check that it
            // has actually changed
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
            accept_arbiter(&buyer.contract, 0).await;
        }

        #[tokio::test]
        async fn accepts_proposal_in_two_escrows() {
            // TODO: when getters are implemented this should change the arbiter and check that it
            // has actually changed
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 4).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;
            propose_arbiter(&seller.contract, arbiter_obj, 1).await;
            accept_arbiter(&buyer.contract, 0).await;
            accept_arbiter(&buyer.contract, 1).await;
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            transfer_to_seller(&buyer.contract, 0).await;
            accept_arbiter(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            accept_arbiter(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_proposal_is_not_set() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            accept_arbiter(&buyer.contract, 0).await;
        }
        
    }

}

mod create_escrow {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn creates_escrow() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        async fn creates_two_escrow() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_assets_are_not_specified() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            // TODO: this test likely fails because the param expects an ARRAY of 2 and we provide 0
            //       args. This is likely a panic because of the SDK rather than the test itself
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_not_in_the_future() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_fee_is_zero() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, 0).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deposit_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount-1, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_asset_used_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), ContractId::from([2u8; 32]), defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(buyer.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(seller.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_asset_amount_is_zero() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(0, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }
        
    }

}

mod deposit {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn deposits() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            assert_eq!(
                defaults.asset_amount,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn deposits_to_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            assert_eq!(
                defaults.asset_amount * 2,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(
                defaults.asset_amount,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 1).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_reached() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), 5).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_depositing_more_than_once() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_incorrect_asset_amount_is_sent() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount-1, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_incorrect_asset_is_sent() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            let (id, salted_asset) = create_asset_with_salt([1u8; 32], buyer.wallet.clone()).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&salted_asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &ContractId::from(*id), &buyer.contract, 0).await;
        }
        
    }

}

mod dispute {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn disputes() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
        }

        #[tokio::test]
        async fn disputes_in_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 1).await;

            dispute(&buyer.contract, 0).await;
            dispute(&buyer.contract, 1).await;
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_disputing_more_than_once() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            dispute(&buyer.contract, 0).await;
        }
        
    }

}

mod propose_arbiter {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn proposes_arbiter() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(
                defaults.asset_amount,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn proposes_arbiter_twice() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 3).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;

            assert_eq!(
                defaults.asset_amount,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;

            assert_eq!(
                defaults.asset_amount,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn proposes_arbiter_in_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 4).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;

            assert_eq!(
                defaults.asset_amount,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj, 1).await;

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn proposes_arbiter_in_two_escrows_twice() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 6).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(
                defaults.asset_amount * 4,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;

            assert_eq!(
                defaults.asset_amount * 3,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 1).await;

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            propose_arbiter(&seller.contract, arbiter_obj, 1).await;

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            transfer_to_seller(&buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            propose_arbiter(&buyer.contract, arbiter_obj, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            let arbiter_obj = create_arbiter(buyer.wallet.address(), defaults.asset_id, defaults.asset_amount).await;

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            let arbiter_obj = create_arbiter(seller.wallet.address(), defaults.asset_id, defaults.asset_amount).await;

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_fee_is_zero() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            let arbiter_obj = create_arbiter(seller.wallet.address(), defaults.asset_id, 0).await;

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deposit_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(Some(arbiter_obj.fee_amount-1), Some(AssetId::from(*arbiter_obj.asset)), Some(100_000));

            seller.contract.propose_arbiter(arbiter_obj, 0).tx_params(tx_params).call_params(call_params).append_variable_outputs(1).call().await.unwrap();
        }

        #[tokio::test]
        #[should_panic]
        async fn when_asset_used_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
            let (id, salted_asset) = create_asset_with_salt([1u8; 32], buyer.wallet.clone()).await;

            mint(&salted_asset, seller.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(Some(arbiter_obj.fee_amount), Some(AssetId::from(*id)), Some(100_000));

            seller.contract.propose_arbiter(arbiter_obj, 0).tx_params(tx_params).call_params(call_params).append_variable_outputs(1).call().await.unwrap();
        }
        
    }

}

mod resolve_dispute {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn resolves_in_buyers_favour() {}

        #[tokio::test]
        #[ignore]
        async fn resolves_in_sellers_favour() {}

        #[tokio::test]
        #[ignore]
        async fn resolves_in_two_escrows() {}

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_not_disputed() {}

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_caller_is_not_arbiter() {}

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_user_is_not_buyer_or_seller() {}

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {}

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_payment_amount_is_too_large() {}
        
    }

}

mod return_deposit {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_deposit() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            return_deposit(&seller.contract, 0).await;

            assert_eq!(
                defaults.asset_amount,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn returns_deposit_after_proposing_arbiter() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            return_deposit(&seller.contract, 0).await;

            assert_eq!(
                defaults.asset_amount,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn returns_deposit_in_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 1).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            return_deposit(&seller.contract, 0).await;

            assert_eq!(
                defaults.asset_amount,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            return_deposit(&seller.contract, 1).await;

            assert_eq!(
                defaults.asset_amount * 2,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            return_deposit(&seller.contract, 0).await;
            return_deposit(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            return_deposit(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            return_deposit(&seller.contract, 0).await;
        }
        
    }

}

mod take_payment {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn takes_payment() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), 6).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            // TODO: need to shift block by one, waiting on SDK then uncomment below

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            // take_payment(&seller.contract, 0).await;

            // assert_eq!(
            //     defaults.asset_amount,
            //     seller
            //         .wallet
            //         .get_asset_balance(&AssetId::from(*defaults.asset_id))
            //         .await
            //         .unwrap()
            // );

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn takes_payment_after_proposing_arbiter() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), 6).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            // This should really be above `deposit` but given SDK limitations for block manipulation
            // we put this here
            propose_arbiter(&seller.contract, arbiter_obj, 0).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            take_payment(&seller.contract, 0).await;

            assert_eq!(
                defaults.asset_amount * 3,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        #[ignore]
        async fn takes_payment_in_two_escrows() {
            // TODO: skipping similar to takes_payment
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            return_deposit(&seller.contract, 0).await;

            take_payment(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_not_in_the_past() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            take_payment(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_disputed() {
            // TODO: skipping similar to takes_payment
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_caller_is_not_seller() {
            // TODO: skipping similar to takes_payment
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            // TODO: skipping similar to takes_payment
        }
        
    }

}

mod transfer_to_seller {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn transfers_to_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            transfer_to_seller(&buyer.contract, 0).await;

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn transfers_to_seller_after_proposing_arbiter() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj, 0).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            transfer_to_seller(&buyer.contract, 0).await;

            assert_eq!(
                defaults.asset_amount * 3,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn transfers_to_seller_in_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 1).await;

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            transfer_to_seller(&buyer.contract, 0).await;

            assert_eq!(
                defaults.asset_amount * 2,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            transfer_to_seller(&buyer.contract, 1).await;

            assert_eq!(
                defaults.asset_amount * 4,
                seller
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );

            assert_eq!(
                0,
                buyer
                    .wallet
                    .get_asset_balance(&AssetId::from(*defaults.asset_id))
                    .await
                    .unwrap()
            );
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            transfer_to_seller(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&seller.contract, 0).await;
        }
        
    }

}
