library data_structures;

use std::{address::Address, contract_id::ContractId, vec::Vec};

// There are two variants of swap kind that a user can perform -
//
// * `GivenIn` swaps -- where the amount of tokens in (sent to the Pool) is known, and the Pool determines
// the amount of tokens out (to send to the recipient).
//
// * `GivenOut` swaps -- where the amount of tokens out (received from the Pool) is known, and the Pool determines
// (via the `onSwap` hook) the amount of tokens in (to receive from the sender).
pub enum SwapKind {
    GivenIn: (),
    GivenOut: (),
}

// Required data for the single swap which is executed by the `swap()` abi function.
// Amount is either `amount_in` or `amount_out` depending on the swap kind variant (enum SwapKind)
//
// `asset_in` or `asset_out` are either token addresses or sentinal value for FUEL (the zero address i.e 0x0000...)
// Pools are not going to interact with the FUEL directly: it will be wrapped to or unwrapped from WFUEL by the Vault.
//
// `user_data` field is ignored by the vault, but will be forwarded to the respective pool.
pub struct SingleSwap {
    amount: u64,
    asset_in: ContractId,
    asset_out: ContractId,
    kind: SwapKind,
    pool_id: b256,
    user_data: b256,
}

// All tokens in a swap are either sent from the `sender` account to the Vault, or from the Vault to the
// `recipient` account.
//
// If the caller is not `sender`, it must be an authorized relayer for them.
//
// If the `from_internal_balance` field is true, the `sender`'s Internal Balance will be preferred, performing an ERC20
// transfer for the difference between the requested amount and the User's Internal Balance (if any).
//
// If the `to_internal_balance` field is true, tokens will be deposited to `recipient`'s internal balance instead of
// transferred.
//
// FUEL cannot be deposited to or withdrawn from Internal Balance: attempting to do so will trigger a revert.
pub struct FundManagement {
    sender: Address,
    from_internal_balance: bool,
    to_internal_balance: bool,
    recipient: Address,
}

// This data structure represents a request for a token swap, where `kind` indicates the swap type (`GivenIn` or
// `GivenOut`) which indicates whether or not the amount sent by the pool is known.
//
// The pool receives `token_in` and sends `token_out`. `amount` is the number of `token_in` tokens the pool will take
// in, or the number of `token_out` tokens the Pool will send out, depending on the given swap `kind`.
//
// `pool_id` is the ID of the Pool involved in the swap - this is useful for Pool contracts that implement more than
// one Pool.
//
// The meaning of `last_change_block` depends on the Pool specialization:
//   - Two Token or Minimal Swap Info: the last block in which either `token_in` or `token_out` changed its total
//     balanace.
//   - General: the last block in which *any* of the Pool's registered tokens changed its total balance.
//
// The field `from` is the origin address for the funds the Pool receives, and `to` is the destination address
// where the Pool sends the outgoing tokens.
//
// `user_data` is extra data provided by the caller - typically a signature from a trusted party.
pub struct SwapRequest {
    kind: SwapKind,
    token_in: ContractId,
    token_out: ContractId,
    amount: u64,
    pool_id: b256,
    last_change_block: u64,
    from: Address,
    to: Address,
    user_data: b256,
}

// The data for each individual swap executed by `batch_swap()` abi function. The asset in and out fields 
// are indexes into the `assets` array passed to that function, and FUEL assets are converted to WFUEL.
//
// If `amount` is zero then the multihop mechanism is used to determine the actual amount based on the amount in/out
// from the previous swap, depending on the swap kind.
//
// The `user_data` field is ignored by the Vault, but forwarded to the Pool.
pub struct BatchSwapStep {
    pool_id: b256,
    asset_in_index: u64,
    asset_out_index: u64,
    amount: u64,
    user_data: b256,
}

// * DepositInternal -- 
// Increases the Internal Balance of the `recipient` account by transferring tokens from the corresponding
// `sender`.
//
// FUEL can be used by passing the FUEL sentinel value as the asset and forwarding FUEL in the call: it will be wrapped
// and deposited as WFUEL. Any FUEL amount remaining will be sent back to the caller (not the sender, which is
// relevant for relayers).
//
// Emits an `EventInternalBalanceChanged` event.
//
// * WithdrawInternal -- 
// Decreases the Internal Balance of the `sender` account by transferring tokens to the `recipient`.
//
// FUEL can be used by passing the FUEL sentinel value as the asset. This will deduct WFUEL instead, unwrap it and send
// it to the recipient as FUEL.
//
// Emits an `EventInternalBalanceChanged` event.
//
// * TransferInternal -- 
// Transfers tokens from the Internal Balance of the `sender` account to the Internal Balance of `recipient`.
// Reverts if the FUEL sentinel value is passed.
//
// Emits an `EventInternalBalanceChanged` event.
//
// * TransferExternal -- 
// Transfers tokens from `sender` to `recipient`, using the Vault's ERC20 allowance. This is typically used by
// relayers, as it lets them reuse a user's Vault allowance.
// Reverts if the FUEL sentinel value is passed.
//
// Emits an `EventExternalBalanceTransfer` event.
pub enum UserBalanceOpKind {
    DepositInternal: (),
    WithdrawInternal: (),
    TransferInternal: (),
    TransferExternal: (),
}

// Data for `manage_user_balance()` abi function operations, which include the possibility for FUEL to be sent 
// and received without manual WFUEL wrapping or unwrapping.
pub struct UserBalanceOp {
    kind: UserBalanceOpKind,
    asset: ContractId,
    amount: u64,
    sender: Address,
    recipient: Address,
}

// Types of Pool
//
// There are three specialization settings for Pools, which allow for cheaper swaps at the cost of reduced
// functionality:
//
// * General -- 
// no specialization, suited for all Pools. Swap request callbacks are made by passing the balance of all 
// tokens in the Pool. These Pools have the largest swap costs (because of the extra storage reads),
// which increase with the number of registered tokens.
//
// * MinimalSwapInfo -- 
// Saves gas by only passing the balance of the two tokens involved in the swap. This is suitable for
// some pricing algorithms, like the weighted constant product one popularized by Balancer V1. Swap costs
// are smaller compared to general Pools, and are independent of the number of registered tokens.
//
// * TwoToken -- 
// Only allows two tokens to be registered. This achieves the lowest possible swap gas cost. Like minimal
// swap info Pools.
pub enum PoolSpecialization {
    General: (),
    MinimalSwapInfo: (),
    TwoToken: (),
}

// * Withdraw -- 
// Decrease the Pool's cash, but increase its managed balance, leaving the total balance unchanged.
//
// * Deposit -- 
// Increase the Pool's cash, but decrease its managed balance, leaving the total balance unchanged.
//
// * Update -- 
// Updates don't affect the Pool's cash balance, but because the managed balance changes, it does alter the total.
// The external amount can be either increased or decreased by this call (i.e., reporting a gain or a loss).
pub enum PoolBalanceOpKind {
    Withdraw: (),
    Deposit: (),
    Update: (),
}

// `PoolBalanceOp` struct fields contains data that is used to call `manage_pool_balance()` abi function.
// Performs a set of Pool balance operations, which may be either withdrawals, deposits or updates.
pub struct PoolBalanceOp {
    kind: PoolBalanceOpKind,
    pool_id: b256,
    token: ContractId,
    amount: u64,
}

// AbiEncode struct for creating a workaround for solidity `abi.encode()` function.
pub struct AbiEncode {
    token_a: ContractId,
    token_b: ContractId,
}

// Required data for pools with the Two Token specialization setting.
//
// These are similar to the Minimal Swap Info Pool case (because the Pool only has two tokens, and therefore there
// are only two balances to read), but there's a key difference in how data is stored. Keeping a set makes little
// sense, as it will only ever hold two tokens, so we can just store those two directly.
//
// The gas savings associated with using these Pools come from how token balances are stored: cash amounts for token
// A and token B are packed together, as are managed amounts. Because only cash changes in a swap, there's no need
// to write to this second storage slot. A single last change block number for both tokens is stored with the packed
// cash fields.
pub struct TwoTokenPoolBalances {
    shared_cash: b256,
    shared_managed: b256,
}

// Tokens which are associated with the TwoToken Pool are stored in this struct.
//
// * `token_a` -- Address of first token.
//
// * `token_b` -- Address of second token.
//
// * `balances` -- Balances of both the tokens which are stored in the storage map with key as b256.
pub struct TwoTokenPoolTokens {
    token_a: ContractId,
    token_b: ContractId,
    balances: b256,
}

// Struct used by IERC20ToBytes32Map.
pub struct IERC20ToBytes32MapEntry {
    key: ContractId,
    value: b256,
}

// The original OpenZeppelin implementation uses a generic Map type with bytes32 keys: this was replaced with
// IERC20ToBytes32Map and IERC20ToUint256Map, resulting in more dense bytecode (as long as each contract only uses
// one of these - there'll otherwise be duplicated code).
pub struct IERC20ToBytes32Map {
    entries: u64,
    indexes: ContractId,
    length: u64,
}

// User needs to pass `JoinPoolRequest` struct object to `join_pool()` abi function to join a Pool, which
// transfers tokens from `sender` into the Pool's balance. This will trigger custom Pool behavior, which will
// typically grant something in return to `recipient` - often tokenized Pool shares.
//
// The `assets` and `max_amounts_in` arrays must have the same length, and each entry indicates the maximum amount
// to send for each asset. The amounts to send are decided by the Pool and not the Vault: it just enforces
// these maximums.
//
// `assets` must have the same length and order as the array returned by `get_pool_tokens()`. This prevents issues when
// interacting with Pools that register and deregister tokens frequently. If sending FUEL however, the array must be
// sorted *before* replacing the WFUEL address with the FUEL sentinel value (the zero address, i.e 0x00..), which
// means the final `assets` array might not be sorted. Pools with no registered tokens cannot be joined.
//
// If `from_internal_balance` is true, the caller's Internal Balance will be preferred: ERC20 transfers will only
// be made for the difference between the requested amount and Internal Balance (if any). Note that FUEL cannot be
// withdrawn from Internal Balance: attempting to do so will trigger a revert.
pub struct JoinPoolRequest {
    assets: Vec<ContractId>,
    max_amounts_in: Vec<u64>,
    user_data: UserData,
    from_internal_balance: bool,
}

// User needs to pass `JoinPoolRequest` struct object to `join_pool()` abi function to exit a Pool, which transfers
// tokens from the Pool's balance to `recipient`. This will trigger custom Pool behavior, which will typically ask
// for something in return from `sender` - often tokenized Pool shares. The amount of tokens that can be withdrawn
// is limited by the Pool's `cash` balance.
//
// The `tokens` and `min_amounts_out` arrays must have the same length, and each entry in these indicates the minimum
// token amount to receive for each token contract. The amounts to send are decided by the Pool and not the Vault:
// it just enforces these minimums.
//
// `assets` must have the same length and order as the array returned by `get_pool_tokens()`. This prevents issues when
// interacting with Pools that register and deregister tokens frequently. If receiving FUEL however, the array must
// be sorted *before* replacing the WFUEL address with the FUEL sentinel value (the zero address, i.e 0x00..), which
// means the final `assets` array might not be sorted. Pools with no registered tokens cannot be exited.
//
// If `toInternalBalance` is true, the tokens will be deposited to `recipient`'s Internal Balance. Otherwise,
// an ERC20 transfer will be performed. Note that ETH cannot be deposited to Internal Balance: attempting to
// do so will trigger a revert.
pub struct ExitPoolRequest {
    assets: Vec<ContractId>,
    min_amounts_out: Vec<u64>,
    user_data: UserData,
    to_internal_balance: bool,
}

// This has the exact same layout as JoinPoolRequest and ExitPoolRequest, except the `max_amounts_in` and
// `min_amounts_out` are called `limits`. Internally we use this struct for both since these two functions are quite
// similar, but expose the others to callers for clarity.
pub struct PoolBalanceChange {
    assets: Vec<ContractId>,
    limits: Vec<u64>,
    user_data: UserData,
    use_internal_balance: bool,
}

enum RequestKind {
    ExactToken: (),
    ExactTokensOut: (),
    Init: (),
    InForExactTokensOut: (),
    Token: (),
}

pub struct UserData {
    amount: u64,
    amounts_in_out: Vec<u64>,
    bpt_amount_in_out: u64,
    kind: RequestKind,
    max_min_bpt_amount: u64,
}

pub enum PoolBalanceChangeKind {
    Join: (),
    Exit: (),
}