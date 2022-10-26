library events;

pub struct AddLiquidityEvent {
    /// Amount of asset A added to reserves
    a: u64,
    /// Amount of asset B added to reserves
    b: u64,
    /// Amount of liquidity pool assets minted and transferred to sender
    liquidity: u64,
}

pub struct DefineAssetPairEvent {
    /// The pair that makes up the pool
    pair: (ContractId, ContractId),
}

pub struct DepositEvent {
    /// Deposited asset
    asset: ContractId,
    /// Deposited amount
    amount: u64,
    /// New deposit balance of asset in contract
    balance: u64,
}

pub struct RemoveLiquidityEvent {
    /// Amount of asset A removed from reserves and transferred to sender
    amount_a: u64,
    /// Amount of asset B removed from reserves and transferred to sender
    amount_b: u64,
    /// Amount of liquidity pool assets burned
    liquidity: u64,
}

pub struct SwapEvent {
    /// Identifier of input asset
    input: ContractId,
    /// Identifier of output asset
    output: ContractId,
    /// Sold input asset amount
    sold: u64,
    /// Bought output asset amount
    bought: u64,
}

pub struct WithdrawEvent {
    /// Identifier of withdrawn asset
    asset: ContractId,
    /// Amount of withdrawal
    amount: u64,
    /// Remaining deposit balance of asset in contract
    balance: u64,
}
