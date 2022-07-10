use fuels::prelude::*;

pub async fn get_balance(provider: &Provider, address: Address, asset: AssetId) -> u64 {
    let balance = provider.get_asset_balance(&address, asset).await.unwrap();
    balance
}
