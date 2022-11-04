
contract;

use std::{
    address::*, 

    block::*,
    chain::auth::*,
    context::{*, call_frames::*},
    contract_id::ContractId,
    hash::*,
    option::Option,
    result::*,
    revert::{revert, require},
    storage::*,
    token::*,
    u128::U128,
    vec::Vec,
};


// use Authentication::*; 
// use Authorizer::*;

abi SingletonAuthentication{
    // #[storage(read)]fn getVault() -> IVault;
    #[storage(read)]fn getAuthorizer() -> ContractId;
    // fn _can_Perform(actionId: b256, account: Address) -> bool;
    // fn _canPerform(actionId: b256,account: Address, to: Address) -> bool;
}

const this: ContractId = ~ContractId::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);

storage {
    _vault: ContractId = ~ContractId::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),
}


/**
    * @notice Returns the Balancer Vault
    */
#[storage(read)]fn getVault() -> ContractId {
    storage._vault;
}

#[storage(read)]fn _can_Perform(actionId: b256, account: Address) -> bool {
    getAuthorizer().canPerform(actionId, account, ~Address::from(this.into()));
}

#[storage(read)]fn _canPerform(
    actionId: b256,
    account: Address,
    to: Address
) -> bool {
    getAuthorizer().canPerform(actionId, account, to);
}


impl SingletonAuthentication for Contract {


    // Use the contract's own address to disambiguate action identifiers


    /**
        * @notice Returns the Authorizer
        */
    #[storage(read)]fn getAuthorizer() -> ContractId {
        getVault().getAuthorizer();
    }


}