contract;

use std ::{
    u128::U128,
    address::Address,
    math::*,
    hash::keccak256,
    context::call_frames::first_param,
    assert::assert,
    chain::auth::{AuthError, msg_sender},
    identity::Identity,
    result::Result,
    revert::{revert, require},
};

use BalancerErrors::{SENDER_NOT_ALLOWED};


abi Authentication {
    fn _authenticate_caller();
}

struct abi_encode {
    _action_in_disambiguator: b256,
    selector: b256,
}

// contructor value so for now a dummy value
const _ACTION_ID_DISAMBIGUATOR: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;

fn get_action_id(selector: b256) -> b256 {
    // Each external fn is dynamically assigned an action identifier as the hash of the disambiguator and the
    // fn selector. Disambiguation is necessary to avoid potential collisions in the fn selectors of
    // multiple contracts.
    let encode = abi_encode {
        _action_in_disambiguator: _ACTION_ID_DISAMBIGUATOR,
        selector: selector
    };
    return keccak256(encode);
}

/**
 * @dev Building block for performing access control on external functions.
 *
 * This contract is used via the `authenticate` modifier (or the `_authenticateCaller` fn), which can be applied
 * to external functions to only make them callable by authorized accounts.
 *
 * Derived contracts must implement the `_canPerform` fn, which holds the actual access control logic.
 */
impl Authentication for Contract {
    /**
     * @dev The main purpose of the `actionIdDisambiguator` is to prevent accidental fn selector collisions in
     * multi contract systems.
     *
     * There are two main uses for it:
     *  - if the contract is a singleton, any unique identifier can be used to make the associated action identifiers
     *    unique. The contract's own address is a good option.
     *  - if the contract belongs to a family that shares action identifiers for the same functions, an identifier
     *    shared by the entire family (and no other contract) should be used instead.
     */
    /**
     * @dev Reverts unless the caller is allowed to call this fn. Should only be applied to external functions.
     */

    /**
     * @dev Reverts unless the caller is allowed to call the entry point fn.
     */
    fn _authenticate_caller() {
        // let actionId: b256 = get_action_id(first_param());
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        // require(_can_perform(actionId, sender), SENDER_NOT_ALLOWED);
    }
}
