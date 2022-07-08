mod utils;

use utils::{
    abi_calls::{
        accept_arbiter, create_escrow, deposit, dispute, propose_arbiter, resolve_dispute, 
        return_deposit, take_payment, transfer_to_seller
    },
    test_helpers::{setup},
};

mod accept_arbiter {

    use super::*;

    mod success {

        #[tokio::test]
        async fn accepts_proposal() {}

        #[tokio::test]
        async fn accepts_proposal_in_two_escrows() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {}

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_proposal_is_not_set() {}
        
    }

}

mod create_escrow {

    use super::*;

    mod success {

        #[tokio::test]
        async fn creates_escrow() {}

        #[tokio::test]
        async fn creates_two_escrow() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_assets_are_not_specified() {}

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_not_in_the_future() {}

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_fee_is_zero() {}

        #[tokio::test]
        #[should_panic]
        async fn when_deposit_for_arbiter_fee_is_unequal() {}

        #[tokio::test]
        #[should_panic]
        async fn when_asset_used_for_arbiter_fee_is_unequal() {}

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_buyer() {}

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_seller() {}

        #[tokio::test]
        #[should_panic]
        async fn when_asset_amount_is_zero() {}
        
    }

}

mod deposit {

    use super::*;

    mod success {

        #[tokio::test]
        async fn deposits() {}

        #[tokio::test]
        async fn deposits_to_two_escrows() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_reached() {}

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {}

        #[tokio::test]
        #[should_panic]
        async fn when_depositing_more_than_once() {}

        #[tokio::test]
        #[should_panic]
        async fn when_incorrect_asset_amount_is_sent() {}

        #[tokio::test]
        #[should_panic]
        async fn when_incorrect_asset_is_sent() {}
        
    }

}

mod dispute {

    use super::*;

    mod success {

        #[tokio::test]
        async fn disputes() {}

        #[tokio::test]
        async fn disputes_in_two_escrows() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_disputing_more_than_once() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {}

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {}
        
    }

}

mod propose_arbiter {

    use super::*;

    mod success {

        #[tokio::test]
        async fn proposes_arbiter() {}

        #[tokio::test]
        async fn proposes_arbiter_twice() {}

        #[tokio::test]
        async fn proposes_arbiter_in_two_escrows() {}

        #[tokio::test]
        async fn proposes_arbiter_in_two_escrows_twice() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_seller() {}

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_buyer() {}

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_address_is_set_to_seller() {}

        #[tokio::test]
        #[should_panic]
        async fn when_arbiter_fee_is_zero() {}

        #[tokio::test]
        #[should_panic]
        async fn when_deposit_for_arbiter_fee_is_unequal() {}

        #[tokio::test]
        #[should_panic]
        async fn when_asset_used_for_arbiter_fee_is_unequal() {}
        
    }

}

mod resolve_dispute {

    use super::*;

    mod success {

        #[tokio::test]
        async fn resolves_in_buyers_favour() {}

        #[tokio::test]
        async fn resolves_in_sellers_favour() {}

        #[tokio::test]
        async fn resolves_in_two_escrows() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_not_disputed() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_arbiter() {}

        #[tokio::test]
        #[should_panic]
        async fn when_user_is_not_buyer_or_seller() {}

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {}

        #[tokio::test]
        #[should_panic]
        async fn when_payment_amount_is_too_large() {}
        
    }

}

mod return_deposit {

    use super::*;

    mod success {

        #[tokio::test]
        async fn returns_deposit() {}

        #[tokio::test]
        async fn returns_deposit_in_two_escrows() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_seller() {}

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {}
        
    }

}

mod take_payment {

    use super::*;

    mod success {

        #[tokio::test]
        async fn takes_payment() {}

        #[tokio::test]
        async fn takes_payment_in_two_escrows() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_deadline_is_not_in_the_past() {}

        #[tokio::test]
        #[should_panic]
        async fn when_disputed() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_seller() {}

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {}
        
    }

}

mod transfer_to_seller {

    use super::*;

    mod success {

        #[tokio::test]
        async fn transfers_to_seller() {}

        #[tokio::test]
        async fn transfers_to_seller_in_two_escrows() {}

    }

    mod revert {

        #[tokio::test]
        #[should_panic]
        async fn when_escrow_is_not_pending() {}

        #[tokio::test]
        #[should_panic]
        async fn when_buyer_has_not_deposited() {}

        #[tokio::test]
        #[should_panic]
        async fn when_caller_is_not_buyer() {}
        
    }

}

// async fn init(
//     deployer: &Metadata,
//     user1: &LocalWallet,
//     user2: &LocalWallet,
//     asset_id: ContractId,
//     asset_amount: u64,
// ) -> bool {
//     deployer
//         .escrow
//         .constructor(user1.address(), user2.address(), asset_id, asset_amount)
//         .call()
//         .await
//         .unwrap()
//         .value
// }

// async fn mint(deployer: &Metadata, user: &LocalWallet, asset_amount: u64) {
//     deployer
//         .asset
//         .as_ref()
//         .unwrap()
//         .mint_and_send_to_address(asset_amount, user.address())
//         .append_variable_outputs(1)
//         .call()
//         .await
//         .unwrap()
//         .value;
// }

// async fn balance(escrow: &Escrow) -> (MetaAsset, MetaAsset) {
//     escrow.get_balance().call().await.unwrap().value
// }

// async fn user_data(escrow: &Escrow, user: &LocalWallet) -> (bool, bool) {
//     escrow
//         .get_user_data(user.address())
//         .call()
//         .await
//         .unwrap()
//         .value
// }

// mod constructor {

//     use super::*;

//     #[tokio::test]
//     async fn initializes() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let users = [user1.wallet.address(), user1.wallet.address()];
//         let assets = [
//             MetaAsset {
//                 id: [1u8; 32],
//                 amount: 100,
//             },
//             MetaAsset {
//                 id: [2u8; 32],
//                 amount: 200,
//             },
//         ];

//         assert!(
//             deployer
//                 .escrow
//                 .constructor(users, assets)
//                 .call()
//                 .await
//                 .unwrap()
//                 .value
//         )

//         // assert!(
//         //     init(
//         //         &deployer,
//         //         &user1.wallet,
//         //         &user2.wallet,
//         //         asset_id,
//         //         asset_amount
//         //     )
//         //     .await
//         // );
//     }
// }

// mod deposit {

//     use super::*;

//     #[tokio::test]
//     async fn deposits() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;

//         assert_eq!(balance(&deployer.escrow).await, 0);
//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (false, false)
//         );

//         // Test
//         assert!(
//             user1
//                 .escrow
//                 .deposit()
//                 .tx_params(tx_params)
//                 .call_params(call_params)
//                 .call()
//                 .await
//                 .unwrap()
//                 .value
//         );

//         assert_eq!(balance(&deployer.escrow).await, asset_amount);
//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (true, false)
//         );
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_not_initialized() {
//         let (_, user1, _, _, _) = setup().await;

//         // Should panic
//         user1.escrow.deposit().call().await.unwrap();
//     }

//     // Uncomment when https://github.com/FuelLabs/fuels-rs/pull/305 (deploy_with_salt) lands in a new release
//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_with_incorrect_asset() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let another_asset_id = Contract::deploy_with_salt(
//             "./tests/artifacts/asset/out/debug/asset.bin",
//             &deployer.wallet,
//             TxParameters::default(),
//             Salt::from([1u8; 32]),
//         )
//         .await
//         .unwrap();

//         let another_asset = MyAsset::new(another_asset_id.to_string(), deployer.wallet.clone());

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params =
//             CallParameters::new(Some(asset_amount), Some(AssetId::from(*another_asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         another_asset
//             .mint_and_send_to_address(asset_amount, user1.wallet.address())
//             .append_variable_outputs(1)
//             .call()
//             .await
//             .unwrap();

//         // Should panic
//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params)
//             .call_params(call_params)
//             .call()
//             .await
//             .unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_with_incorrect_asset_amount() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params =
//             CallParameters::new(Some(asset_amount - 1), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;

//         // Should panic
//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params)
//             .call_params(call_params)
//             .call()
//             .await
//             .unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_sender_is_not_the_correct_address() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &deployer.wallet, asset_amount).await;

//         // Should panic
//         deployer
//             .escrow
//             .deposit()
//             .tx_params(tx_params)
//             .call_params(call_params)
//             .call()
//             .await
//             .unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_already_deposited() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

//         let call_params1 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params2 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, 2 * asset_amount).await;

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params1)
//             .call_params(call_params1)
//             .call()
//             .await
//             .unwrap();

//         // Should panic
//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params2)
//             .call_params(call_params2)
//             .call()
//             .await
//             .unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_after_both_parties_approve() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params3 = TxParameters::new(None, Some(1_000_000), None, None);

//         let call_params1 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params2 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params3 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;
//         mint(&deployer, &user2.wallet, asset_amount).await;

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params1)
//             .call_params(call_params1)
//             .call()
//             .await
//             .unwrap();
//         user2
//             .escrow
//             .deposit()
//             .tx_params(tx_params2)
//             .call_params(call_params2)
//             .call()
//             .await
//             .unwrap();

//         user1.escrow.approve().call().await.unwrap();
//         user2
//             .escrow
//             .approve()
//             .append_variable_outputs(2)
//             .call()
//             .await
//             .unwrap();

//         // Should panic
//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params3)
//             .call_params(call_params3)
//             .call()
//             .await
//             .unwrap();
//     }
// }

// mod approve {

//     use super::*;

//     #[tokio::test]
//     async fn approves() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

//         let call_params1 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params2 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;
//         mint(&deployer, &user2.wallet, asset_amount).await;

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params1)
//             .call_params(call_params1)
//             .call()
//             .await
//             .unwrap();
//         user2
//             .escrow
//             .deposit()
//             .tx_params(tx_params2)
//             .call_params(call_params2)
//             .call()
//             .await
//             .unwrap();

//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (true, false)
//         );
//         assert_eq!(
//             user_data(&deployer.escrow, &user2.wallet).await,
//             (true, false)
//         );
//         assert_eq!(balance(&deployer.escrow).await, 2 * asset_amount);

//         // Test
//         assert!(user1.escrow.approve().call().await.unwrap().value);
//         assert!(
//             user2
//                 .escrow
//                 .approve()
//                 .append_variable_outputs(2)
//                 .call()
//                 .await
//                 .unwrap()
//                 .value
//         );

//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (true, true)
//         );
//         assert_eq!(
//             user_data(&deployer.escrow, &user2.wallet).await,
//             (true, true)
//         );
//         assert_eq!(balance(&deployer.escrow).await, 0);
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_not_initialized() {
//         let (_, user1, _, _, _) = setup().await;

//         // Should panic
//         user1.escrow.approve().call().await.unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_sender_is_not_the_correct_address() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         // Should panic
//         deployer.escrow.approve().call().await.unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_not_deposited() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         // Should panic
//         user1.escrow.approve().call().await.unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_after_both_parties_approve() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

//         let call_params1 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params2 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;
//         mint(&deployer, &user2.wallet, asset_amount).await;

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params1)
//             .call_params(call_params1)
//             .call()
//             .await
//             .unwrap();
//         user2
//             .escrow
//             .deposit()
//             .tx_params(tx_params2)
//             .call_params(call_params2)
//             .call()
//             .await
//             .unwrap();

//         user1.escrow.approve().call().await.unwrap();
//         user2
//             .escrow
//             .approve()
//             .append_variable_outputs(2)
//             .call()
//             .await
//             .unwrap();

//         // Should panic
//         user1.escrow.approve().call().await.unwrap();
//     }
// }

// mod withdraw {

//     use super::*;

//     #[tokio::test]
//     async fn withdraws() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params)
//             .call_params(call_params)
//             .call()
//             .await
//             .unwrap();

//         user1.escrow.approve().call().await.unwrap();

//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (true, true)
//         );
//         assert_eq!(balance(&deployer.escrow).await, asset_amount);

//         // Test
//         assert!(
//             user1
//                 .escrow
//                 .withdraw()
//                 .append_variable_outputs(1)
//                 .call()
//                 .await
//                 .unwrap()
//                 .value
//         );

//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (false, false)
//         );
//         assert_eq!(balance(&deployer.escrow).await, 0);
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_not_initialized() {
//         let (_, user1, _, _, _) = setup().await;

//         // Should panic
//         user1.escrow.withdraw().call().await.unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_sender_is_not_the_correct_address() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         // Should panic
//         deployer.escrow.withdraw().call().await.unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_not_deposited() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         // Should panic
//         user1.escrow.withdraw().call().await.unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_after_both_parties_approve() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

//         let call_params1 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params2 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;
//         mint(&deployer, &user2.wallet, asset_amount).await;

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params1)
//             .call_params(call_params1)
//             .call()
//             .await
//             .unwrap();
//         user2
//             .escrow
//             .deposit()
//             .tx_params(tx_params2)
//             .call_params(call_params2)
//             .call()
//             .await
//             .unwrap();

//         user1.escrow.approve().call().await.unwrap();
//         user2
//             .escrow
//             .approve()
//             .append_variable_outputs(2)
//             .call()
//             .await
//             .unwrap();

//         // Should panic
//         user1
//             .escrow
//             .withdraw()
//             .append_variable_outputs(1)
//             .call()
//             .await
//             .unwrap();
//     }
// }

// mod get_balance {

//     use super::*;

//     #[tokio::test]
//     async fn returns_zero() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         assert_eq!(balance(&deployer.escrow).await, 0);
//     }

//     #[tokio::test]
//     async fn returns_asset_amount() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params)
//             .call_params(call_params)
//             .call()
//             .await
//             .unwrap();

//         assert_eq!(balance(&deployer.escrow).await, asset_amount);
//     }
// }

// mod get_user_data {

//     use super::*;

//     #[tokio::test]
//     async fn gets_user_data() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

//         let call_params1 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params2 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;
//         mint(&deployer, &user2.wallet, asset_amount).await;

//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (false, false)
//         );
//         assert_eq!(
//             user_data(&deployer.escrow, &user2.wallet).await,
//             (false, false)
//         );

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params1)
//             .call_params(call_params1)
//             .call()
//             .await
//             .unwrap();
//         user2
//             .escrow
//             .deposit()
//             .tx_params(tx_params2)
//             .call_params(call_params2)
//             .call()
//             .await
//             .unwrap();

//         user1.escrow.approve().call().await.unwrap();
//         user2
//             .escrow
//             .approve()
//             .append_variable_outputs(2)
//             .call()
//             .await
//             .unwrap();

//         assert_eq!(
//             user_data(&deployer.escrow, &user1.wallet).await,
//             (true, true)
//         );
//         assert_eq!(
//             user_data(&deployer.escrow, &user2.wallet).await,
//             (true, true)
//         );
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_not_initialized() {
//         let (_, user1, _, _, _) = setup().await;

//         // Should panic
//         user1
//             .escrow
//             .get_user_data(user1.wallet.address())
//             .call()
//             .await
//             .unwrap();
//     }

//     #[tokio::test]
//     #[should_panic(expected = "Revert(42)")]
//     async fn panics_when_sender_is_not_the_correct_address() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         // Should panic
//         user1
//             .escrow
//             .get_user_data(deployer.wallet.address())
//             .call()
//             .await
//             .unwrap();
//     }
// }

// mod get_state {

//     use super::*;

//     #[tokio::test]
//     async fn not_initialized() {
//         let (deployer, _, _, _, _) = setup().await;

//         assert_eq!(deployer.escrow.get_state().call().await.unwrap().value, 0);
//     }

//     #[tokio::test]
//     async fn initialized() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         // Init conditions
//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;

//         assert_eq!(deployer.escrow.get_state().call().await.unwrap().value, 1);
//     }

//     #[tokio::test]
//     async fn completed() {
//         let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

//         let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
//         let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

//         let call_params1 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));
//         let call_params2 = CallParameters::new(Some(asset_amount), Some(AssetId::from(*asset_id)));

//         // Init conditions
//         assert_eq!(deployer.escrow.get_state().call().await.unwrap().value, 0);

//         init(
//             &deployer,
//             &user1.wallet,
//             &user2.wallet,
//             asset_id,
//             asset_amount,
//         )
//         .await;
//         mint(&deployer, &user1.wallet, asset_amount).await;
//         mint(&deployer, &user2.wallet, asset_amount).await;

//         assert_eq!(deployer.escrow.get_state().call().await.unwrap().value, 1);

//         user1
//             .escrow
//             .deposit()
//             .tx_params(tx_params1)
//             .call_params(call_params1)
//             .call()
//             .await
//             .unwrap();
//         user2
//             .escrow
//             .deposit()
//             .tx_params(tx_params2)
//             .call_params(call_params2)
//             .call()
//             .await
//             .unwrap();

//         // Test
//         user1.escrow.approve().call().await.unwrap();
//         user2
//             .escrow
//             .approve()
//             .append_variable_outputs(2)
//             .call()
//             .await
//             .unwrap();

//         assert_eq!(deployer.escrow.get_state().call().await.unwrap().value, 2);
//     }
// }
