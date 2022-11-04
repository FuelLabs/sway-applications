contract;

use std::{
    address::*,

    block::*,
    chain::auth::*,
    context::{*, call_frames::*},
    contract_id::ContractId,
    hash::*,
    result::*,
    revert::revert,
    storage::*,
    token::*,
    u128::U128,
    vec::Vec,
    identity::Identity,
};

use Authentication::{_is_owner_only_action, _get_authorizer};

abi BasePoolAuthorization {
    // fn get_authorizer() -> IAuthorizer;
    fn _can_perform(actionId: b256, account: Address) -> bool;
}


// constructor arguments so for now a dummy value
const _OWNER: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _DELEGATE_OWNER: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;

/**
 * @dev Base authorization layer implementation for Pools.
 *
 * The owner account can call some of the permissioned functions - access control of the rest is delegated to the
 * Authorizer. Note that this owner is immutable: more sophisticated permission schemes, such as multiple ownership,
 * granular roles, etc., could be built on top of this by making the owner a smart contract.
 *
 * Access control of all other permissioned functions is delegated to an Authorizer. It is also possible to delegate
 * control of *all* permissioned functions to the Authorizer by setting the owner address to `_DELEGATE_OWNER`.
 */
fn get_owner() -> Address {
    return ~Address::from(_OWNER);
}

impl BasePoolAuthorization for Contract {
    // fn get_authorizer() -> IAuthorizer {
    //     return _get_authorizer();
    // }

    fn _can_perform(actionId: b256, account: Address) -> bool {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        if ((get_owner() != ~Address::from(_DELEGATE_OWNER)) && _is_owner_only_action(actionId)) {
            // Only the owner can perform "owner only" actions, unless the owner is delegated.
            return sender == get_owner();
        } else {
            // Non-owner actions are always processed via the Authorizer, as "owner only" ones are when delegated.
            return _get_authorizer().canPerform(actionId, account, sender);
        }
    }

    // fn _is_owner_only_action(bytes32 actionId) internal view virtual returns (bool);

    // fn _get_authorizer() internal view virtual returns (IAuthorizer);
}
