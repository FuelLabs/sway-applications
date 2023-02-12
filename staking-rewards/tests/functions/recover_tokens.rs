// #[tokio::test]
// async fn can_recover_tokens() {
//     let (staking_contract, id, wallet, wallet2, _inittimestamp) = setup().await;

//     let _receipt = wallet2
//         .force_transfer_to_contract(&id, 50000, RANDOM_ASSET, TxParameters::default())
//         .await
//         .unwrap();

//     let owner_balance_before = get_balance(&wallet, RANDOM_ASSET).await;

//     recover_tokens(&staking_contract, ContractId::new([3u8; 32]), 50000).await;

//     let owner_balance_after = get_balance(&wallet, RANDOM_ASSET).await;

//     assert_eq!(owner_balance_before + 50000, owner_balance_after);
// }
use crate::utils::{
    abi::recover_tokens,
    setup, get_balance, RANDOM_ASSET
};

use fuels::{prelude::TxParameters, types::ContractId};

#[tokio::test]
async fn can_recover_tokens() {
    let (staking_contract, id, wallet, wallet2, _inittimestamp) = setup().await;

    let _receipt = wallet2
        .force_transfer_to_contract(&id, 50000, RANDOM_ASSET, TxParameters::default())
        .await
        .unwrap();

    let owner_balance_before = get_balance(&wallet, RANDOM_ASSET).await;

    recover_tokens(&staking_contract, ContractId::new([3u8; 32]), 50000).await;

    let owner_balance_after = get_balance(&wallet, RANDOM_ASSET).await;

    assert_eq!(owner_balance_before + 50000, owner_balance_after);
}