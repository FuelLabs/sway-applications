contract;

use std::{
    address::*,
    block::*,
    chain::auth::*,
    // chain::auth::{AuthError, msg_sender},
    context::{*, call_frames::*},
    contract_id::ContractId,
    hash::*,
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
// use Authentication::*;
use InputHelpers::ensure_input_length_match;

// use IVault::*; 

// use IAuthorizer::*; 
struct ScheduledExecution {
    recipient:Address,
    data:b256,
    executed:bool,
    cancelled:bool,
    protected:bool,
    executableAt:u64,
}

const recipient: Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
const data: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
const executed: bool = true;
const cancelled: bool = true;
const protected: bool = true;
const executableAt: u64 = 10;


storage {
    _pendingRoot:Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),
    _root:Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),

    Authentication: ContractId = ~ContractId::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),
    _executor: ContractId = ~ContractId::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),
    _vault: ContractId = ~ContractId::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff),
    scheduledExecution : ScheduledExecution = ScheduledExecution{
        recipient,
        data,
        executed,
        cancelled, 
        protected,
        executableAt
    },

    _scheduledExecutions: Vec<ScheduledExecution> = ~Vec::new(),

    grantActionId: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
    revokeActionId: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
    grantWhateverActionId: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
    revokeWhateverActionId: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,

    _isPermissionGranted: StorageMap<b256,bool> = StorageMap{},
    _delaysPerActionId: StorageMap<b256,u64> = StorageMap{},
}


abi TimelockAuthorizer {

    #[storage(read, write)]fn setPendingRoot(pendingRoot:Address);
    #[storage(read, write)]fn claimRoot();
    #[storage(read, write)]fn scheduleRootChange(newRoot: Address, executors: Vec<Address>) -> u64;
    #[storage(read, write)]fn scheduleDelayChange(actionId: b256, newDelay: u64, executors: Vec<Address>) -> u64;
    #[storage(read, write)]fn schedule(recipient: Address, data: b256,executors: Vec<Address>) -> u64;
    #[storage(read)]fn execute(scheduledExecutionId: u64) -> (b256);
    #[storage(read)]fn cancel(scheduledExecutionId: u64);
    #[storage(read, write)]fn manageGranter(actionId:b256, account:Address, recipient:Address, allowed:bool );
    #[storage(read, write)]fn grantPermissions( actionIds: Vec<b256>, account: Address, recipient: Vec<Address> );
    #[storage(read, write)]fn scheduleGrantPermission(actionId: b256, account: Address, recipient: Address, executors: Vec<Address>) -> u64;
    #[storage(read, write)]fn manageRevoker(actionId: b256, account: Address, recipient: Address, allowed: bool);
    #[storage(read, write)]fn revokePermissions(actionIds: Vec<b256>, account: Address, recipient: Vec<Address>);
    #[storage(read, write)]fn scheduleRevokePermission(actionId: b256, account: Address, recipient: Address, executors: Vec<Address>) -> (u64);
    #[storage(read, write)]fn renouncePermissions(actionId: Vec<b256>, recipient: Vec<Address>);

}


// use TimelockExecutor::*; 

const WHATEVER:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
const EVERYWHERE:Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
const MAX_DELAY:u64 = 730;

const GRANT_ACTION_ID:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff ;
const REVOKE_ACTION_ID:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff ;
const EXECUTE_ACTION_ID:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff ;
const SCHEDULE_DELAY_ACTION_ID:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff ;

const _GRANT_WHATEVER_ACTION_ID:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
const _REVOKE_WHATEVER_ACTION_ID:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

// const _root:Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff) ;
// const _pendingRoot:Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);

const _rootTransferDelay:u64 = 10;
const this: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
//change



// _grantPermission(grantWhateverActionId, admin, EVERYWHERE);
// _grantPermission(revokeWhateverActionId, admin, EVERYWHERE);

// GRANT_ACTION_ID = grantActionId;
// REVOKE_ACTION_ID = revokeActionId;
// EXECUTE_ACTION_ID = getActionId(TimelockAuthorizer.execute.selector);
// SCHEDULE_DELAY_ACTION_ID = getActionId(TimelockAuthorizer.scheduleDelayChange.selector);
// _GRANT_WHATEVER_ACTION_ID = grantWhateverActionId;
// _REVOKE_WHATEVER_ACTION_ID = revokeWhateverActionId;


/**
 * @dev Basic Authorizer implementation using timelocks.
 *
 * Users are allowed to perform actions if they have the permission to do so.
 *
 * This Authorizer implementation allows defining a delay per action identifier. If a delay is set for an action, users
 * are now allowed to schedule an execution that will be triggered in the future by the Authorizer instead of executing
 * it directly themselves.
 *
 * Glossary:
 * - Action: Op that can be performed to a target contract. These are identified by a unique bytes32 `actionId` defined
 *   by each target contract following `Authentication#getActionId`.
 * - Scheduled execution: The Authorizer can define different delays per `actionId` in order to determine that a
 *   specific time window must pass before these can be executed. When a delay is set for an `actionId`, executions
 *   must be scheduled. These executions are identified with an unsigned integer called `scheduledExecutionId`.
 * - Permission: Unique identifier to refer to a user (who) that is allowed to perform an action (what) in a specific
 *   target contract (recipient). This identifier is called `permissionId` and is computed as
 *   `keccak256(actionId, account, recipient)`.
 *
 * Permission granularity:
 *   In addition to the who/what/recipient of a permission, an extra notion of "how" is introduced to enable more granular
 *   configuration. This concept is used within the Authorizer to provide clarity among four ambiguous actions:
 *   granting/revoking permissions, executing scheduled actions, and setting action delays. For example, in managing
 *   the permission to set action delays, it is desirable to delineate whether an account can set delays for all
 *   actions indiscriminately or only for a specific action ID. In this case, the permission's "what" is the action
 *   ID for scheduling a delay change, and the "how" is the action ID for which the delay will be changed. The "what"
 *   and "how" of a permission are combined into a single `actionId` by computing `keccak256(what, how)`.
 
 */



//helper functions

#[storage(read)]fn onlyExecutor() -> bool{
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
    (sender == ~Address::from(storage._executor.into()))
}

/**
    * @dev Returns true if `account` is the root.
    */
fn isRoot(account : Address) -> bool {
    account == storage._root;
}

/**
    * @dev Returns true if `account` is the pending root.
    */
fn isPendingRoot(account: Address) -> bool {
    account == storage._pendingRoot;
}

/**
    * @dev Returns the delay required to transfer the root address.
    */
fn getRootTransferDelay() -> u64 {
    _rootTransferDelay;
}

/**
    * @dev Returns the vault address.
    */
fn getVault() -> Address {
    ~Address::from(_vault);
}

/**
    * @dev Returns the executor address.
    */
#[storage(read)]fn getExecutor() -> Address {
    ~Address::from(storage._executor.into());
}

/**
    * @dev Returns the root address.
    */
fn getRoot() -> Address {
    storage._root
}

/**
    * @dev Returns the currently pending new root address.
    */
fn getPendingRoot() -> Address {
    storage._pendingRoot
}

/**
    * @dev Returns the action ID for function selector `selector`.
    */
// change
fn getActionId(selector:b256) -> b256 {
    // return keccak256(abi.encodePacked(bytes32(uint256(address(this))), selector));
    //remove
    let ret = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    ret
}

/**
    * @dev Returns the action ID for action `actionId` with specific params `how`.
    */
// change
fn getActionIdTwo( actionId:b256, how:b256 ) -> b256 {
    // return keccak256(abi.encodePacked(actionId, how));
    let ret = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    ret
}

/**
    * @dev Returns the execution delay for action `actionId`.
    */
#[storage(read)]fn getActionIdDelay(actionId: b256) -> u64 {
    storage._delaysPerActionId.get(actionId);
}

/**
    * @dev Returns the permission ID for action `actionId`, account `account` and target `recipient`.
    */


fn permissionId(
    actionId : b256,
    account : Address,
    recipient : Address
) -> b256{
    // return keccak256(abi.encodePacked(actionId, account, recipient));
    let ret = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    ret
}

/**
    * @dev Returns true if `account` has the permission defined by action `actionId` and target `recipient`.
    * This function is specific for the strict permission defined by the tuple `(actionId, recipient)`, `account` may also
    * hold the global permission for the action `actionId` which would allow them to perform this action on `recipient`.
    * For this reason, it's recommended to use `hasPermission` if checking whether `account` is allowed to perform
    * a given action.
    */
#[storage(read)]fn isPermissionGrantedOnTarget(
    actionId:b256,
    account:Address,
    recipient:Address
) -> bool {
    storage._isPermissionGranted.get(permissionId(actionId, account, recipient));
}

/**
    * @dev Returns true if `account` is allowed to perform action `actionId` in target `recipient`.
    */
#[storage(read)]fn hasPermission(
    actionId:b256,
    account:Address,
    recipient:Address
) -> bool {
        storage._isPermissionGranted.get(permissionId(actionId, account, recipient)) ||
        storage._isPermissionGranted.get(permissionId(actionId, account, EVERYWHERE));
}

/**
    * @dev Returns true if `account` is allowed to grant permissions for action `actionId` in target `recipient`.
    */
#[storage(read)]fn isGranter(
    actionId:b256,
    account:Address,
    recipient:Address
) -> bool {
    _hasPermissionOrWhatever(GRANT_ACTION_ID, account, recipient, actionId);
}

/**
    * @dev Returns true if `account` is allowed to revoke permissions for action `actionId` in target `recipient`.
    */
#[storage(read)]fn isRevoker(
    actionId:b256,
    account:Address,
    recipient:Address
) -> bool {
    _hasPermissionOrWhatever(REVOKE_ACTION_ID, account, recipient, actionId);
}

/**
    * @dev Returns true if `account` can perform action `actionId` in target `recipient`.
    */
#[storage(read)]fn canPerform(
    actionId:b256,
    account:Address,
    recipient:Address
) -> bool {
        if storage._delaysPerActionId.get(actionId) > 0 { 
            account == ~Address::from(storage._executor.into()) 
        } 
        else {
            hasPermission(actionId, account, recipient)
        }
}

/**
    * @dev Returns true if `account` can grant permissions for action `actionId` in target `recipient`.
    */
#[storage(read, write)]fn canGrant(
    actionId:b256,
    account:Address,
    recipient:Address
) -> bool {
    _canPerformOrWhatever(GRANT_ACTION_ID, account, recipient, actionId);
}

/**
    * @dev Returns true if `account` can revoke permissions for action `actionId` in target `recipient`.
    */
#[storage(read, write)]fn canRevoke(
    actionId:b256,
    account:Address,
    recipient:Address
) -> bool {
    _canPerformOrWhatever(REVOKE_ACTION_ID, account, recipient, actionId);
}

/**
    * @dev Returns the scheduled execution `scheduledExecutionId`.
    */
//change
#[storage(read)]fn getScheduledExecution(scheduledExecutionId:u64) -> ScheduledExecution {
    storage._scheduledExecutions.get(scheduledExecutionId).unwrap()
}

/**
    * @dev Returns true if execution `scheduledExecutionId` can be executed.
    * Only true if it is not already executed or cancelled, and if the execution delay has passed.
    */
    //change
#[storage(read)]fn canExecute(scheduledExecutionId:u64) -> bool {
    require(scheduledExecutionId < storage._scheduledExecutions.len(), "ACTION_DOES_NOT_EXIST");
    storage.scheduledExecution = storage._scheduledExecutions.get(scheduledExecutionId).unwrap();
    return
        !storage.scheduledExecution.executed &&
        !storage.scheduledExecution.cancelled &&
        block_timestamp >= storage.scheduledExecution.executableAt;
    // solhint-disable-previous-line not-rely-on-time
}


// /**
//     * @notice Sets the pending root address to `pendingRoot`.
//     * @dev Once set as the pending root, `pendingRoot` may then call `claimRoot` to become the new root.
//     */

#[storage(write)]fn _setPendingRoot(pendingRoot:Address)  {
    storage._pendingRoot = pendingRoot;
    // emit PendingRootSet(pendingRoot);
}

   
fn _setRoot(root:Address) {
    storage._root = root;
    // emit RootSet(root);
}

/**
    * @dev Sets a new delay `delay` for action `actionId`.
    */
#[storage(read, write)]fn setDelay( actionId:b256, delay:u64) {
    require(onlyExecutor(), true);
    let setAuthorizerActionId:b256 = _vault.getActionId(setAuthorizer.selector);
    let isAllowed:bool = actionId == setAuthorizerActionId || delay <= storage._delaysPerActionId.get(setAuthorizerActionId);
    
    require(isAllowed, "DELAY_EXCEEDS_SET_AUTHORIZER");

    storage._delaysPerActionId.insert(actionId, delay);
    // emit ActionDelaySet(actionId, delay);
}

fn _decodeSelector(data: b256) -> b256 {
    // The bytes4 type is left-aligned and padded with zeros: we make use of that property to build the selector
    
    // if (data.len() < 4) return bytes4(0);
    // return bytes4(data[0]) | (bytes4(data[1]) >> 8) | (bytes4(data[2]) >> 16) | (bytes4(data[3]) >> 24);
    let bdata: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    bdata;
}


// private functions

#[storage(read, write)]fn _grantPermission(
    actionId: b256,
    account: Address,
    recipient: Address,
)  {
    let permission: b256 = permissionId(actionId, account, recipient);
    if !storage._isPermissionGranted.get(permission) {
        storage._isPermissionGranted.insert(permission, true);
        // emit PermissionGranted(actionId, account, recipient);
    }
}

#[storage(read, write)]fn _revokePermission(
    actionId: b256,
    account: Address,
    recipient: Address,
)  {
    let permission: b256 = permissionId(actionId, account, recipient);
    if (storage._isPermissionGranted.get(permission)) {
        storage._isPermissionGranted.insert(permission, false);
        // emit PermissionRevoked(actionId, account, recipient);
    }
}

#[storage(read, write)]fn _schedule(
    actionId: b256,
    recipient: Address,
    data: b256,
    executors: Vec<Address>
) -> u64 {
    let delay: u64 = storage._delaysPerActionId.get(actionId);
    require(delay > 0, "CANNOT_SCHEDULE_ACTION");
    _scheduleWithDelay(actionId, recipient, data, delay, executors);
}


#[storage(read, write)]fn _scheduleWithDelay(
    actionId: b256,
    recipient: Address,
    data: b256,
    delay: u64,
    executors: Vec<Address>
) -> u64 {
    let scheduledExecutionId:u64 = storage._scheduledExecutions.len();
    // emit ExecutionScheduled(actionId, scheduledExecutionId);

    // solhint-disable-next-line not-rely-on-time

    let executableAt: u64 = block_timestamp + delay;
    let protected: bool = executors.len() > 0;
    let executed: bool = false;
    let cancelled: bool = false;
    storage._scheduledExecutions.push(ScheduledExecution{recipient, data, executed, cancelled, protected, executableAt});

    let executeActionId: b256 = getActionIdTwo(EXECUTE_ACTION_ID, bytes32(scheduledExecutionId));
    let mut i = 0;
    while i < executors.len() {
        _grantPermission(executeActionId, executors.get(i).unwrap(), ~Address::from(this));
        i = i + 1;
    }
}

/*
    function _ar(bytes32 item) private pure returns (bytes32[] memory result) {
        result = new bytes32[](1);
        result[0] = item;
    
    }

    function _ar(address item) private pure returns (address[] memory result) {
        result = new address[](1);
        result[0] = item;
    }
*/

fn _ar(item: b256) -> (Vec<b256>) {
    let mut result = ~Vec::new();
    result.push(item);
    result
}

fn _ar(item:Address) -> (Vec<Address>) {
    let mut result = ~Vec::new();
    result.push(item);
    result
}


// internal functions

#[storage(read)]fn _hasPermissionOrWhatever(
    actionId: b256,
    account: Address,
    recipient: Address,
    how: b256
) -> (bool) {
    let granularActionId: b256 = getActionIdTwo(actionId, how);
    let globalActionId: b256 = getActionIdTwo(actionId, WHATEVER);
    hasPermission(granularActionId, account, recipient) || hasPermission(globalActionId, account, recipient);
}


#[storage(read,write)]fn _canPerformOrWhatever(
    actionId: b256,
    account: Address,
    recipient: Address,
    how: b256
) -> bool {
    // If there is a delay defined for the granular action ID, then the sender must be the authorizer (scheduled
    // execution)
    let granularActionId: b256 = getActionIdTwo(actionId, how);
    if storage._delaysPerActionId.get(granularActionId) > 0 {
        account == ~Address::from(storage._executor.into());
    }

    // If there is no delay, we check if the account has that permission
    if hasPermission(granularActionId, account, recipient) {
        true;
    }

    // If the account doesn't have the explicit permission, we repeat for the global permission
    let globalActionId: b256 = getActionIdTwo(actionId, WHATEVER);
    canPerform(globalActionId, account, recipient);
}



impl TimelockAuthorizer for Contract {



    /**
        * @notice Sets the pending root address to `pendingRoot`.
        * @dev Once set as the pending root, `pendingRoot` may then call `claimRoot` to become the new root.
        */
    #[storage(read, write)]fn setPendingRoot(pendingRoot:Address) {
        require(onlyExecutor(), true);
        _setPendingRoot(pendingRoot);
    }


    /**
    * @notice Transfers root powers from the current to the pending root address.
    * @dev This function prevents accidentally transferring root to an invalid address.
    * To become root, the pending root must call this function to ensure that it's able to interact with this contract.
    */
    #[storage(read, write)]fn claimRoot() {
        let currentRoot:Address = storage._root;
        let pendingRoot:Address = storage._pendingRoot;
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        require(sender == pendingRoot, SENDER_NOT_ALLOWED);

        // Grant powers to new root to grant or revoke any permission over any contract.
        _grantPermission(_GRANT_WHATEVER_ACTION_ID, pendingRoot, EVERYWHERE);
        _grantPermission(_REVOKE_WHATEVER_ACTION_ID, pendingRoot, EVERYWHERE);

        // Revoke these powers from the outgoing root.
        _revokePermission(_GRANT_WHATEVER_ACTION_ID, currentRoot, EVERYWHERE);
        _revokePermission(_REVOKE_WHATEVER_ACTION_ID, currentRoot, EVERYWHERE);

        // Complete the root transfer and reset the pending root.
        _setRoot(pendingRoot);
        _setPendingRoot(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000));
    }



    /**
     * @dev Schedules an execution to change the root address to `newRoot`.
     */
    //change
    #[storage(read, write)]fn scheduleRootChange(
        newRoot: Address,
        executors: Vec<Address>
        )
        -> (u64)
    {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        require(isRoot(sender), SENDER_NOT_ALLOWED);
        let actionId: b256 = getActionId(this.setPendingRoot.selector);
        // let data: b256 = abi.encodeWithSelector(this.setPendingRoot.selector, newRoot);
        let data: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
        return _scheduleWithDelay(actionId, ~Address::from(this), data, getRootTransferDelay(), executors);
    }

 

    /**
     * @dev Schedules an execution to set action `actionId`'s delay to `newDelay`.
     */
    //change

    #[storage(read, write)]fn scheduleDelayChange(
        actionId: b256,
        newDelay: u64,
        executors: Vec<Address>
    ) -> u64 {

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };


        require(newDelay <= MAX_DELAY, "DELAY_TOO_LARGE");
        require(isRoot(sender), SENDER_NOT_ALLOWED);

        // The delay change is scheduled to execute after the current delay for the action has elapsed. This is
        // critical, as otherwise it'd be possible to execute an action with a delay shorter than its current one
        // by first changing it to a smaller (or zero) value.

        let actionDelay: u64 = storage._delaysPerActionId.get(actionId);
        let scheduleDelayActionId: b256 = getActionIdTwo(SCHEDULE_DELAY_ACTION_ID, actionId);

        // let data: 256 = abi.encodeWithSelector(this.setDelay.selector, actionId, newDelay);
        let data: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
        _scheduleWithDelay(scheduleDelayActionId, ~Address::from(this), data, actionDelay, executors);
        
    }

    /**
     * @dev Schedules an arbitrary execution of `data` in target `recipient`.
     */
    #[storage(read, write)]fn schedule(
        recipient: Address,
        data: b256,
        executors: Vec<Address>
    ) -> u64 {

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };


        require(recipient != ~Address::from(this), "CANNOT_SCHEDULE_AUTHORIZER_ACTIONS");
        let actionId: b256 = Authentication(recipient).getActionId(_decodeSelector(data));
        require(hasPermission(actionId, sender, recipient), SENDER_NOT_ALLOWED);
        return _schedule(actionId, recipient, data, executors);
    }

    /**
     * @dev Executes a scheduled action `scheduledExecutionId`.
     */
    #[storage(read)]fn execute(scheduledExecutionId: u64) -> (b256) {
        require(scheduledExecutionId < storage._scheduledExecutions.len(), "ACTION_DOES_NOT_EXIST");
        storage.scheduledExecution = storage._scheduledExecutions.get(scheduledExecutionId).unwrap();
        require(!storage.scheduledExecution.executed, "ACTION_ALREADY_EXECUTED");
        require(!storage.scheduledExecution.cancelled, "ACTION_ALREADY_CANCELLED");

        // solhint-disable-next-line not-rely-on-time
        require(block_timestamp >= storage.scheduledExecution.executableAt, "ACTION_NOT_EXECUTABLE");

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        if (storage.scheduledExecution.protected) {
            let executeScheduledActionId: b256 = getActionIdTwo(EXECUTE_ACTION_ID, bytes32(scheduledExecutionId));
            let isAllowed: bool = hasPermission(executeScheduledActionId, sender, ~Address::from(this));
            require(isAllowed, SENDER_NOT_ALLOWED);
        }

        storage.scheduledExecution.executed = true;
        let result: b256 = (storage._executor.into()).execute(storage.scheduledExecution.recipient, storage.scheduledExecution.data);

        result
        // emit ExecutionExecuted(scheduledExecutionId);
    }

    /**
     * @dev Cancels a scheduled action `scheduledExecutionId`.
     */
    #[storage(read)]fn cancel(scheduledExecutionId: u64)
        {
        require(scheduledExecutionId < storage._scheduledExecutions.len(), "ACTION_DOES_NOT_EXIST");
        storage.scheduledExecution = storage._scheduledExecutions.get(scheduledExecutionId).unwrap();

        require(!storage.scheduledExecution.executed, "ACTION_ALREADY_EXECUTED");
        require(!storage.scheduledExecution.cancelled, "ACTION_ALREADY_CANCELLED");

        // The permission to cancel a scheduled action is the same one used to schedule it
        let target: ContractId = Authentication(storage.scheduledExecution.recipient);
        let actionId: b256 = target.getActionId(_decodeSelector(storage.scheduledExecution.data));
        let sender: Result<Identity, AuthError> = msg_sender();

        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        require(hasPermission(actionId, sender, storage.scheduledExecution.recipient), SENDER_NOT_ALLOWED);

        storage.scheduledExecution.cancelled = true;
        // emit ExecutionCancelled(scheduledExecutionId);
    }

    /**
     * @dev Sets `account`'s granter status to `allowed` for action `actionId` in target `recipient`.
     * Note that granters can revoke the granter status of other granters, even banning the root.
     * However, the root can always rejoin, and then revoke any malicious granters.
     */
    #[storage(read, write)]fn manageGranter(
        actionId:b256,
        account:Address,
        recipient:Address,
        allowed:bool
    ) {
        // Root may grant or revoke granter status from any 
        //:Address Granters may only revoke a granter status from any address.
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        let isAllowed:bool = isRoot(sender) || (!allowed && isGranter(actionId, sender, recipient));
        require(isAllowed, SENDER_NOT_ALLOWED);

        let  grantPermissionsActionId = getActionIdTwo(GRANT_ACTION_ID, actionId);
        if allowed {
            _grantPermission(grantPermissionsActionId, account, recipient);
        } 
        else {
            _revokePermission(grantPermissionsActionId, account, recipient);
        }
    }

    /**
     * @dev Grants multiple permissions to a single `account`.
     */
    #[storage(read, write)]fn grantPermissions(
        actionIds: Vec<b256>,
        account: Address,
        recipient: Vec<Address>
    )  {
        ensure_input_length_match(actionIds.len(), recipient.len());

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        let mut i:u32 = 0;
        while i < actionIds.len() {
            require(canGrant(actionIds.get(i).unwrap(), sender, recipient.get(i).unwrap()), SENDER_NOT_ALLOWED);
            _grantPermission(actionIds.get(i).unwrap(), account, recipient.get(i).unwrap());
            i = i + 1;
        }
    }

    /**
     * @dev Schedules a grant permission to `account` for action `actionId` in target `recipient`.
     */
     //change 570
    #[storage(read, write)]fn scheduleGrantPermission(
        actionId: b256,
        account: Address,
        recipient: Address,
        executors: Vec<Address>
    ) -> u64 {

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        require(isGranter(actionId, sender, recipient), SENDER_NOT_ALLOWED);

        // let data: b256 = abi.encodeWithSelector(this.grantPermissions.selector, _ar(actionId), account, _ar(recipient));
        let data: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

        let grantPermissionId:b256 = getActionIdTwo(GRANT_ACTION_ID, actionId);
        _schedule(grantPermissionId, ~Address::from(this), data, executors);
    }

    /**
     * @dev Sets `account`'s revoker status to `allowed` for action `actionId` in target `recipient`.
     * Note that revokers can revoke the revoker status of other revokers, even banning the root.
     * However, the root can always rejoin, and then revoke any malicious revokers.
     */
    #[storage(read, write)]fn manageRevoker(
        actionId: b256,
        account: Address,
        recipient: Address,
        allowed: bool
    )
    {
        // Root may grant or revoke revoker status from any address.
        // Revokers may only revoke a revoker status from any address.

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        let isAllowed: bool = isRoot(sender) || (!allowed && isRevoker(actionId, sender, recipient));
        require(isAllowed, SENDER_NOT_ALLOWED);

        let revokePermissionsActionId: b256 = getActionIdTwo(REVOKE_ACTION_ID, actionId);
        if allowed { 
            _grantPermission(revokePermissionsActionId, account, recipient);
        }
        else {
            _revokePermission(revokePermissionsActionId, account, recipient);
        };
    }

    /**
     * @dev Revokes multiple permissions from a single `account`.
     */
    #[storage(read, write)]fn revokePermissions(
        actionIds: Vec<b256>,
        account: Address,
        recipient: Vec<Address>
    )
    {
        ensure_input_length_match(actionIds.len(), recipient.len());

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        let mut i:u32 = 0;
        while i < actionIds.len() {
            require(canRevoke(actionIds.get(i).unwrap(), sender, recipient.get(i).unwrap()), SENDER_NOT_ALLOWED);
            _revokePermission(actionIds.get(i).unwrap(), account, recipient.get(i).unwrap());
            i = i + 1;
        }
    }

    /**
     * @dev Schedules a revoke permission from `account` for action `actionId` in target `recipient`.
     */
     /// change
    #[storage(read, write)]fn scheduleRevokePermission(
        actionId: b256,
        account: Address,
        recipient: Address,
        executors: Vec<Address>
    ) -> (u64) {

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        require(isRevoker(actionId, sender, recipient), SENDER_NOT_ALLOWED);
        // let data: b256 = abi.encodeWithSelector(this.revokePermissions.selector, _ar(actionId), account, _ar(recipient));
        let data: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

        let revokePermissionId: b256 = getActionIdTwo(REVOKE_ACTION_ID, actionId);
        _schedule(revokePermissionId, ~Address::from(this), data, executors);
    }

    /**
     * @dev Revokes multiple permissions from the caller.
     */
    #[storage(read, write)]fn renouncePermissions(
        actionIds: Vec<b256>,
        recipient: Vec<Address>
    ) 
    {

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        ensure_input_length_match(actionIds.len(), recipient.len());
        let mut i: u32 = 0;
        while i < actionIds.len() {
            _revokePermission(actionIds.get(i).unwrap(), sender, recipient.get(i).unwrap());
            i = i + 1;
        }
    }


}
