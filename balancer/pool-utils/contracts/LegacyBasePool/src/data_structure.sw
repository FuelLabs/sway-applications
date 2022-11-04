library data_structures;

use std::{
    contract_id::ContractId,
    identity::Identity,
    option::Option,
    vec::Vec
};

/// Pools
///
/// There are three specialization settings for Pools, which allow for cheaper swaps at the cost of reduced
/// functionality:
///
///  - General: no specialization, suited for all Pools. IGeneralPool is used for swap request callbacks, passing the
/// balance of all tokens in the Pool. These Pools have the largest swap costs (because of the extra storage reads),
/// which increase with the number of registered tokens.
///
///  - Minimal Swap Info: IMinimalSwapInfoPool is used instead of IGeneralPool, which saves gas by only passing the
/// balance of the two tokens involved in the swap. This is suitable for some pricing algorithms, like the weighted
/// constant product one popularized by Balancer V1. Swap costs are smaller compared to general Pools, and are
/// independent of the number of registered tokens.
///
///  - Two Token: only allows two tokens to be registered. This achieves the lowest possible swap gas cost. Like
/// minimal swap info Pools, these are called via IMinimalSwapInfoPool.
pub enum PoolSpecialization{
    GENERAL: (),
    MINIMAL_SWAP_INFO: (),
    TWO_TOKEN: () 
}

pub trait onJoinPool {
    fn on_join_pool()-> u64;
    fn on_exit_pool()-> u64;
}

