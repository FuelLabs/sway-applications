library events;

dep entity;

use entity::Entity;

pub struct ApprovalEvent {
    owner: Entity,
    approved: Entity,
    token_id: u64
}

pub struct BurnEvent {
    owner: Entity,
    token_id: u64
}

pub struct MintEvent {
    owner: Entity,
    token_id: u64
}

pub struct OperatorEvent {
    owner: Entity,
    operator: Entity
}

pub struct TransferEvent {
    from: Entity,
    to: Entity,
    token_id: u64
}
