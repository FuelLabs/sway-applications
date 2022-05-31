contract;

use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, Sender, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    result::*,
    revert::revert,
    token::{force_transfer, transfer_to_output}
};

abi Escrow {
    fn constructor(user1: Sender, user2: Sender, user1_asset: Asset, user2_asset: Asset) -> bool;
    fn deposit() -> bool;
    fn approve() -> bool;
    fn withdraw() -> bool;
    fn get_balance() -> (Asset, Asset);
    fn get_user_data(user: Sender) -> (bool, bool);
    fn get_state() -> u64;
}

enum Error {
    CannotReinitialize: (),
    DepositRequired: (),
    IncorrectAssetAmount: (),
    IncorrectAssetId: (),
    StateNotInitialized: (),
    StateNotPending: (),
    UnauthorizedUser: (),
    UserHasAlreadyDeposited: (),
}

// TODO: add enums when they are supported in storage
// enum State {
//     Void: (),
//     Pending: (),
//     Completed: (),
// }

struct Asset {
    amount: u64,
    id: ContractId,
}

struct Identity {
    address: b256,
    approved: bool,
    asset: Asset,
    deposited: bool,
}

storage {
    user1: Identity,
    user2: Identity,
    // state: State,
    state: u64,
}

impl Escrow for Contract {
    /// Initializes the escrow with the users and their required asset and amount of asset
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor is called more than once
    fn constructor(user1: Sender, user2: Sender, user1_asset: Asset, user2_asset: Asset) -> bool {
        // require(storage.state == State::Void, Error::CannotReinitialize);
        require(storage.state == 0, Error::CannotReinitialize);

        storage.user1 = Identity {
            approved: false, asset: user1_asset, deposited: false, address: _get_address(user1)
        };
        storage.user2 = Identity {
            approved: false, asset: user2_asset, deposited: false, address: _get_address(user2)
        };
        storage.state = 1;
        // storage.state = State::Pending;

        true
    }

    /// Updates the user state to indicate that they have deposited
    /// A successful deposit unlocks the approval functionality for that user
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user that has been set in the constructor
    /// - The user deposits when they still have their previous deposit in the escrow
    /// - The user deposits an asset that is not the specified asset in the constructor
    /// - The user sends an incorrect amount of the asset that has been specified in the constructor
    fn deposit() -> bool {
        // require(storage.state == State::Pending, Error::StateNotPending);
        require(storage.state == 1, Error::StateNotPending);

        let sender: Result<Sender, AuthError> = msg_sender();
        let address = _get_address(sender.unwrap());

        _is_authorized(address);

        if address == storage.user1.address {
            require(!storage.user1.deposited, Error::UserHasAlreadyDeposited);
            require(storage.user1.asset.id == msg_asset_id(), Error::IncorrectAssetId);
            require(storage.user1.asset.amount == msg_amount(), Error::IncorrectAssetAmount);

            storage.user1.deposited = true;
        } else {
            require(!storage.user2.deposited, Error::UserHasAlreadyDeposited);
            require(storage.user2.asset.id == msg_asset_id(), Error::IncorrectAssetId);
            require(storage.user2.asset.amount == msg_amount(), Error::IncorrectAssetAmount);

            storage.user2.deposited = true;
        }

        true
    }

    /// Updates the user state to indicate that they have approved
    /// Once both of the users approve the escrow will lock the approval and deposit functions leaving
    /// withdrawal unlocked
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user that has been set in the constructor
    /// - The user has not successfully deposited through the deposit() function
    /// - The user approves again after both users have approved
    fn approve() -> bool {
        // require(storage.state == State::Pending, Error::StateNotPending);
        require(storage.state == 1, Error::StateNotPending);

        let sender: Result<Sender, AuthError> = msg_sender();
        let address = _get_address(sender.unwrap());

        _is_authorized(address);

        if address == storage.user1.address {
            require(storage.user1.deposited, Error::DepositRequired);
            storage.user1.approved = true;
        } else {
            require(storage.user2.deposited, Error::DepositRequired);
            storage.user2.approved = true;
        }

        if storage.user1.approved && storage.user2.approved {
            // storage.state = State::Completed;
            storage.state = 2;
        }

        true
    }

    /// Returns the deposited asset back to the user and resets their approval to false
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user that has been set in the constructor
    /// - The user has not successfully deposited through the deposit() function
    fn withdraw() -> bool {
        // require(storage.state == State::Pending, Error::StateNotPending);
        require(storage.state == 1 || storage.state == 2, Error::StateNotPending);

        let sender: Result<Sender, AuthError> = msg_sender();
        let address = _get_address(sender.unwrap());

        _is_authorized(address);

        if address == storage.user1.address {
            require(storage.user1.deposited, Error::DepositRequired);

            storage.user1.deposited = false;
            storage.user1.approved = false;
        } else {
            require(storage.user2.deposited, Error::DepositRequired);

            storage.user2.deposited = false;
            storage.user2.approved = false;
        }

        match sender.unwrap() {
            Sender::Address(address) => {
                if address.value == storage.user1.address {
                    transfer_to_output(storage.user1.asset.amount, storage.user1.asset.id, ~Address::from(storage.user1.address));
                } else {
                    transfer_to_output(storage.user2.asset.amount, storage.user2.asset.id, ~Address::from(storage.user2.address));
                }
            },
            Sender::ContractId(address) => {
                if address.value == storage.user1.address {
                    force_transfer(storage.user1.asset.amount, storage.user1.asset.id, ~ContractId::from(storage.user1.address));
                } else {
                    force_transfer(storage.user2.asset.amount, storage.user2.asset.id, ~ContractId::from(storage.user2.address));
                }
            }
        }

        true
    }

    /// Returns each asset specified in the constructor and the amount that has been deposited by anyone
    fn get_balance() -> (Asset, Asset) {
        (Asset {
            amount: this_balance(storage.user1.asset.id), id: storage.user1.asset.id
        },
        Asset {
            amount: this_balance(storage.user2.asset.id), id: storage.user2.asset.id
        }
        )
    }

    /// Returns data regarding the state of a user i.e. whether they have (deposited, approved)
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user that has been set in the constructor
    fn get_user_data(user: Sender) -> (bool, bool) {
        // require(storage.state != State::Void, Error::StateNotInitialized);
        require(storage.state != 0, Error::StateNotInitialized);

        let address = _get_address(user);
        _is_authorized(address);

        if storage.state == 2 {
            (true, true)
        } else {
            if address == storage.user1.address {
                (storage.user1.deposited, storage.user1.approved)
            } else {
                (storage.user2.deposited, storage.user2.approved)
            }
        }
    }

    /// Returns a value indicating the current state of the escrow
    ///
    /// # State
    ///
    /// 0 = The constructor has yet to be called to initialize the contract state
    /// 1 = The constructor has been called to initialize the contract and is pending the deposit & approval from both parties
    /// 2 = Both parties have deposited and approved and the escrow has completed its purpose
    fn get_state() -> u64 {
        storage.state
    }
}

fn _get_address(user: Sender) -> b256 {
    match user {
        Sender::Address(address) => address.value, Sender::ContractId(address) => address.value, 
    }
}

fn _is_authorized(address: b256) {
    require(address == storage.user1.address || address == storage.user2.address, Error::UnauthorizedUser);
}
