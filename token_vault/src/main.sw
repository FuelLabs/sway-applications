contract;

////////////////////////////////////////
// Constants
////////////////////////////////////////

////////////////////////////////////////
// Storage declarations
////////////////////////////////////////

////////////////////////////////////////
// ABI definitions
////////////////////////////////////////

abi FungibleToken {
    fn constructor(owner: Identity);
    fn mint(to: Identity, amount: u64);
    fn burn(from: Identity, amount: u64);
    fn name() -> str[11];
    fn symbol() -> str[11];
    fn decimals() -> u8;
    fn total_supply() -> u64;
    // fn transfer() no need to call contract to transfer native assets
    // fn transfer_from() can't use transfer_from with native assets
    // fn approve() can't use approve with native assets
    // fn allowance() no need for allowance() without approve() native assets
}

abi Vault {
    // vault-specific abi
    fn asset() -> ContractId;
    fn total_assets() -> u64;
    fn convert_to_shares(assets: u64) -> u64;
    fn convert_to_assets(shares: u64) -> u64;
    fn max_deposit(receiver: Identity) -> u64;
    fn preview_deposit(assets: u64) -> u64;
    fn deposit(assets: u64, receiver: Identity) -> u64;
    fn max_mint(receiver: Identity) -> u64;
    fn preview_mint(shares: u64) -> u64;
    fn mint(shares: u64, receiver: Identity) -> u64;
    fn max_withdraw(owner: Identity) -> u64;
    fn preview_withdraw(assets: u64) -> u64;
    fn withdraw(assets: u64, receiver: Identity, owner: Identity) -> u64;
    fn max_redeem(owner: Identity) -> u64;
    fn preview_redeem(shares: u64) -> u64;
    fn redeem(shares: u64, receiver: Identity, owner: Identity) -> u64;
}

////////////////////////////////////////
// Errors
////////////////////////////////////////

////////////////////////////////////////
// Events
////////////////////////////////////////

struct Deposit {
    caller: Identity,
    owner: Identity,
    assets: u64,
    shares: u64,
}

struct Withdraw {
    caller: Identity,
    reciever: Identity,
    owner: Identity,
    assets: u64,
    shares: u64,
}

////////////////////////////////////////
// ABI Implementations
////////////////////////////////////////

impl FungibleToken for Contract {

}

impl Vault for Contract {

}
