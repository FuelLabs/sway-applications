use crate::utils::setup::EnglishAuction;
use fuels::{
    prelude::{AssetId, CallParameters, TxPolicies, WalletUnlocked},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::Identity,
};

pub(crate) mod auction {
    use super::*;

    pub(crate) async fn bid(
        auction_id: u64,
        bid_asset: AssetId,
        bid_amount: u64,
        contract: &EnglishAuction<WalletUnlocked>,
    ) -> FuelCallResponse<()> {
        let call_params = CallParameters::new(bid_amount, bid_asset, 1_000_000);

        contract
            .methods()
            .bid(auction_id)
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn cancel(
        auction_id: u64,
        contract: &EnglishAuction<WalletUnlocked>,
    ) -> FuelCallResponse<()> {
        contract.methods().cancel(auction_id).call().await.unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn create(
        bid_asset: AssetId,
        contract: &EnglishAuction<WalletUnlocked>,
        duration: u32,
        initial_price: u64,
        reserve_price: Option<u64>,
        seller: Identity,
        sell_asset: AssetId,
        sell_amount: u64,
    ) -> u64 {
        let call_params = CallParameters::new(sell_amount, sell_asset, 1_000_000);

        contract
            .methods()
            .create(bid_asset, duration, initial_price, reserve_price, seller)
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn withdraw(
        auction_id: u64,
        contract: &EnglishAuction<WalletUnlocked>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(auction_id)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}
