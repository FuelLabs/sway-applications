contract;

dep data_structures/example;
dep errors;
dep events;
dep interface;
dep utils;

use errors::*;
use events::{DepositEvent, MintEvent, RedeemEvent, WithdrawEvent};
use example::*;
use interface::{Info, Vault};
use utils::*;

impl Vault for Contract {
    fn deposit(assets: u64, receiver: Identity) -> u64 {
        0
    }

    fn mint(shares: u64, receiver: Identity) -> u64 {
        0
    }

    fn redeem(shares: u64, receiver: Identity, owner: Identity) -> u64 {
        0
    }

    fn withdraw(assets: u64, receiver: Identity, owner: Identity) -> u64 {
        0
    }
}

impl Info for Contract {
    fn asset() -> ContractId {
        ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000)
    }

    fn convert_to_assets(shares: u64) -> u64 {
        0
    }

    fn convert_to_shares(assets: u64) -> u64 {
        0
    }

    fn max_deposit(receiver: Identity) -> u64 {
        0
    }

    fn max_mint(receiver: Identity) -> u64 {
        0
    }

    fn max_redeem(owner: Identity) -> u64 {
        0
    }

    fn max_withdraw(owner: Identity) -> u64 {
        0
    }

    fn preview_deposit(assets: u64) -> u64 {
        0
    }

    fn preview_mint(shares: u64) -> u64 {
        0
    }

    fn preview_redeem(shares: u64) -> u64 {
        0
    }

    fn preview_withdraw(assets: u64) -> u64 {
        0
    }

    fn total_assets() -> u64 {
        0
    }
}
