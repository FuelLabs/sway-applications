library events;

pub struct AddLiquidityEvent {
    /// Amount of asset A added to reserves
    asset_a: u64,
    /// Amount of asset B added to reserves
    asset_b: u64,
    /// Amount of liquidity pool assets minted and transferred to sender
    liquidity: u64,
}

pub struct DefineAssetPairEvent {
    /// The pair that makes up the pool
    pair: (ContractId, ContractId),
}

pub struct DepositEvent {
    /// Deposited asset that is either asset A or asset B
    asset: ContractId,
    /// Deposited amount of the asset that may be withdrawn of used to add liquidity
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
    /// Amount of the input asset that was sold
    sold: u64,
    /// Amount of the output asset that was bought
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
