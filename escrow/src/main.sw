contract;

// TODO:
//      * enum Eq not implemented for self therefore using u64 for now
//      * change arrays to vec and update threshold in creation
//      * currently the UserEscrows does not handle the completed => active logic and you can only
//        have 1 active escrow at a time (for display purposes)

// Our library dependencies
dep abi;
dep data_structures;
dep errors;
dep events;

// Standard library code
use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    identity::*,
    logging::log,
    option::*,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::transfer,
    vec::Vec,
};

// Bring our code into scope
use abi::Escrow;
// use data_structures::{Asset, EscrowData, State, User, UserEscrows};
use data_structures::*; // workaround to import Eq for State
use errors::{AccessError, ApproveError, CreationError, DepositError, InitError, StateError};
use events::{ApproveEvent, CreatedEscrowEvent, DepositEvent, ThresholdReachedEvent, WithdrawEvent};

// Note: mappings inside structs are not a thing therefore this mess exists
storage {
    /// State associated with the activity of a user for a specified EscrowData ID
    /// Map((EscrowData ID, Identity address) => user data for EscrowData)
    authorized_users: StorageMap<(u64, Identity), User>, 
    
    /// O(1) look-up used to check the amount required to deposit for a specific asset for an escrow
    /// This saves us from iterating over the vec
    /// Map((escrows ID, EscrowData AssetId) => deposit amount)
    deposit_amount: StorageMap<(u64, ContractId), u64>, 
    
    /// Metadata related to a newly created escrow via create_escrow()
    /// Map(ID => Data)
    escrows: StorageMap<u64, EscrowData>, 
    
    /// Number of created escrows
    /// Used for O(1) look up in mappings
    escrow_count: u64,

    /// Completed and currently active escrows for a specified user
    user_escrows: StorageMap<Identity, UserEscrows>, 
}

impl Escrow for Contract {
    /// Creates an internal representation of an escrow by setting the users and assets
    ///
    /// Instead of having a contract per escrow we create an internal representation for the data
    ///
    /// # Arguments
    ///
    /// - `assets` - The assets, with the required deposit amounts, that the campaign accepts
    /// - `users` - The users who are able to interact with the escrow
    ///
    /// # Reverts
    ///
    /// - When the amount of any asset required for deposit is set to 0
    #[storage(read, write)]
    fn create_escrow(assets: Vec<Asset>, users: Vec<Identity>) {
        storage.escrow_count += 1;

        // Set the assets that this escrow accepts
        let mut asset_index = 0;
        while asset_index < assets.len() {
            let asset: Option<Asset> = assets.get(asset_index);
            let asset = asset.unwrap();
            require(0 < asset.amount, CreationError::DepositAmountCannotBeZero);

            storage.deposit_amount.insert((storage.escrow_count, asset.id), asset.amount);
            asset_index += 1;
        }

        // Set the users that can interact with the escrow
        let mut user_index = 0;
        while user_index < users.len() {
            let user: Option<Identity> = users.get(user_index);
            let user = user.unwrap();
            storage.authorized_users.insert((storage.escrow_count, user), User {
                approved: false, asset: Option::None::<ContractId>(), exists: true, deposited: false
            });

            // Add a newly active escrow to the active list for the user
            let mut user_escrows = storage.user_escrows.get(user);

            // TODO: use mappings
            // user_escrows.active = [storage.escrow_count];

            storage.user_escrows.insert(user, user_escrows);
            user_index += 1;
        }

        let escrow = EscrowData {
            approval_count: 0,
            assets, state: State::Pending,
            threshold: users.len(),
            users, 
        };

        storage.escrows.insert(storage.escrow_count, escrow);

        log(CreatedEscrowEvent {
            escrow, identifier: storage.escrow_count
        });
    }

    /// Accepts a deposit from an authorized user for any of the specified assets
    /// A successful deposit unlocks the approval functionality for that user
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - The user is not an authorized user
    /// - The escrow is not in the State::Pending state
    /// - The user deposits when they still have their previous deposit in the escrow
    /// - The user deposits an asset that has not been specified in the constructor
    /// - The user sends an incorrect amount of an asset for the specified asset in the escrow
    #[storage(read, write)]
    fn deposit(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // Retrieve user data under escrow ID
        let identity = sender_identity();
        let mut user = storage.authorized_users.get((identifier, identity));

        // Make sure caller has been specified as a user for this escrow
        require(user.exists, AccessError::UnauthorizedUser);

        // User can only deposit when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Prevent the user from depositing if they still have a deposit in the escrow
        // This means they can only deposit 1 asset at a time and we do not have to juggle their
        // deposits in order to refund them correctly
        require(!user.deposited, DepositError::AlreadyDeposited);

        // Check how much of the deposited asset is required
        let deposited_asset = msg_asset_id();
        let required_amount = storage.deposit_amount.get((identifier, deposited_asset));

        // If the amount is 0 then this asset has not been specified at creation of the escrow
        require(required_amount != 0, DepositError::IncorrectAssetDeposited);

        // Check that the amount deposited is the amount specified at creation of the escrow
        // Exact amount reduces bytecode since we are not juggling calculations to ensure the correct
        // total is deposited
        require(required_amount == msg_amount(), DepositError::IncorrectAssetAmount);

        // Update user state to indicate that they have deposited one of the assets
        user.asset = Option::Some(deposited_asset);
        user.deposited = true;

        // Overwrite the previous state in storage with the latest changes
        storage.authorized_users.insert((identifier, identity), user);

        log(DepositEvent {
            amount: required_amount, asset: deposited_asset, identifier, user: identity
        });
    }

    /// Updates the user state to indicate a user has approved
    /// Once all of the users approve the escrow will lock the approve() & deposit() functions
    /// leaving withdrawal as the last function unlocked
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - The escrow is not in the State::Pending state
    /// - The user has not successfully deposited through the deposit() function
    /// - The user approves again after they have already approved
    #[storage(read, write)]
    fn approve(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only approve when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Make sure caller has been specified as a user for this escrow
        let identity = sender_identity();
        let mut user = storage.authorized_users.get((identifier, identity));

        // User must deposit before they can approve
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // To prevent approval count manipulation and (some) event spam ensure they haven't approved
        require(!user.approved, ApproveError::AlreadyApproved);

        user.approved = true;

        storage.authorized_users.insert((identifier, identity), user);
        escrow.approval_count = escrow.approval_count + 1;

        log(ApproveEvent {
            approval_count: escrow.approval_count, identifier, user: identity
        });

        if escrow.threshold <= escrow.approval_count {
            // Everyone has approved so lock the escrow down from further use
            // Locks: deposit() & approve()
            escrow.state = State::Completed;

            log(ThresholdReachedEvent {
                identifier
            });
        }

        storage.escrows.insert(identifier, escrow);
    }

    /// Returns the deposited asset back to the user and resets their deposit & approval flags to false
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - The specified escrow identifier is not in the valid range of existing escrows
    /// - The user has not successfully deposited through the deposit() function
    #[storage(read, write)]
    fn withdraw(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // Retrieve data from whoever is trying to get access for the specified identifier
        let identity = sender_identity();
        let mut user = storage.authorized_users.get((identifier, identity));

        // User can only withdraw their deposit if they have a deposit currently in the escrow
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // Safe to unwrap since the asset is set during the deposit
        // let deposited_asset = user.asset.unwrap();

        let deposited_asset = match user.asset {
            Option::Some(asset) => asset, _ => revert(0), // temporary workaround for unimplemented feature / bug
        };

        // Retrieve the amount the specified escrow set for the asset
        // No need to validate asset since user can only deposit a specified asset
        let amount = storage.deposit_amount.get((identifier, deposited_asset));

        // They're about to withdraw so reset their currently deposited asset back to None
        user.asset = Option::None::<ContractId>();
        user.deposited = false;
        user.approved = false;

        escrow.approval_count = escrow.approval_count - 1;

        storage.authorized_users.insert((identifier, identity), user);
        storage.escrows.insert(identifier, escrow);

        // Transfer the asset back to the user
        transfer(amount, deposited_asset, identity);

        log(WithdrawEvent {
            amount, approval_count: escrow.approval_count, asset: deposited_asset, identifier, user: identity, 
        });
    }

    /// Returns data regarding the state of a user i.e. whether they have deposited, approved, their
    /// chosen asset and whether they are a valid user
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    #[storage(read)]
    fn user_data(identifier: u64, user: Identity) -> User {
        storage.authorized_users.get((identifier, user))
    }

    /// Returns data regarding the identifiers for active and completed escrows
    /// Works for all users - including ones not in the contract
    ///
    /// # Arguments
    ///
    /// - `user` -
    #[storage(read)]
    fn user_escrows(user: Identity) -> UserEscrows {
        storage.user_escrows.get(user)
    }

    /// Returns the meta data regarding a created escrow
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    #[storage(read)]fn escrow_data(identifier: u64) -> EscrowData {
        storage.escrows.get(identifier)
    }
}

fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}
