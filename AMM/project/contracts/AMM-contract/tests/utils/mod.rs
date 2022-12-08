use fuels::prelude::*;
use test_utils::{
    abi::AMM,
    data_structures::WalletAssetConfiguration,
    setup::common::{deploy_amm, deploy_and_initialize_amm, setup_wallet_and_provider},
};

pub async fn setup(initialize: bool) -> (WalletUnlocked, AMM, Vec<(AssetId, AssetId)>) {
    let (wallet, asset_ids, _provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let amm = if initialize {
        deploy_and_initialize_amm(&wallet).await
    } else {
        deploy_amm(&wallet).await
    };

    // setup two asset pairs that will be used in tests
    let asset_pairs = vec![(asset_ids[0], asset_ids[1]), (asset_ids[1], asset_ids[2])];

    (wallet, amm.instance, asset_pairs)
}
