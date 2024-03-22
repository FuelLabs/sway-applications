use crate::utils::setup::{Arbiter, Asset, EscrowInfo, User};

pub(crate) async fn arbiter_proposal(caller: &User, identifier: u64) -> Option<Arbiter> {
    caller
        .contract
        .methods()
        .arbiter_proposal(identifier)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn assets(caller: &User, identifier: u64) -> Option<Asset> {
    caller
        .contract
        .methods()
        .assets(identifier)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn escrows(caller: &User, identifier: u64) -> Option<EscrowInfo> {
    caller
        .contract
        .methods()
        .escrows(identifier)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn escrow_count(caller: &User) -> u64 {
    caller
        .contract
        .methods()
        .escrow_count()
        .call()
        .await
        .unwrap()
        .value
}
