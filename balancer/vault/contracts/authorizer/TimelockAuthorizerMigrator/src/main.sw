
contract;

use std::{
    address::*,

    block::*,
    chain::auth::*,
    context::{*, call_frames::*},
    contract_id::ContractId,
    hash::*,
    result::*,
    revert::{revert, require},
    storage::*,
    token::*,
    u128::U128,
    vec::Vec,
};


use math::{
    min
};

const WHATEVER : b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
const EVERYWHERE : Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
const CHANGE_ROOT_DELAY : u32 = 7 ;
const DEFAULT_ADMIN_ROLE : b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;


// const vault: IVault = ;
const root: Address = ~Address::from(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
// const oldAuthorizer: IBasicAuthorizer = ;
// const newAuthorizer: TimelockAuthorizer = ;

const rootChangeExecutionId: u64 = 0;



struct RoleData {
    grantee: Address,
    role: b256,
    target: Address
}

storage {
    _roleMigrationComplete: bool = true,

    revokersData: Vec<RoleData> = ~Vec::new(),
    rolesData: Vec<RoleData> = ~Vec::new(),
    grantersData: Vec<RoleData> = ~Vec::new(),

    existingRolesMigrated:u64 = 0,
    grantersMigrated: u64 = 0,
    revokersMigrated: u64 = 0,
}

abi TimelockAuthorizerMigrator {
    #[storage(read, write)]fn migrate(rolesToMigrate : u64);
    #[storage(read, write)]fn finalizeMigration();
}



#[storage(read)]fn isComplete() -> bool {
    _roleMigrationComplete;
}
/*
    * @notice Migrates to TimelockAuthorizer by setting up roles from the old Authorizer and new granters/revokers.
    * @dev Attempting to migrate roles in excess of the amount of unmigrated roles of any particular type results in
    * all remaining roles of that type being migrated. The unused role migrations will then flow over into the next
    * "role type".
    * @param rolesToMigrate The number of permissions to set up on the new TimelockAuthorizer.
    */

#[storage(read, write)]fn _migrate( rolesToMigrate : u64)  {
    // Each function returns the amount of unused role migrations which is then fed into the next function.
    let mut newRolesToMigrate: u64 = rolesToMigrate; 
    newRolesToMigrate = _migrateExistingRoles(newRolesToMigrate);
    newRolesToMigrate = _setupGranters(newRolesToMigrate);
    _setupRevokers(newRolesToMigrate);

    // As we set up the revoker roles last we can use them to determine whether the full migration is complete.
    if storage.revokersMigrated >= storage.revokersData.len() {
        storage._roleMigrationComplete = true;
    }
}

/**
    * @notice Migrates listed roles from the old Authorizer to the new TimelockAuthorizer.
    * @dev Attempting to migrate roles in excess of the unmigrated roles results in all remaining roles being migrated.
    * The amount of unused role migrations is then returned so they can be used to perform the next migration step.
    * @param rolesToMigrate - The desired number of roles to migrate (may exceed the remaining unmigrated roles).
    * @return remainingRolesToMigrate - The amount of role migrations which were unused in this function.
    */

#[storage(read, write)]fn _migrateExistingRoles(rolesToMigrate:u64) -> u64 {
    let i: u64 = storage.existingRolesMigrated;
    let to: u64 = Math.min(i + rolesToMigrate, storage.rolesData.len());
    let remainingRolesToMigrate: u64 = (i + rolesToMigrate) - to;

    while i < to {
        let roleData: RoleData = storage.rolesData.get(i).unwrap();
        newAuthorizer.grantPermissions(_arr(roleData.role), roleData.grantee, _arrAddress(roleData.target));
    }

    storage.existingRolesMigrated = i;

    remainingRolesToMigrate
}
/**
    * @notice Sets up granters for the listed roles on the new TimelockAuthorizer.
    * @dev Attempting to migrate roles in excess of the unmigrated roles results in all remaining roles being migrated.
    * The amount of unused role migrations is then returned so they can be used to perform the next migration step.
    * @param rolesToMigrate - The desired number of roles to migrate (may exceed the remaining unmigrated roles).
    * @return remainingRolesToMigrate - The amount of role migrations which were unused in this function.
    */
#[storage(read, write)]fn _setupGranters(rolesToMigrate:u64) -> u64 {
    let mut i = storage.grantersMigrated;
    let to:u64 = min(i + rolesToMigrate, storage.grantersData.len());
    let remainingRolesToMigrate = (i + rolesToMigrate) - to;

    while i<to {
        let granterData: RoleData = storage.grantersData.get(i).unwrap();
        newAuthorizer.manageGranter(granterData.role, granterData.grantee, granterData.target, true);
        i = i + 1;
    }

    storage.grantersMigrated = i;
    remainingRolesToMigrate
}


/**
    * @notice Sets up revokers for the listed roles on the new TimelockAuthorizer.
    * @dev Attempting to migrate roles in excess of the unmigrated roles results in all remaining roles being migrated.
    * @param rolesToMigrate - The desired number of roles to migrate (may exceed the remaining unmigrated roles).
    */
#[storage(read, write)]fn _setupRevokers(rolesToMigrate:u64)  {
    let mut i: u64 = storage.revokersMigrated;
    let to = Math.min(i + rolesToMigrate, storage.revokersData.len());

    while i < to {
        let revokerData: RoleData = storage.revokersData.get(i).unwrap();
        newAuthorizer.manageRevoker(revokerData.role, revokerData.grantee, revokerData.target, true);
        i = i + 1;
    }

    storage.revokersMigrated = i;
}

#[storage(read)]fn _afterMigrate() {
    // Execute only once after the migration ends
    if !isComplete() {return};

    // Finally trigger the first step of transferring root ownership over the TimelockAuthorizer to `root`.
    // Before the migration can be finalized, `root` must call `claimRoot` on the `TimelockAuthorizer`.
    require(newAuthorizer.canExecute(rootChangeExecutionId), "CANNOT_TRIGGER_ROOT_CHANGE_YET");
    newAuthorizer.execute(rootChangeExecutionId);
}


fn _arr(a : b256) -> (Vec<b256>) {
let mut arr = ~Vec::new();
arr.push(a);
arr
}

fn _arrAddress( a : Address) -> (Vec<Address>) {
let mut arr = ~Vec::new();
arr.push(a);
arr
}




impl TimelockAuthorizerMigrator for Contract {



    // let granterData: Vec<RoleData> = grantersData[i];


     /*

    * @dev Reverts if _rolesData contains a role for an account which doesn't hold the same role on the old Authorizer.

    struct RoleData {
        address grantee;
        bytes32 role;
        address target;
    }
  
    constructor(
        IVault _vault,
        address _root,
        IBasicAuthorizer _oldAuthorizer,
        RoleData[] memory _rolesData,
        RoleData[] memory _grantersData,
        RoleData[] memory _revokersData
    ) {

        TimelockAuthorizer _newAuthorizer = new TimelockAuthorizer(address(this), _vault, CHANGE_ROOT_DELAY);
        newAuthorizer = _newAuthorizer;
        oldAuthorizer = _oldAuthorizer;
        root = _root;
        vault = _vault;

        for (uint256 i = 0; i < _rolesData.length; i++) {
            RoleData memory roleData = _rolesData[i];

            require(_oldAuthorizer.canPerform(roleData.role, roleData.grantee, roleData.target), "UNEXPECTED_ROLE");
            rolesData.push(roleData);
        }
        for (uint256 i = 0; i < _grantersData.length; i++) {

            grantersData.push(_grantersData[i]);
        }
        for (uint256 i = 0; i < _revokersData.length; i++) {
            revokersData.push(_revokersData[i]);
        }

        rootChangeExecutionId = _newAuthorizer.scheduleRootChange(_root, _arr(address(this)));
    }

    */



    #[storage(read, write)]fn migrate( rolesToMigrate : u64)  {
        // require!(!isComplete(), "MIGRATION_COMPLETE");
        _migrate(rolesToMigrate);
        _afterMigrate();
    }

    #[storage(read, write)]fn finalizeMigration()  {
        require(isComplete(), "MIGRATION_NOT_COMPLETE");
        // Safety check to avoid us migrating to a authorizer with an invalid root.
        // `root` must call `claimRoot` on `newAuthorizer` in order for us to set it on the Vault.
        require(newAuthorizer.isRoot(root), "ROOT_NOT_CLAIMED_YET");

        // Ensure the migrator contract has authority to change the vault's authorizer

        // let setAuthorizerId: b256 = IAuthentication(~Address::from(vault)).getActionId(IVault::setAuthorizer::selector);
        let setAuthorizerId: b256 = (~Address::from(vault)).getActionId(setAuthorizer::selector);

        let canSetAuthorizer:bool = oldAuthorizer::canPerform(setAuthorizerId, ~Address::from(this), ~Address::from(vault));
        require(canSetAuthorizer, "MIGRATOR_CANNOT_SET_AUTHORIZER");

        // Finally change the authorizer in the vault and trigger root change
        vault::setAuthorizer(newAuthorizer);
    }






}
