library interface;

abi Info {
    /// The underlying token used for the vault for accounting, depositing, and withdrawing
    fn asset() -> ContractId;

    /// The amount of assets that the vault would exchange for the amount of `shares` provided
    fn convert_to_assets(shares: u64) -> u64;

    /// The amount of shares that the vault would exchange for the amount of `assets` provided
    fn convert_to_shares(assets: u64) -> u64;

    /// Maximum amount of the underlying asset that can be deposited into the vault for the `receiver`, through a `deposit` call
    fn max_deposit(receiver: Identity) -> u64;

    /// Maximum amount of shares that can be minted from the vault for the `receiver`, through a `mint` call
    fn max_mint(receiver: Identity) -> u64;

    /// Maximum amount of vault shares that can be redeemed from the `owner` balance in the vault, through a `redeem` call
    fn max_redeem(owner: Identity) -> u64;

    /// Maximum amount of the underlying asset that can be withdrawn from the `owner` balance in the vault, through a `withdraw` call
    fn max_withdraw(owner: Identity) -> u64;

    /// Allows an on-chain or off-chain user to simulate the effects of their deposit
    fn preview_deposit(assets: u64) -> u64;

    /// Allows an on-chain or off-chain user to simulate the effects of their mint
    fn preview_mint(shares: u64) -> u64;

    /// Allows an on-chain or off-chain user to simulate the effects of their redemption
    fn preview_redeem(shares: u64) -> u64;

    /// Allows an on-chain or off-chain user to simulate the effects of their withdrawal
    fn preview_withdraw(assets: u64) -> u64;

    /// Total amount of the underlying asset that is “managed” by vault
    fn total_assets() -> u64;
}

abi Vault {
    /// Mints vault shares to `receiver` by depositing exactly `assets` of underlying tokens
    fn deposit(assets: u64, receiver: Identity) -> u64;
    /// Mints exactly `shares` vault shares to `receiver` by depositing assets of underlying tokens
    fn mint(shares: u64, receiver: Identity) -> u64;
    /// Burns exactly `shares` from `owner` and sends assets of underlying tokens to `receiver`
    fn redeem(shares: u64, receiver: Identity, owner: Identity) -> u64;
    /// Burns shares from `owner` and sends exactly `assets` of underlying tokens to `receiver`
    fn withdraw(assets: u64, receiver: Identity, owner: Identity) -> u64;
}
