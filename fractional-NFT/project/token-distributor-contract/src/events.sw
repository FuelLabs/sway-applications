library events;

dep data_structures;

use data_structures::TokenDistribution;

pub struct Canceled {
    fractional_nft: ContractId,
}

pub struct Closed {
    fractional_nft: ContractId,
}

pub struct Created {
    fractional_nft: ContractId,
    token_distribution: TokenDistribution,
}

pub struct Purchased {
    amount: u64,
    buyer: Identity,
    fractional_nft: ContractId,
}

pub struct PurchasedReserve {
    fractional_nft: ContractId,
    owner: Option<Identity>,
    reserve: Option<u64>,
}

pub struct RequestedReturn {
    fractional_nft: ContractId,
    token_price: u64,
}

pub struct Sell {
    fractional_nft: ContractId,
    seller: Identity,
}

pub struct Withdraw {
    amount: u64,
    external_asset: ContractId,
    fractional_nft: ContractId,
}
