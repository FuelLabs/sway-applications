use fuels::prelude::*;
use fuels::tx::{AssetId, ContractId, Salt};
use fuels_abigen_macro::abigen;

abigen!(Escrow, "out/debug/escrow-abi.json");
abigen!(MyAsset, "tests/artifacts/asset/out/debug/asset-abi.json");

struct MetaAsset {
    amount: u64,
    id: [u8; 32],
}

struct Metadata {
    escrow: Escrow,
    asset: Option<MyAsset>,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata, Metadata, ContractId, u64) {
    let num_wallets = 3;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_provider_and_get_wallets(config).await;

    let deployer_wallet = wallets.pop().unwrap();
    let user1_wallet = wallets.pop().unwrap();
    let user2_wallet = wallets.pop().unwrap();

    let escrow_id = Contract::deploy(
        "./out/debug/escrow.bin",
        &deployer_wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let asset_id = Contract::deploy(
        "./tests/artifacts/asset/out/debug/asset.bin",
        &deployer_wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let deployer = Metadata {
        escrow: Escrow::new(escrow_id.to_string(), deployer_wallet.clone()),
        asset: Some(MyAsset::new(asset_id.to_string(), deployer_wallet.clone())),
        wallet: deployer_wallet,
    };

    let user1 = Metadata {
        escrow: Escrow::new(escrow_id.to_string(), user1_wallet.clone()),
        asset: None,
        wallet: user1_wallet,
    };

    let user2 = Metadata {
        escrow: Escrow::new(escrow_id.to_string(), user2_wallet.clone()),
        asset: None,
        wallet: user2_wallet,
    };

    let asset_amount: u64 = 100;

    (deployer, user1, user2, asset_id, asset_amount)
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

mod constructor {

    use super::*;

    #[tokio::test]
    async fn initializes() {
        let (deployer, user1, user2, asset_id, asset_amount) = setup().await;

        let users = [user1.wallet.address(), user1.wallet.address()];
        let assets = [
            MetaAsset {
                id: [1u8; 32],
                amount: 100,
            },
            MetaAsset {
                id: [2u8; 32],
                amount: 200,
            },
        ];

        assert!(
            deployer
                .escrow
                .constructor(users, assets)
                .call()
                .await
                .unwrap()
                .value
        )

        // assert!(
        //     init(
        //         &deployer,
        //         &user1.wallet,
        //         &user2.wallet,
        //         asset_id,
        //         asset_amount
        //     )
        //     .await
        // );
    }
}

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
