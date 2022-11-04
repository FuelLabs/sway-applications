library interface;

/// Linear Pools are designed to hold two assets: "main" and "wrapped" tokens that have an equal value underlying
/// token (e.g., DAI and waDAI). There must be an external feed available to provide an exact, non-manipulable exchange
/// rate between the tokens. In particular, any reversible manipulation (e.g. causing the rate to increase and then
/// decrease) can lead to severe issues and loss of funds.
///
/// The Pool will register three tokens in the Vault however: the two assets and the BPT itself,
/// so that BPT can be exchanged (effectively joining and exiting) via swaps.
///
/// Despite inheriting from BasePool, much of the basic behavior changes. This Pool does not support regular joins and
/// exits, as the entire BPT supply is 'preminted' during initialization.
///
/// Unlike most other Pools, this one does not attempt to create revenue by charging fees: value is derived by holding
/// the wrapped, yield-bearing asset. However, the 'swap fee percentage' value is still used, albeit with a different
/// meaning. This Pool attempts to hold a certain amount of "main" tokens, between a lower and upper target value.
/// The pool charges fees on trades that move the balance outside that range, which are then paid back as incentives to
/// traders whose swaps return the balance to the desired region.
/// The net revenue via fees is expected to be zero: all collected fees are used to pay for this 'rebalancing'.
const _TOTAL_TOKENS: u64 = 3; // Main token, wrapped token, BPT

// This is the maximum token amount the Vault can hold. In regular operation, the total BPT supply remains constant
// and equal to _INITIAL_BPT_SUPPLY, but most of it remains in the Pool, waiting to be exchanged for tokens. The
// actual amount of BPT in circulation is the total supply minus the amount held by the Pool, and is known as the
// 'virtual supply'.
// The total supply can only change if the emergency pause is activated by governance, enabling an
// alternative proportional exit that burns BPT. As this is not expected to happen, we optimize for
// success by using _INITIAL_BPT_SUPPLY instead of total_supply(), saving a storage read. This optimization is only
// valid if the Pool is never paused: in case of an emergency that leads to burned tokens, the Pool should not
// be used after the buffer period expires and it automatically 'unpauses'.
const _INITIAL_BPT_SUPPLY = 2.pow(112) - 1;
//!no token Standard available as of now so taking the dummy data
const TOTAL_SUPPLY = 10000000;


// The lower and upper target are in BasePool's misc data field, which has 192 bits available (as it shares the same
// storage slot as the swap fee percentage, which is 64 bits). These are already scaled by the main token's scaling
// factor, which means that the maximum upper target is ~80 billion in the main token units if the token were to
// have 18 decimals (2^(192/2) / 10^18), which is more than enough.
// [        64 bits       |    96 bits   |    96 bits    ]
// [       reserved       | upper target |  lower target ]
// [  base pool swap fee  |         misc data            ]
// [ MSB                                             LSB ]

const _LOWER_TARGET_OFFSET = 0;
const _UPPER_TARGET_OFFSET = 96;

const _MAX_UPPER_TARGET = 2.pow(96) - 1;
// Both BPT and the main token have a regular, constant scaling factor (equal to FixedPoint.ONE for BPT, and
// dependent on the number of decimals for the main token). However, the wrapped token's scaling factor has two
// components: the usual token decimal scaling factor, and an externally provided rate used to convert wrapped
// tokens to an equivalent main token amount. This external rate is expected to be ever increasing, reflecting the
// fact that the wrapped token appreciates in value over time (e.g. because it is accruing interest).
    
abi LinearPool {
    fn init_linear_pool(
        vault: Address,
        mainToken: ContractId, 
        wrappedToken: ContractId,
        swapFeePercentage: u64,
        pauseWindowDuration: u64,
        bufferPeriodDuration: u64,
        owner: Identity
    );

    ///
    /// * Finishes initialization of the Linear Pool: it is unusable before calling this fn as no BPT will have
    /// * been minted.
    /// *
    /// * Since Linear Pools have preminted BPT stored in the Vault, they require an initial join to deposit said BPT as
    /// * their balance. Unfortunately, this cannot be performed during construction, as a join involves calling the
    /// * `onJoinPool` fn on the Pool, and the Pool will not have any code until construction finishes. Therefore,
    /// * this must happen in a separate call.
    /// *
    /// * It is highly recommended to create Linear pools using the LinearPoolFactory, which calls `initialize`
    /// * automatically.
    /// 
    fn initialize();
    
    // In most Pools, swaps involve exchanging one token held by the Pool for another. In this case however, since
    // one of the three tokens is the BPT itself, a swap might also be a join (main/wrapped for BPT) or an exit
    // (BPT for main/wrapped).
    // All three swap types (swaps, joins and exits) are fully disabled if the emergency pause is enabled. Under
    // these circumstances, the Pool should be exited using the regular Vault.exitPool function.

    fn on_swap(
        request: SwapRequest,
        balances: Vec<u64>,
        indexIn: u64,
        indexOut: u64
    )  ->u64;

    fn _on_initialize_pool(
        poolId: b256,
        address sender: Address,
        address recipient: Address,
        scalingFactors: Vec<u64>,
        //userData: bytes
    ) ->(u64, Vec<u64>);



    /// *Pack targets as two uint96 values into a single storage slot. This results in targets being capped to 96
    /// *bits, but that should be more than enough. Values are already checked for validity above.
    fn _set_targets(mainToken: ContractId, lowerTarget: u64, upperTargetL: u64);

    ///    * For a new target range to be valid:
    ///  * the pool must currently be between the current targets (meaning no fees are currently pending)
    ///  * the pool must currently be between the new targets (meaning setting them does not cause for fees to be
    ///    pending)
    /// *The first requirement could be relaxed, as the LPs actually benefit from the pending fees not being paid out,
    /// *but being stricter makes analysis easier at little expense.

    fn set_targets(newLowerTarget: u64, newUpperTarget: u64);

    ///* When a user calls `exitPool`, this is the first point of entry from the Vault.
    ///* We first check whether this is a Recovery Mode exit - if so, we proceed using this special lightweight exit
    ///* mechanism which avoids computing any complex values, interacting with external contracts, etc., and generally
    ///* should always work, even if the Pool's mathematics or a dependency break down.
    fn on_exit_pool( poolId: b256, sender: Address, recipient: Address, balances: Vec<u64>, lastChangeBlock: u64, protocolSwapFeePercentage: u64,
     //userData: bytes
    )->(u64, Vec<u64>, Vec<u64>);

    ///
    ///Called whenever the Pool is joined after the first initialization join (see `_onInitializePool`).
    ///
    ///Returns the amount of BPT to mint, the token amounts that the Pool will receive in return, and the number of
    ///tokens to pay in protocol swap fees.
    ///
    ///Implementations of this fn might choose to mutate the `balances` array to save gas (e.g. when
    ///performing intermediate calculations, such as subtraction of due protocol fees). This can be done safely.
    ///
    ///Minted BPT will be sent to `recipient`.
    ///
    ///The tokens granted to the Pool will be transferred from `sender`. These amounts are considered upscaled and will
    ///be downscaled (rounding up) before being returned to the Vault.
    ///
    ///Due protocol swap fees will be taken from the Pool's balance in the Vault (see `IBasePool.onJoinPool`). These
    ///amounts are considered upscaled and will be downscaled (rounding down) before being returned to the Vault.
    ///
    
    ///*Vault hook for adding liquidity to a pool (including the first time, "initializing" the pool).
    ///*This fn can only be called from the Vault, from `joinPool`.
    
    fn on_join_pool(poolId: b256, sender: Address, recipient: Address, balances: Vec<u64>, lastChangeBlock: u64, protocolSwapFeePercentage: u64,
    //userData: bytes
    )->(Vec<u64>, Vec<u64>);

    fn _on_join_pool(
        poolId :b256,
        sender :address,
        recipient: address,
        balances: Vec<u64>uint256[] memory,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        scalingFactors: Vec<u64>,
        //userData: bytes
    ) -> (u64, Vec<u64>, Vec<u64>);

    ///*Called whenever the Pool is exited.
    ///
    ///*Returns the amount of BPT to burn, the token amounts for each Pool token that the Pool will grant in return, and
    ///*the number of tokens to pay in protocol swap fees.
    ///
    ///*Implementations of this fn might choose to mutate the `balances` array to save gas (e.g. when
    ///*performing intermediate calculations, such as subtraction of due protocol fees). This can be done safely.
    ///
    ///*BPT will be burnt from `sender`.
    ///
    ///*The Pool will grant tokens to `recipient`. These amounts are considered upscaled and will be downscaled
    ///*(rounding down) before being returned to the Vault.
    ///
    ///*Due protocol swap fees will be taken from the Pool's balance in the Vault (see `IBasePool.onExitPool`). These
    ///*amounts are considered upscaled and will be downscaled (rounding down) before being returned to the Vault.
    ///
    
    /// * Vault hook for removing liquidity from a pool.
    /// * This fn can only be called from the Vault, from `exitPool`.
     
    fn _on_exit_pool( poolId: b256, sender: Address, recipient: Address, balances: Vec<u64>, lastChangeBlock: u64, protocolSwapFeePercentage: u64,
    //userData: bytes
    )->(Vec<u64>, Vec<u64>);

    // This proportional exit fn is only enabled if the contract is paused, to provide users a way to
    // retrieve their tokens in case of an emergency.
    //
    // This particular exit fn is the only one available because it is the simplest, and therefore least
    // likely to be incorrect, or revert and lock funds.
    fn _emergency_proportional_exit(
        balances: Vec<u64>,
        ///userData: bytes
    )-> (u64, Vec<u64>);

    // For the swap fee percentage to be changeable:
    //  - the pool must currently be between the current targets (meaning no fees are currently pending)
    //
    // As the amount of accrued fees is not explicitly stored but rather derived from the main token balance and the
    // current swap fee percentage, requiring for no fees to be pending prevents the fee setter from changing the
    // amount of pending fees, which they could use to e.g. drain Pool funds in the form of inflated fees.

    fn set_swap_fee_percentage_(swapFeePercentage: u64);
}