contract;

use core::num::*;
use std::{
    context::this_balance,
    contract_id::ContractId,
    token::
}


////////////////////////////////////////
// Constants
////////////////////////////////////////

// const ASSET = ~ContractId::from(0x...);


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
    fn constructor(owner: Identity);
    fn mint(to: Identity, amount: u64);
    fn burn(from: Identity, amount: u64);
    fn name() -> str[11];
    fn symbol() -> str[11];
    fn decimals() -> u8;
    fn total_supply() -> u64;
}

impl Vault for Contract {

    /**
    MUST be an ERC-20 token contract.
    MUST NOT revert.
    */
    /// The address of the underlying asset used for the Vault for accounting,
    /// depositing, and withdrawing.
    fn asset() -> ContractId {
        ASSET
    }


    /**
    SHOULD include any compounding that occurs from yield.
    MUST be inclusive of any fees that are charged against assets in the Vault.
    MUST NOT revert.
    */
    /// The total amount of the underlying asset that is managed by the Vault.
    fn total_assets() -> u64 {
        let current_vault_balance = this_balance(ASSET);
    }

    /**
    MUST NOT be inclusive of any fees that are charged against assets in the Vault.
    MUST NOT show any variations depending on the caller.
    MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
    MUST NOT revert unless due to integer overflow caused by an unreasonably large input.
    MUST round down towards 0.
    This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and from
    */
    /// The amount of shares that the Vault would exchange for the amount of
    /// assets provided, in an ideal scenario where all the conditions are met.
    fn convert_to_shares(assets: u64) -> u64 {

    }

    /**
    MUST NOT be inclusive of any fees that are charged against assets in the Vault.
    MUST NOT show any variations depending on the caller.
    MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
    MUST NOT revert unless due to integer overflow caused by an unreasonably large input.
    MUST round down towards 0.
    This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and from.
    */
    /// The amount of assets that the Vault would exchange for the amount of
    /// shares provided, in an ideal scenario where all the conditions are met.
    fn convert_to_assets(shares: u64) -> u64 {

    }

    /**
    MUST return the maximum amount of assets deposit would allow to be deposited for receiver and not cause a revert, which MUST NOT be higher than the actual maximum that would be accepted (it should underestimate if necessary). This assumes that the user has infinite assets, i.e. MUST NOT rely on balanceOf of asset.
    MUST factor in both global and user-specific limits, like if deposits are entirely disabled (even temporarily) it MUST return 0.
    MUST return ~u64::max() if there is no limit on the maximum amount of assets that may be deposited.
    MUST NOT revert.
    */
    /// Maximum amount of the underlying asset that can be deposited into the
    /// Vault for the receiver, through a deposit call.
    fn max_deposit(receiver: Identity) -> u64 {

    }

    /**
    MUST return as close to and no more than the exact amount of Vault shares that would be minted in a deposit call in the same transaction. I.e. deposit should return the same or more shares as previewDeposit if called in the same transaction.
    MUST NOT account for deposit limits like those returned from maxDeposit and should always act as though the deposit would be accepted, regardless if the user has enough tokens approved, etc.
    MUST be inclusive of deposit fees. Integrators should be aware of the existence of deposit fees.
    MUST NOT revert due to vault specific user/global limits. MAY revert due to other conditions that would also cause deposit to revert.
    Note that any unfavorable discrepancy between convertToShares and previewDeposit SHOULD be considered slippage in share price or some other type of condition, meaning the depositor will lose assets by depositing.
    */
    /// Allows an on-chain or off-chain user to simulate the effects of their
    /// deposit at the current block, given current on-chain conditions.
    fn preview_deposit(assets: u64) -> u64{

    }

    /**

    */
    ///
    fn deposit(assets: u64, receiver: Identity) -> u64 {

    }

    /**

    */
    ///
    fn max_mint(receiver: Identity) -> u64 {

    }

    /**

    */
    ///
    fn preview_mint(shares: u64) -> u64 {

    }

    /**

    */
    ///
    fn mint(shares: u64, receiver: Identity) -> u64 {

    }

    /**

    */
    ///
    fn max_withdraw(owner: Identity) -> u64 {

    }

    /**

    */
    ///
    fn preview_withdraw(assets: u64) -> u64 {

    }

    /**

    */
    ///
    fn withdraw(assets: u64, receiver: Identity, owner: Identity) -> u64 {

    }

    /**

    */
    ///
    fn max_redeem(owner: Identity) -> u64 {

    }

    /**

    */
    ///
    fn preview_redeem(shares: u64) -> u64 {

    }

    /**

    */
    ///
    fn redeem(shares: u64, receiver: Identity, owner: Identity) -> u64 {

    }


}
