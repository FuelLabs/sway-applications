library interface;

dep data_structures;

use data_structures::{Arbiter, Asset};
use std::{identity::Identity, vec::Vec};

abi Escrow {
    /// Allows the buyer and seller to propose a new arbiter and/or change the arbiter fee
    ///
    /// If a dispute has been initiated and the arbiter is taking too long then the users can change
    /// the arbiter and the fee percentage for the new arbiter
    /// Users can also set the same arbiter but with a lower fee
    /// The arbiter in the escrow will only be changed if both users set the same address and fee
    ///
    /// # Arguments
    ///
    /// * `arbiter` - A third party which decides how a dispute is resolved
    /// * `identifier` - Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the arbiter fee is greater than 100%
    /// * When the caller is not the buyer or the seller
    /// * When the caller is setting the buyer or seller as the new arbiter
    #[storage(read, write)]fn change_arbiter(arbiter: Arbiter, identifier: u64);

    /// Creates an internal representation of an escrow instead of deploying a contract per escrow
    ///
    /// The escrow allows the buyer to deposit any asset from the specified assets
    ///
    /// # Arguments
    ///
    /// * `assets` - The assets, with the required deposit amounts, that the campaign accepts
    /// * `arbiter` - A third party which decides how a dispute is resolved
    /// * `buyer` - User who deposits funds into the escrow
    /// * `deadline` - TODO
    ///
    /// # Reverts
    ///
    /// * When the user does not specify any assets
    /// * When the deadline is not in the future
    /// * When the arbiter fee pecentage is greater than 100%
    /// * When the user is setting the buyer or themselves as the new arbiter
    /// * When the amount of any asset required for deposit is set to 0
    #[storage(read, write)] fn create_escrow(assets: Vec<Asset>, arbiter: Arbiter, buyer: Identity, deadline: u64);

    /// Accepts a deposit from the buyer for any of the assets specified in the escrow
    ///
    /// A successful deposit unlocks functionality for the rest of the escrow
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the deposit is made after the deadline
    /// * When the escrow is not in the State::Pending state
    /// * When the user is not the buyer
    /// * When the user deposits and they still have their previous deposit in the escrow
    /// * When the user sends an incorrect amount of an asset for the specified asset in the escrow
    /// * When the user deposits an asset that has not been specified in the escrow
    #[storage(read, write)] fn deposit(identifier: u64);

    /// Changes a flag in the escrow marking it as disputed which results in the escrow being locked
    ///
    /// Once the escrow is locked the seller cannot take the payment after those conditions are met
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the escrow is already in a dispute
    /// * When the user is not the buyer
    /// * When the user does not currently have a deposit in the escrow
    #[storage(read, write)]fn dispute(identifier: u64);

    // if a dispute has been filed and the escrow expires then the arbiter can choose who the funds are sent to
    #[storage(read, write)]fn resolve_dispute(identifier: u64, user: Identity);

    /// The seller transfers the funds from the escrow back to the buyer
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the user is not the seller
    /// * When the buyer does not currently have a deposit in the escrow
    #[storage(read, write)]fn return_deposit(identifier: u64);

    /// If a user has deposited but not transferred in time & they have not disputed then the seller
    /// can take the payment themselves
    //
    /// # Arguments
    ///
    /// * `identifier` - Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When a user attempts to take payment before the deadline
    /// * When a user attempts to take payment during a dispute
    /// * When the user is not the seller
    /// * When the buyer does not currently have a deposit in the escrow
    #[storage(read, write)]fn take_payment(identifier: u64);

    /// After a buyer deposits they can transfer the deposit to the seller
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the user is not the buyer
    #[storage(read, write)]fn transfer_to_seller(identifier: u64);
}
