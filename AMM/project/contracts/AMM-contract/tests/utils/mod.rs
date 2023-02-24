<<<<<<< HEAD
use fuels::prelude::*;
=======
use fuels::prelude::{AssetId, ContractId, WalletUnlocked};
>>>>>>> origin/master
use test_utils::{
    data_structures::WalletAssetConfiguration,
    interface::AMM,
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
<<<<<<< HEAD

    (wallet, amm.instance, asset_pairs)
=======

    (wallet, amm.instance, asset_pairs)
}

pub fn ordered_pair(pair: (AssetId, AssetId)) -> (ContractId, ContractId) {
    if pair.0 < pair.1 {
        (ContractId::new(*pair.0), ContractId::new(*pair.1))
    } else {
        (ContractId::new(*pair.1), ContractId::new(*pair.0))
    }
>>>>>>> origin/master
}
