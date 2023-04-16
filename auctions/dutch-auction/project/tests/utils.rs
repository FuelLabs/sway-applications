use fuels::{prelude::*, types::Identity};

// Load abi from json
abigen!(Contract(
    name = "DutchAuction",
    abi = "out/debug/dutch-auction-abi.json"
));

pub async fn get_contract_instance() -> (DutchAuction, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let config = Config {
        manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
        ..Config::local_node()
    };
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new_multiple_assets(
            2,
            vec![
                AssetConfig {
                    id: AssetId::new([0; 32]),
                    num_coins: 1,
                    coin_amount: 10000,
                },
                AssetConfig {
                    id: AssetId::new([1; 32]),
                    num_coins: 1,
                    coin_amount: 10000,
                },
            ],
        ),
        Some(config),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/dutch-auction.bin",
        &wallet,
        DeployConfiguration::default(),
    )
    .await
    .unwrap();

    (DutchAuction::new(id, wallet.clone()), wallet)
}

pub async fn active_auctions_of_author(instance: &DutchAuction, author: Identity) -> Vec<u64> {
    instance
        .methods()
        .active_auctions_of_author(author)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn auction(instance: &DutchAuction, auction_id: u64) -> Auction {
    instance
        .methods()
        .auction(auction_id)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn auctions_of_author(instance: &DutchAuction, author: Identity) -> Vec<u64> {
    instance
        .methods()
        .auctions_of_author(author)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn auctions_won(instance: &DutchAuction, bidder: Identity) -> Vec<u64> {
    instance
        .methods()
        .auctions_won(bidder)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn bid(instance: &DutchAuction, auction_id: u64, amount: u64) {
    instance
        .methods()
        .bid(auction_id)
        .call_params(
            CallParameters::default()
                .set_amount(amount)
                .set_asset_id(AssetId::BASE),
        )
        .unwrap()
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();
}

pub async fn cancel_auction(instance: &DutchAuction, auction_id: u64) {
    instance
        .methods()
        .cancel_auction(auction_id)
        .call()
        .await
        .unwrap();
}

pub async fn change_asset(instance: &DutchAuction, new_asset: ContractId, auction_id: u64) {
    instance
        .methods()
        .change_asset(new_asset, auction_id)
        .call()
        .await
        .unwrap();
}

pub async fn change_beneficiary(
    instance: &DutchAuction,
    new_beneficiary: Identity,
    auction_id: u64,
) {
    instance
        .methods()
        .change_beneficiary(new_beneficiary, auction_id)
        .call()
        .await
        .unwrap();
}

pub async fn create_auction(
    instance: &DutchAuction,
    opening_price: u64,
    reserve_price: u64,
    start_time: u64,
    end_time: u64,
    beneficiary: Identity,
    asset: ContractId,
) {
    instance
        .methods()
        .create_auction(
            opening_price,
            reserve_price,
            start_time,
            end_time,
            beneficiary,
            asset,
        )
        .call()
        .await
        .unwrap();
}

pub async fn price(instance: &DutchAuction, auction_id: u64) -> u64 {
    instance
        .methods()
        .price(auction_id)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn auction_count(instance: &DutchAuction) -> u64 {
    instance
        .methods()
        .auction_count()
        .call()
        .await
        .unwrap()
        .value
}
