contract;

use std::{
    contract_id::ContractId,
    address::Address,
    option::Option,
    assert::assert ,
    vec::Vec,
    context::balance_of,
    chain::auth::{AuthError, msg_sender},
    identity::Identity,
    result::*,
    context::call_frames::contract_id,
    storage::{StorageMap,get, store},
    revert::{revert, require},
};


// abi MyContract {
//     fn test_function() -> bool;
// }


/**
 * @dev Contract module that allows children to implement role-based access
 * control mechanisms.
 *
 * Roles are referred to by their `bytes32` identifier. These should be exposed
 * in the external API and be unique. The best way to achieve this is by
 * using `public constant` hash digests:
 *
 * ```
 * bytes32 public constant MY_ROLE = keccak256("MY_ROLE");
 * ```
 *
 * Roles can be used to represent a set of permissions. To restrict access to a
 * fn call, use {has_role}:
 *
 * ```
 * fn foo() public {
 *     require(has_role(MY_ROLE, msg.sender));
 *     ...
 * }
 * ```
 *
 * Roles can be granted and revoked dynamically via the {grant_role} and
 * {revoke_role} functions. Each role has an associated admin role, and only
 * accounts that have a role's admin role can call {grant_role} and {revoke_role}.
 *
 * By default, the admin role for all roles is `DEFAULT_ADMIN_ROLE`, which means
 * that only accounts with this role will be able to grant or revoke other
 * roles. More complex role relationships can be created by using
 * {_set_role_admin}.
 *
 * WARNING: The `DEFAULT_ADMIN_ROLE` is also its own admin: it has permission to
 * grant and revoke this role. Extra precautions should be taken to secure
 * accounts that have been granted it.
 */

struct AddressSet {
    // Storage of set values
    _values: Vec<Address>,
    // Position of the value in the `values` array, plus 1 because index 0
    // means a value is not in the set.
    _indexes: StorageMap<Address, u64>,
}

struct RoleData {
    members: AddressSet,
    adminRole: b256,
}

storage {
    _roles: StorageMap<b256, RoleData> = StorageMap{},
}


const DEFAULT_ADMIN_ROLE: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;


/**
    * @dev Returns `true` if `account` has been granted `role`.
    */
#[storage(read)]
fn has_role(role: b256, account: Address) -> bool {
    let mut count = 0;
    while count < storage._roles.get(role).members._values.len() {
        if storage._roles.get(role).members._values.get(count).unwrap() == account {
            return true;
        }
        count = count + 1;
    }
    return false;

}

/**
    * @dev Returns the number of accounts that have `role`. Can be used
    * together with {get_role_member} to enumerate all bearers of a role.
    */
// #[storage(read)]
// fn get_role_member_count(role: b256) -> u64 {
//     return storage._roles.get(role).members.len();
// }

/**
    * @dev Returns one of the accounts that have `role`. `index` must be a
    * value between 0 and {get_role_member_count}, non-inclusive.
    *
    * Role bearers are not sorted in any particular way, and their ordering may
    * change at any point.
    *
    * WARNING: When using {get_role_member} and {get_role_member_count}, make sure
    * you perform all queries on the same block. See the following
    * https://forum.openzeppelin.com/t/iterating-over-elements-on-enumerableset-in-openzeppelin-contracts/2296[forum post]
    * for more information.
    */
// #[storage(read)]
// fn get_role_member(role: b256, index: u64) -> Address {
//     return storage._roles.get(role).members.at(index);
// }

/**
    * @dev Returns the admin role that controls `role`. See {grant_role} and
    * {revoke_role}.
    *
    * To change a role's admin, use {_set_role_admin}.
    */
#[storage(read)]
fn get_role_admin(role: b256) -> b256 {
    return storage._roles.get(role).adminRole;
}

/**
    * @dev Grants `role` to `account`.
    *
    * If `account` had not been already granted `role`, emits a {RoleGranted}
    * event.
    *
    * Requirements:
    *
    * - the caller must have ``role``'s admin role.
    */
#[storage(read)]
fn grant_role(role: b256, account: Address) {
    let sender: Result<Identity, AuthError> = msg_sender();
    let sender: Address = match sender.unwrap() {
        Identity::Address(addr) => {
            addr
        },
        _ => {
            revert(0);
        },
    };
    require(has_role(storage._roles.get(role).adminRole, sender), "GRANT_SENDER_NOT_ADMIN");

    // _grantRole(role, account);
}

/**
    * @dev Revokes `role` from `account`.
    *
    * If `account` had already been granted `role`, emits a {RoleRevoked} event.
    *
    * Requirements:
    *
    * - the caller must have ``role``'s admin role.
    */
#[storage(read)]
fn revoke_role(role: b256, account: Address) {
    let sender: Result<Identity, AuthError> = msg_sender();
    let sender: Address = match sender.unwrap() {
        Identity::Address(addr) => {
            addr
        },
        _ => {
            revert(0);
        },
    };
    require(has_role(storage._roles.get(role).adminRole, sender), "REVOKE_SENDER_NOT_ADMIN");
}

/**
    * @dev Revokes `role` from the calling account.
    *
    * Roles are often managed via {grant_role} and {revoke_role}: this fn's
    * purpose is to provide a mechanism for accounts to lose their privileges
    * if they are compromised (such as when a trusted device is misplaced).
    *
    * If the calling account had been granted `role`, emits a {RoleRevoked}
    * event.
    *
    * Requirements:
    *
    * - the caller must be `account`.
    */
#[storage(read)]
fn renounce_role(role: b256, account: Address) {
    let sender: Result<Identity, AuthError> = msg_sender();
    let sender: Address = match sender.unwrap() {
        Identity::Address(addr) => {
            addr
        },
        _ => {
            revert(0);
        },
    };
    require(account == sender, "RENOUNCE_SENDER_NOT_ALLOWED");
}

/**
    * @dev Grants `role` to `account`.
    *
    * If `account` had not been already granted `role`, emits a {RoleGranted}
    * event. Note that unlike {grant_role}, this fn doesn't perform any
    * checks on the calling account.
    *
    * [WARNING]
    * ====
    * This fn should only be called from the constructor when setting
    * up the initial roles for the system.
    *
    * Using this fn in any other way is effectively circumventing the admin
    * system imposed by {AccessControl}.
    * ====
    */
/**
    * @dev Sets `adminRole` as ``role``'s admin role.
    *
    * Emits a {RoleAdminChanged} event.
    */
#[storage(read, write)]
fn _set_role_admin(role: b256, adminRole: b256) {
    let tmp = RoleData {
        members: storage._roles.get(role).members,
        adminRole: adminRole,
    };
    storage._roles.insert(role, tmp);
}



