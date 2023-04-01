library interface;

dep data_structures;

use data_structures::{Arbiter, Asset, EscrowInfo};

abi Escrow {
    /// Buyer accepts proposal to change arbiter details
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the caller is not the buyer
    /// * When the arbiter has not been proposed by the seller
    #[storage(read, write)]
    fn accept_arbiter(identifier: u64);

    /// Creates an internal representation of an escrow instead of deploying a contract per escrow
    ///
    /// The escrow allows the buyer to deposit any asset from the specified assets
    ///
    /// # Arguments
    ///
    /// * `arbiter`: A third party which decides how a dispute is resolved
    /// * `assets`: The assets, with the required deposit amounts, that the campaign accepts
    /// * `buyer`: User who deposits funds into the escrow
    /// * `deadline`: End height after which the buyer can no longer deposit and the seller can take payment
    ///
    /// # Reverts
    ///
    /// * When the caller does not specify any assets
    /// * When the deadline is not in the future
    /// * When the arbiter fee is set to 0
    /// * When the caller does not deposit the amount specified for the arbiter fee
    /// * When the caller does not deposit the specified asset for the arbiter fee
    /// * When the caller is setting the buyer or themselves as the arbiter
    /// * When the amount of any asset required for deposit is set to 0
    #[payable, storage(read, write)]
    fn create_escrow(arbiter: Arbiter, assets: Vec<Asset>, buyer: Identity, deadline: u64);

    /// Accepts a deposit from the buyer for any of the assets specified in the escrow
    ///
    /// A successful deposit unlocks functionality for the rest of the escrow
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the deposit is made during / after the deadline
    /// * When the escrow is not in the State::Pending state
    /// * When the caller is not the buyer
    /// * When the caller deposits more than once
    /// * When the caller sends an incorrect amount of an asset for the specified asset in the escrow
    /// * When the caller deposits an asset that has not been specified in the escrow
    #[payable, storage(read, write)]
    fn deposit(identifier: u64);

    /// Changes a flag in the escrow marking it as disputed which results in the escrow being locked
    ///
    /// Once the escrow is locked the seller cannot take the payment given that the conditions for
    /// taking a payment have been otherwise met
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the escrow is already in a dispute
    /// * When the caller is not the buyer
    /// * When the caller does not currently have a deposit in the escrow
    #[storage(read, write)]
    fn dispute(identifier: u64);

    /// Allows the seller to propose a new arbiter and/or change the arbiter fee
    ///
    /// If a dispute has been initiated and the arbiter is taking too long then the seller can change
    /// the arbiter, the asset for payment and the fee amount
    /// Seller can also set the same arbiter but with a different fee
    ///
    /// # Arguments
    ///
    /// * `arbiter`: A third party which decides how a dispute is resolved
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the caller is not the seller
    /// * When the caller is setting the buyer or seller as the new arbiter
    /// * When the arbiter fee is set to 0
    /// * When the caller does not deposit the amount specified for the arbiter fee
    /// * When the caller does not deposit the specified asset for the arbiter fee
    #[payable, storage(read, write)]
    fn propose_arbiter(arbiter: Arbiter, identifier: u64);

    /// The arbiter decides who the deposit is sent to and how much of the designated payment they
    /// will take
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    /// * `payment_amount`: The amount the arbiter will take as a payment for their work
    /// * `user`: The user who the deposit from the buyer will be sent to (either buyer or seller)
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the escrow is not in dispute
    /// * When the caller is not the arbiter
    /// * When the `user` is not the buyer or seller
    /// * When the buyer does not currently have a deposit in the escrow
    /// * When the `payment_amount` is greater than the deposit by the seller
    #[storage(read, write)]
    fn resolve_dispute(identifier: u64, payment_amount: u64, user: Identity);

    /// The seller transfers the funds from the escrow to the buyer
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the caller is not the seller
    /// * When the buyer does not currently have a deposit in the escrow
    #[storage(read, write)]
    fn return_deposit(identifier: u64);

    /// If a user has deposited but not transferred in time & they have not disputed then the seller
    /// can take the payment themselves
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the caller attempts to take payment before / during the deadline
    /// * When the caller attempts to take payment during a dispute
    /// * When the caller is not the seller
    /// * When the buyer does not currently have a deposit in the escrow
    #[storage(read, write)]
    fn take_payment(identifier: u64);

    /// After a buyer deposits they can transfer the deposit to the seller
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the buyer does not currently have a deposit in the escrow
    /// * When the caller is not the buyer
    #[storage(read, write)]
    fn transfer_to_seller(identifier: u64);

    /// If a buyer has not deposited and the deadline has been surpassed then the seller can withdraw
    /// their collateral
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    ///
    /// # Reverts
    ///
    /// * When the escrow is not in the State::Pending state
    /// * When the caller attempts to withdraw before / during the deadline
    /// * When the caller attempts to withdraw during a dispute
    /// * When the caller is not the seller
    /// * When the buyer deposited
    #[storage(read, write)]
    fn withdraw_collateral(identifier: u64);
}

abi Info {
    /// Returns the proposed arbiter for resolution
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    #[storage(read)]
    fn arbiter_proposal(identifier: u64) -> Option<Arbiter>;

    /// Returns the information about the asset used in an escrow
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    #[storage(read)]
    fn assets(identifier: u64) -> Option<Asset>;

    /// Returns information about an escrow
    ///
    /// # Arguments
    ///
    /// * `identifier`: Identifier used to find a specific escrow
    #[storage(read)]
    fn escrows(identifier: u64) -> Option<EscrowInfo>;

    /// Returns the total number of escrows created in the contract
    #[storage(read)]
    fn escrow_count() -> u64;
}
