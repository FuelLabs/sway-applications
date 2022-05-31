contract;

use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, Sender, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    result::*,
    revert::revert,
    token::transfer_to_output,
};

abi Escrow {
    fn constructor(buyer: Address, seller: Address, buyer_asset: Asset, seller_asset: Asset) -> bool;
    fn deposit() -> bool;
    fn approve() -> bool;
    fn withdraw() -> bool;
    fn get_balance() -> (Asset, Asset);
    fn get_user_data(user: Address) -> (bool, bool);
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

// TODO: add enums back in when they are supported in storage and "matching" them is implemented
// enum State {
//     Void: (),
//     Pending: (),
//     Completed: (),
// }

struct Asset {
    amount: u64,
    id: ContractId,
}

struct User {
    address: Address,
    asset: Asset,
    approved: bool,
    deposited: bool,
}

storage {
    buyer: User,
    seller: User,
    // state: State,
    state: u64,
}

impl Escrow for Contract {
    /// Initializes the escrow with the users, the asset and amount of asset
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor is called more than once
    fn constructor(buyer: Address, seller: Address, buyer_asset: Asset, seller_asset: Asset) -> bool {
        // require(storage.state == State::Void, Error::CannotReinitialize);
        require(storage.state == 0, Error::CannotReinitialize);

        storage.buyer = User {
            address: buyer, asset: buyer_asset, approved: false, deposited: false
        };
        storage.seller = User {
            address: seller, asset: seller_asset, approved: false, deposited: false
        };
        storage.state = 1;
        // storage.state = State::Pending;

        true
    }

    /// Updates the user state to indicate that they have deposited
    /// A successful deposit unlocks the approval functionality
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user that has been set in the constructor
    /// - The user deposits an asset that is not the specified asset in the constructor
    /// - The user sends an incorrect amount of the asset that has been specified in the constructor
    /// - The user deposits when they still have their previous deposit in the escrow
    fn deposit() -> bool {
        // require(storage.state == State::Pending, Error::StateNotPending);
        require(storage.state == 1, Error::StateNotPending);

        let sender: Result<Sender, AuthError> = msg_sender();

        match sender.unwrap() {
            Sender::Address(address) => {
                require(address == storage.buyer.address || address == storage.seller.address, Error::UnauthorizedUser);

                if address == storage.buyer.address {
                    require(!storage.buyer.deposited, Error::UserHasAlreadyDeposited);
                    require(storage.buyer.asset.id == msg_asset_id(), Error::IncorrectAssetId);
                    require(storage.buyer.asset.amount == msg_amount(), Error::IncorrectAssetAmount);

                    storage.buyer.deposited = true;
                } else {
                    require(!storage.seller.deposited, Error::UserHasAlreadyDeposited);
                    require(storage.seller.asset.id == msg_asset_id(), Error::IncorrectAssetId);
                    require(storage.seller.asset.amount == msg_amount(), Error::IncorrectAssetAmount);

                    storage.seller.deposited = true;
                }
            },
            _ => revert(0), 
        }

        true
    }

    /// Updates the user state to indicate that they have approved
    /// Once both of the users approve the escrow will automatically transfers the assets back to the users
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user that has been set in the constructor
    /// - The user has not successfully deposited through the deposit() function
    /// - The user approves again after both users have approved and the escrow has completed its process
    fn approve() -> bool {
        // require(storage.state == State::Pending, Error::StateNotPending);
        require(storage.state == 1, Error::StateNotPending);

        let sender: Result<Sender, AuthError> = msg_sender();

        match sender.unwrap() {
            Sender::Address(address) => {
                require(address == storage.buyer.address || address == storage.seller.address, Error::UnauthorizedUser);

                if address == storage.buyer.address {
                    require(storage.buyer.deposited, Error::DepositRequired);
                    storage.buyer.approved = true;
                } else {
                    require(storage.seller.deposited, Error::DepositRequired);
                    storage.seller.approved = true;
                }

                if storage.buyer.approved && storage.seller.approved {
                    // storage.state = State::Completed;
                    storage.state = 2;

                    transfer_to_output(storage.buyer.asset.amount, storage.buyer.asset.id, storage.buyer.address);
                    transfer_to_output(storage.seller.asset.amount, storage.seller.asset.id, storage.seller.address);
                }
            },
            _ => revert(0), 
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
        require(storage.state == 1, Error::StateNotPending);

        let sender: Result<Sender, AuthError> = msg_sender();

        match sender.unwrap() {
            Sender::Address(address) => {
                require(address == storage.buyer.address || address == storage.seller.address, Error::UnauthorizedUser);

                if address == storage.buyer.address {
                    require(storage.buyer.deposited, Error::DepositRequired);

                    storage.buyer.deposited = false;
                    storage.buyer.approved = false;

                    transfer_to_output(storage.buyer.asset.amount, storage.buyer.asset.id, storage.buyer.address);
                } else {
                    require(storage.seller.deposited, Error::DepositRequired);

                    storage.seller.deposited = false;
                    storage.seller.approved = false;

                    transfer_to_output(storage.seller.asset.amount, storage.seller.asset.id, storage.seller.address);
                }
            },
            _ => revert(0), 
        }

        true
    }

    /// Returns the amount of the specified asset in this contract
    fn get_balance() -> (u64, u64) {
        (this_balance(storage.buyer.asset.id), this_balance(storage.seller.asset.id))
    }

    /// Returns data regarding the state of a user i.e. whether they have (deposited, approved)
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user that has been set in the constructor
    fn get_user_data(user: Address) -> (bool, bool) {
        // require(storage.state != State::Void, Error::StateNotInitialized);
        require(storage.state != 0, Error::StateNotInitialized);
        require(user == storage.buyer.address || user == storage.seller.address, Error::UnauthorizedUser);

        if user == storage.buyer.address {
            (storage.buyer.deposited, storage.buyer.approved)
        } else {
            (storage.seller.deposited, storage.seller.approved)
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
