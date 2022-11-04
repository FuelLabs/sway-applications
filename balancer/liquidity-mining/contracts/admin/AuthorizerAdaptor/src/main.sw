
contract;

use std::{
    address::*,
    block::*,
    chain::auth::*,
    context::{*, call_frames::*},
    contract_id::ContractId,
    hash::keccak256,
    identity::Identity,
    option::Option,
    result::*,
    revert::{revert, require},
    storage::*,
    token::*,
    u128::U128,
    vec::Vec,
};

use BalancerErrors::*;

/**
 * @title Authorizer Adaptor
 * @notice This contract is intended to act as an adaptor between systems which expect a single admin address
 * and the Balancer Authorizer such that the Authorizer may grant/revoke admin powers to unlimited addresses.
 *
 * The permissions the Authorizer can grant are granular such they may be global or specific to a particular contract
 *
 * @dev When calculating the actionId to call a function on a target contract, it must be calculated as if it were
 * to be called on this adaptor. This can be done by passing the function selector to the `get_actionId` function.
 */

storage {
    _actionIdDisambiguator: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
    _vault: ContractId = ~ContractId::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),
    _authorizer: ContractId = ~ContractId::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),
}


/**
    * @notice Returns the Balancer Vault
    */
#[storage(read)]fn get_vault() -> ContractId {
    storage._vault
}

/**
    * @notice Returns the Authorizer
    */
#[storage(read)]fn get_authorizer() -> ContractId {
    storage._authorizer
}

#[storage(read)]fn _can_perform(
    actionId: b256,
    account: Address,
    to: Address
) -> bool {
    //todo 
    // get_authorizer().can_perform(actionId, account, to);
    true
}

/**
    * @notice Returns the action ID associated with calling a given function through this adaptor
    * @dev As the contracts managed by this adaptor don't have action ID disambiguators, we use the adaptor's globally.
    * This means that contracts with the same function selector will have a matching action ID:
    * if granularity is required then permissions must not be granted globally in the Authorizer.
    *
    * @param selector - The 4 byte selector of the function to be called using `perform_action`
    * @return The associated action ID
    */
#[storage(read)]fn get_actionId(selector: b256) -> b256 {
    keccak256((storage._actionIdDisambiguator, selector))

}

abi AuthorizerAdaptor {
    #[storage(read)]fn perform_action(target: Address, data: b256) -> b256;
}


impl AuthorizerAdaptor for Contract{


    /**
     * @notice Performs an arbitrary function call on a target contract, provided the caller is authorized to do so.
     * @param target - Address of the contract to be called
     * @param data - Calldata to be sent to the target contract
     * @return The bytes encoded return value from the performed function call
     */
    #[storage(read)]fn perform_action(target: Address, data: b256)
        -> (b256)
    {
        let selector:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

        // We want to check that the caller is authorized to call the function on the target rather than this function.
        // We must then pull the function selector from `data` rather than `msg.sig`. The most effective way to do this
        // is via assembly.
        // Note that if `data` is empty this will return an empty function signature (0x00000000)

        // solhint-disable-next-line no-inline-assembly
        asm(output: selector, r1) {

            // The function selector encoded in `data` has an offset relative to the start of msg.data of:
            // - 4 bytes due to the function selector for `perform_action`
            // - 3 words (3 * 32 = 96 bytes) for `target` and the length and offset of `data`
            // 96 + 4 = 100 bytes


            // r1 calldataload(100);
            sw output r1 i0; // store the word in r4 in output + 0 words

            output : b256
        }

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        require(_can_perform(get_actionId(selector),  sender, target), SENDER_NOT_ALLOWED);

        // We don't check that `target` is a contract so all calls to an EOA will succeed.
        // target.functionCallWithValue(data, msg_amount());
        selector

    }
}
