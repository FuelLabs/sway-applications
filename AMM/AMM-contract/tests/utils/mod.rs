use fuels::prelude::{AssetId, WalletUnlocked};
use test_utils::{
    data_structures::WalletAssetConfiguration,
    interface::AMM,
    setup::common::{deploy_amm, deploy_and_initialize_amm, setup_wallet_and_provider},
};

pub async fn setup(
    initialize: bool,
) -> (WalletUnlocked, AMM<WalletUnlocked>, Vec<(AssetId, AssetId)>) {
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

pub fn ordered_pair(pair: (AssetId, AssetId)) -> (AssetId, AssetId) {
    if pair.0 < pair.1 {
        (pair.0, pair.1)
    } else {
        (pair.1, pair.0)
    }
}
