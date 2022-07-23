mod utils;

use fuels::{signers::Signer, tx::{AssetId, ContractId}, prelude::{CallParameters, TxParameters}};

use utils::{
    abi_calls::{
        accept_arbiter, create_escrow, deposit, dispute, propose_arbiter, resolve_dispute, 
        return_deposit, take_payment, transfer_to_seller
    },
    Identity,
    test_helpers::{asset_amount, create_arbiter, create_asset, create_asset_with_salt, mint, setup},
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
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            accept_arbiter(&buyer.contract, 0).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn accepts_proposal_in_two_escrows() {
            // TODO: when getters are implemented this should change the arbiter and check that it
            // has actually changed
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 4).await;
            
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj, 1).await;
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            accept_arbiter(&buyer.contract, 0).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);

            accept_arbiter(&buyer.contract, 1).await;
            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            accept_arbiter(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            accept_arbiter(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_proposal_is_not_set() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

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
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
            
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn creates_two_escrow() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_assets_are_not_specified() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            // TODO: this test likely fails because the param expects an ARRAY of 2 and we provide 0
            //       args. This is likely a panic because of the SDK rather than the test itself
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_not_in_the_future() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_fee_is_zero() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, 0).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;            

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deposit_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount-1, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_asset_used_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), ContractId::from([2u8; 32]), defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_buyer() {
            let (_, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(buyer.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_seller() {
            let (_, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(seller.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;            

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_asset_amount_is_zero() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(0, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;

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
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn deposits_to_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 1).await;
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_reached() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), 5).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_depositing_more_than_once() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_incorrect_asset_amount_is_sent() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount-1, &defaults.asset_id, &buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_incorrect_asset_is_sent() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
            let (id, salted_asset) = create_asset_with_salt([1u8; 32], buyer.wallet.clone()).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&salted_asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

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
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_disputing_more_than_once() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

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
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn proposes_arbiter_twice() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 3).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn proposes_arbiter_in_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 4).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;
            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj, 1).await;
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn proposes_arbiter_in_two_escrows_twice() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 6).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            assert_eq!(defaults.asset_amount * 4, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;
            assert_eq!(defaults.asset_amount * 3, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 1).await;
            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;
            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);

            propose_arbiter(&seller.contract, arbiter_obj, 1).await;
            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            propose_arbiter(&buyer.contract, arbiter_obj, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
            let arbiter_obj_buyer = create_arbiter(buyer.wallet.address(), defaults.asset_id, defaults.asset_amount).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj_buyer, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
            let arbiter_obj_seller = create_arbiter(seller.wallet.address(), defaults.asset_id, defaults.asset_amount).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj_seller, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_fee_is_zero() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
            let arbiter_obj_zero = create_arbiter(arbiter.wallet.address(), defaults.asset_id, 0).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj_zero, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deposit_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(Some(arbiter_obj.fee_amount-1), Some(AssetId::from(*arbiter_obj.asset)), Some(100_000));

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            seller.contract.propose_arbiter(arbiter_obj, 0).tx_params(tx_params).call_params(call_params).append_variable_outputs(1).call().await.unwrap();
        }

        #[tokio::test]
        #[should_panic]
        async fn when_asset_used_for_arbiter_fee_is_unequal() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
            let (id, salted_asset) = create_asset_with_salt([1u8; 32], buyer.wallet.clone()).await;
            
            let arbiter_obj_unequal = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
            let call_params = CallParameters::new(Some(arbiter_obj_unequal.fee_amount), Some(AssetId::from(*id)), Some(100_000));

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;
            mint(&salted_asset, seller.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;  

            seller.contract.propose_arbiter(arbiter_obj_unequal, 0).tx_params(tx_params).call_params(call_params).append_variable_outputs(1).call().await.unwrap();
        }
        
    }

}

mod resolve_dispute {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn resolves_in_buyers_favour_full_payment_taken() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;            

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&arbiter.wallet, &defaults.asset_id).await);

            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(buyer.wallet.address())).await;

            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&arbiter.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn resolves_in_buyers_favour_partial_payment_taken() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount - 1, Identity::Address(buyer.wallet.address())).await;

            assert_eq!(1, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount - 1, asset_amount(&arbiter.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn resolves_in_sellers_favour_full_payment_taken() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(seller.wallet.address())).await;

            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&arbiter.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn resolves_in_sellers_favour_partial_payment_taken() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount-1, Identity::Address(seller.wallet.address())).await;

            assert_eq!(defaults.asset_amount + 1, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount - 1, asset_amount(&arbiter.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn resolves_after_proposing_arbiter() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj.clone(), 0).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(buyer.wallet.address())).await;

            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&arbiter.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn resolves_in_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 1).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&arbiter.wallet, &defaults.asset_id).await);

            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(buyer.wallet.address())).await;

            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&arbiter.wallet, &defaults.asset_id).await);

            dispute(&buyer.contract, 1).await;
            resolve_dispute(&arbiter.contract, 1, arbiter_obj.fee_amount, Identity::Address(seller.wallet.address())).await;

            assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount * 2, asset_amount(&arbiter.wallet, &defaults.asset_id).await);
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            dispute(&buyer.contract, 0).await;

            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(buyer.wallet.address())).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(buyer.wallet.address())).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_not_disputed() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(buyer.wallet.address())).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_arbiter() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
            resolve_dispute(&buyer.contract, 0, arbiter_obj.fee_amount, Identity::Address(buyer.wallet.address())).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_user_is_not_buyer_or_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount, Identity::Address(arbiter.wallet.address())).await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            // Note: Buyer can only dispute after they deposit and we cannot get past the require 
            //       checks in resolve_dispute unless there is a dispute therefore this cannot 
            //       actually be tested however for clarity & completeness this has been left in
        }

        #[tokio::test]
        #[should_panic]
        async fn when_payment_amount_is_too_large() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            dispute(&buyer.contract, 0).await;
            resolve_dispute(&arbiter.contract, 0, arbiter_obj.fee_amount + 1, Identity::Address(buyer.wallet.address())).await;
        }
        
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

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);

            return_deposit(&seller.contract, 0).await;

            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);
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

            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);

            return_deposit(&seller.contract, 0).await;

            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);
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

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);

            return_deposit(&seller.contract, 0).await;

            assert_eq!(defaults.asset_amount, asset_amount(&buyer.wallet, &defaults.asset_id).await);

            return_deposit(&seller.contract, 1).await;

            assert_eq!(defaults.asset_amount * 2, asset_amount(&buyer.wallet, &defaults.asset_id).await);
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            return_deposit(&seller.contract, 0).await;
            return_deposit(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_seller() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            return_deposit(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

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

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            // take_payment(&seller.contract, 0).await;

            // assert_eq!(defaults.asset_amount, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
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

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            take_payment(&seller.contract, 0).await;

            assert_eq!(defaults.asset_amount * 3, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
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
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            return_deposit(&seller.contract, 0).await;
            take_payment(&seller.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_not_in_the_past() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

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
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            transfer_to_seller(&buyer.contract, 0).await;

            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn transfers_to_seller_after_proposing_arbiter() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            propose_arbiter(&seller.contract, arbiter_obj, 0).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            transfer_to_seller(&buyer.contract, 0).await;

            assert_eq!(defaults.asset_amount * 3, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
        }

        #[tokio::test]
        async fn transfers_to_seller_in_two_escrows() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount * 2).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount * 2).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;

            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 1).await;

            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&seller.wallet, &defaults.asset_id).await);

            transfer_to_seller(&buyer.contract, 0).await;

            assert_eq!(defaults.asset_amount * 2, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);

            transfer_to_seller(&buyer.contract, 1).await;

            assert_eq!(defaults.asset_amount * 4, asset_amount(&seller.wallet, &defaults.asset_id).await);
            assert_eq!(0, asset_amount(&buyer.wallet, &defaults.asset_id).await);
        }

    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
            transfer_to_seller(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            transfer_to_seller(&buyer.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {
            let (arbiter, buyer, seller, defaults) = setup().await;
            let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, defaults.asset_amount).await;
            let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

            mint(&defaults.asset, seller.wallet.address(), defaults.asset_amount).await;
            mint(&defaults.asset, buyer.wallet.address(), defaults.asset_amount).await;

            create_escrow(&seller.contract, defaults.asset_amount, &arbiter_obj, &defaults.asset_id, vec![asset.clone(), asset.clone()], Identity::Address(buyer.wallet.address()), defaults.deadline).await;
            deposit(defaults.asset_amount, &defaults.asset_id, &buyer.contract, 0).await;
            transfer_to_seller(&seller.contract, 0).await;
        }
        
    }

}
