library contract_abi;

dep data_structures;

use data_structures::{Arbiter, Asset};
use std::{identity::Identity, vec::Vec};

abi Escrow {
    #[storage(read, write)]fn change_arbiter(arbiter: Arbiter, identifier: u64);

    /// Creates an internal representation of an escrow instead of deploying a contract per escrow
    ///
    /// This sets the available addresses that can make a deposit (instead of forcing a single buyer),
    /// the assets which the escrow accepts and the seller who will receive the funds
    ///
    /// # Arguments
    ///
    /// - `assets` - The assets, with the required deposit amounts, that the campaign accepts
    /// - `buyers` - A whitelist of addresses one of which much deposit funds into the escrow
    /// - `seller` - The seller of the product to whom the funds will be transferred to
    ///
    /// # Reverts
    ///
    /// - When the amount of any asset required for deposit is set to 0
    #[storage(read, write)] fn create_escrow(assets: Vec<Asset>, arbiter: Identity, arbiter_fee_percentage: u64, buyer: Identity, deadline: u64);

    /// Accepts a deposit from am authorized user for any of the assets specified in the escrow
    /// A successful deposit unlocks the approval functionality for that user
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the escrow is not in the State::Pending state
    /// - When the user is not an authorized user (buyer)
    /// - When the user deposits and they still have their previous deposit in the escrow
    /// - When the user sends an incorrect amount of an asset for the specified asset in the escrow
    /// - When the user deposits an asset that has not been specified in the escrow
    #[storage(read, write)] fn deposit(identifier: u64);

    #[storage(read, write)]fn dispute(identifier: u64);

    // if a dispute has been filed and the escrow expires then the arbiter can choose who the funds are sent to
    #[storage(read, write)]fn resolve_dispute(identifier: u64, user: Identity);

    #[storage(read, write)]fn return_deposit(identifier: u64);

    /// If a user has deposited but not transferred in time & they have not disputed then the seller
    /// can take payment themselves
    //
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the escrow is not in the State::Pending state
    /// - When a user attempts to take payment before the deadline
    /// - When a user attempts to take payment during a dispute
    /// - When the user is not an authorized user (seller)
    #[storage(read, write)]fn take_payment(identifier: u64);

    /// After a buyer deposits they can transfer the deposit to the seller
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the escrow is not in the State::Pending state
    /// - When the user is not an authorized user (buyer)
    #[storage(read, write)]fn transfer_to_seller(identifier: u64);
}
