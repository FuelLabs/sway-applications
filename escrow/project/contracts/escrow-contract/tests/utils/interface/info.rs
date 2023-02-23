use crate::utils::setup::{Arbiter, Asset, Escrow, EscrowInfo};

pub(crate) async fn arbiter_proposal(contract: &Escrow, identifier: u64) -> Option<Arbiter> {
    contract
        .methods()
        .arbiter_proposal(identifier)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn assets(contract: &Escrow, identifier: u64) -> Option<Asset> {
    contract
        .methods()
        .assets(identifier)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn escrows(contract: &Escrow, identifier: u64) -> Option<EscrowInfo> {
    contract
        .methods()
        .escrows(identifier)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn escrow_count(contract: &Escrow) -> u64 {
    contract
        .methods()
        .escrow_count()
        .call()
        .await
        .unwrap()
        .value
}
