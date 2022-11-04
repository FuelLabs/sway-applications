library data_structures;

pub const MASK = 1;
pub const ONE: u64 = 1; // 18 decimal places
// TODO For time being there is no standard for fungible token so we are keeping them as ERC20 Later will change to the standards
//!currently it's dummy id when wfuel is added we will replace it
// Wraped FUEL ID
pub const WFUEL: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861;

// Sentinel value used to indicate WFUEL with wrapping/unwrapping semantics. The zero address is a good choice for
// multiple reasons: it is cheap to pass as a calldata argument, it is a known invalid token and non-contract, and
// it is an address Pools cannot register as a token.
pub const FUEL: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

use std::{address::Address, contract_id::ContractId, vec::Vec};

pub enum SwapKind {
    GIVEN_IN: (),
    GIVEN_OUT: (),
}

pub struct SingleSwap {
    poolId: b256,
    kind: SwapKind,
    assetIn: ContractId,
    assetOut: ContractId,
    amount: u64,
    userData: b256,
}

pub struct FundManagement {
    sender: Address,
    fromInternalBalance: bool,
    recipient: Address,
    toInternalBalance: bool,
}

pub struct SwapRequest {
    kind: SwapKind,
    tokenIn: ContractId,
    tokenOut: ContractId,
    amount: u64,
    // Misc data
    poolId: b256,
    lastChangeBlock: u64,
    from: Address,
    to: Address,
    userData: b256,
}

pub struct BatchSwapStep {
    poolId: b256,
    assetInIndex: u64,
    assetOutIndex: u64,
    amount: u64,
    userData: b256,
}

pub enum UserBalanceOpKind {
    DEPOSIT_INTERNAL: (),
    TRANSFER_EXTERNAL: (),
    TRANSFER_INTERNAL: (),
    WITHDRAW_INTERNAL: (),
}

pub struct UserBalanceOp {
    amount: u64,
    asset: ContractId,
    kind: UserBalanceOpKind,
    recipient: Address,
    sender: Address,
}

pub enum PoolSpecialization {
    GENERAL: (),
    MINIMAL_SWAP_INFO: (),
    TWO_TOKEN: (),
}

pub enum PoolBalanceOpKind {
    WITHDRAW: (),
    DEPOSIT: (),
    UPDATE: (),
}

pub struct PoolBalanceOp {
    kind: PoolBalanceOpKind,
    poolId: b256,
    token: ContractId,
    amount: u64,
}

pub struct abi_encode {
    token_a: ContractId,
    token_b: ContractId,
}

pub struct TwoTokenPoolBalances {
    shared_cash: b256,
    shared_managed: b256,
}

pub struct TwoTokenPoolTokens {
    token_a: ContractId,
    token_b: ContractId,
    // workaround of nested storageMap
    // balances: StorageMap<b256, TwoTokenPoolBalances>,
    balances: b256,
}

pub struct IERC20ToBytes32MapEntry {
    key: ContractId,
    value: b256,
}

pub struct IERC20ToBytes32Map {
    // Number of entries in the map
    length: u64,
    // Storage of map keys and values
    entries: u64,
    // Position of the entry defined by a key in the `entries` array, plus 1
    // because index 0 means a key is not in the map.
    indexes: ContractId,
}


// pub struct JoinPoolRequest {
//     assets: Vec<ContractId>,
//     maxAmountsIn: Vec<u64>,
//     // todo this was used in the abi.decode, removing dependencies fo abi.decode
//     // userData: Vec<b256>,
//     userData: UserData,
//     fromInternalBalance: bool,
// }
pub struct JoinPoolRequest {
    // assets: Vec<ContractId>,
    // maxAmountsIn: Vec<u64>,
    assets: [b256; 2],
    maxAmountsIn: [u64; 2],
    // todo this was used in the abi.decode, removing dependencies fo abi.decode
    // userData: Vec<b256>,
    userData: UserData1,
    fromInternalBalance: bool,
}

pub struct ExitPoolRequest {
    assets: Vec<ContractId>,
    minAmountsOut: Vec<u64>,
    // todo this was used in the abi.decode, removing dependencies fo abi.decode
    // userData: Vec<b256>,
    userData: UserData,
    toInternalBalance: bool,
}

// This has the exact same layout as JoinPoolRequest and ExitPoolRequest, except the `maxAmountsIn` and
// `minAmountsOut` are called `limits`. Internally we use this struct for both since these two functions are quite
// similar, but expose the others to callers for clarity.
pub struct PoolBalanceChange {
    assets: Vec<ContractId>,
    limits: Vec<u64>,
    // todo this was used in the abi.decode, removing dependencies fo abi.decode
    // userData: Vec<b256>,
    userData: UserData,
    useInternalBalance: bool,
}

enum JoinKind {
    INIT: (),
    EXACT_TOKENS_IN_FOR_BPT_OUT: (),
    TOKEN_IN_FOR_EXACT_BPT_OUT: (),
    ALL_TOKENS_IN_FOR_EXACT_BPT_OUT: (),
    ADD_TOKEN: (),
}

enum ExitKind {
    INIT: (),
    EXACT_BPT_IN_FOR_ONE_TOKEN_OUT: (),
    EXACT_BPT_IN_FOR_TOKENS_OUT: (),
    BPT_IN_FOR_EXACT_TOKENS_OUT: (),
    REMOVE_TOKEN: (),
}

enum RequestKind {
    INIT: (),
    EXACT_TOKEN: (),
    EXACT_TOKENS_OUT: (),
    IN_FOR_EXACT_TOKENS_OUT: (),
    TOKEN: (),
}

//todo of userData: Vec<b256>
// todo workaround for the vec
pub struct UserData1 {
    kind: RequestKind,
    amount: u64,
    maxMinBPTAmount: u64,
    bptAmountInOut: u64,
    // amountsInOut: Vec<u64>,
    amountsInOut: [u64; 2],
}

//todo of userData: Vec<b256>
pub struct UserData {
    kind: RequestKind,
    amount: u64,
    maxMinBPTAmount: u64,
    bptAmountInOut: u64,
    amountsInOut: Vec<u64>,
}

// //todo of userData: Vec<b256>
// pub struct JoinUserData {
//     kind: JoinKind,
//     bptAmountOut: u64,
//     tokenIndex: u64,
//     bptAmountInOut: u64,
//     amountsIn: Vec<u64>,    
// }
// //todo of userData: Vec<b256>
// pub struct ExitUserData {
//     kind: ExitKind,
//     bptAmountIn: u64,
//     tokenIndex: u64,
//     bptAmountInOut: u64,
//     amountsOut: Vec<u64>,    
// }
pub enum PoolBalanceChangeKind {
    JOIN: (),
    EXIT: (),
}
