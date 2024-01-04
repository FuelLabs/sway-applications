use crate::utils::setup::F_NFT;
use fuels::{
    prelude::{AssetId, CallParameters, TxPolicies, WalletUnlocked, BASE_ASSET_ID},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{Bits256, Identity},
};

pub(crate) async fn start_buyback(
    contract: &F_NFT<WalletUnlocked>,
    nft: AssetId, 
    token_buyback_price: u64,
    f_nft_supply: u64,
) -> FuelCallResponse<()> {
    let call_params = CallParameters::new((f_nft_supply * token_buyback_price) - , BASE_ASSET_ID, 1_000_000);

    contract
        .methods()
        .start_buyback(nft, token_buyback_price)
        //.with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call_params(call_params)
        .call()
        .await
        .unwrap()
}
